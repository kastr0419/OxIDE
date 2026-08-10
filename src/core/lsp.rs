// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use crossbeam_channel::{Receiver, Sender};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicI64, Ordering};
use std::thread;

static REQUEST_ID: AtomicI64 = AtomicI64::new(1);

fn next_id() -> i64 {
    REQUEST_ID.fetch_add(1, Ordering::SeqCst)
}

/// LSP サーバー（rust-analyzer）との通信セッション
pub struct LspClient {
    pub stdin_tx: Sender<String>,
    #[allow(dead_code)]
    pub response_rx: Receiver<LspMessage>, // レスポンス受信用（将来の双方向通信で使用）
    pub ui_tx: Sender<LspMessage>,
    _child: Child,
}

impl Drop for LspClient {
    fn drop(&mut self) {
        // rust-analyzer プロセスを確実に終了させる
        let _ = self._child.kill();
    }
}

/// UI に届くメッセージ
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum LspMessage {
    Initialized,
    CompletionItems(Vec<CompletionItem>),
    Diagnostics(Vec<Diagnostic>),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>, // 型情報
    #[allow(dead_code)]
    pub documentation: Option<String>, // ドキュメント（将来のホバー表示で使用）
    pub insert_text: Option<String>, // 挿入テキスト（スニペット）
    pub kind: CompletionKind,
}

#[derive(Debug, Clone)]
pub enum CompletionKind {
    Function,
    Method,
    Struct,
    Enum,
    Constant,
    Module,
    Keyword,
    Snippet,
    Other,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub line: u32,
    pub col: u32,
    pub message: String,
    pub severity: DiagSeverity,
}

#[derive(Debug, Clone)]
pub enum DiagSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// LSP の `Content-Length: N\r\n\r\nBODY` フレームとして書き込む
fn write_message(stdin: &mut ChildStdin, msg: &Value) -> std::io::Result<()> {
    let body = serde_json::to_string(msg).map_err(|e| std::io::Error::other(e.to_string()))?;
    let frame = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
    stdin.write_all(frame.as_bytes())?;
    stdin.flush()
}

/// rust-analyzer を起動して LspClient を返す
pub fn start_lsp(
    workspace: PathBuf,
    tx_to_ui: Sender<LspMessage>,
    ra_path_override: Option<PathBuf>,
) -> Option<LspClient> {
    let ra_path = ra_path_override
        .filter(|p| p.exists())
        .or_else(|| which::which("rust-analyzer").ok())?;

    let mut ra_cmd = Command::new(ra_path);
    let mut child = crate::core::no_window(&mut ra_cmd)
        .current_dir(&workspace)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    let mut child_stdin = child.stdin.take()?;
    let child_stdout = child.stdout.take()?;

    // ─── stdin 送信スレッド ───────────────────────────────
    let (stdin_tx, stdin_rx) = crossbeam_channel::unbounded::<String>();
    let ui_tx_for_stdin = tx_to_ui.clone();
    thread::spawn(move || {
        while let Ok(msg_str) = stdin_rx.recv() {
            match serde_json::from_str::<Value>(&msg_str) {
                Ok(v) => {
                    if let Err(e) = write_message(&mut child_stdin, &v) {
                        let _ = ui_tx_for_stdin
                            .send(LspMessage::Error(format!("LSP stdin write error: {}", e)));
                    }
                }
                Err(e) => {
                    let _ = ui_tx_for_stdin.send(LspMessage::Error(format!(
                        "Invalid JSON for LSP stdin: {}",
                        e
                    )));
                }
            }
        }
    });

    // ─── stdout 受信スレッド ──────────────────────────────
    let tx_clone = tx_to_ui.clone();
    let stdin_tx_clone = stdin_tx.clone();
    thread::spawn(move || {
        let mut reader = BufReader::new(child_stdout);
        loop {
            // Content-Length ヘッダを読む
            let mut header = String::new();
            if reader.read_line(&mut header).unwrap_or(0) == 0 {
                break;
            }
            let header = header.trim().to_string();
            if !header.starts_with("Content-Length:") {
                continue;
            }
            let content_len: usize = header
                .split(':')
                .nth(1)
                .unwrap_or("0")
                .trim()
                .parse()
                .unwrap_or(0);

            // 空行を読み飛ばす
            let mut blank = String::new();
            let _ = reader.read_line(&mut blank);

            // ボディ読み込み
            let mut body = vec![0u8; content_len];
            use std::io::Read;
            if reader.read_exact(&mut body).is_err() {
                break;
            }

            let Ok(json) = serde_json::from_slice::<Value>(&body) else {
                continue;
            };

            // initialize レスポンスを検出したら initialized 通知を送る
            if json.get("id").is_some()
                && json
                    .get("result")
                    .map(|r| r.get("capabilities").is_some())
                    .unwrap_or(false)
                && json.get("method").is_none()
            {
                let init_notif = json!({
                    "jsonrpc": "2.0",
                    "method": "initialized",
                    "params": {}
                });
                match serde_json::to_string(&init_notif) {
                    Ok(s) => {
                        let _ = stdin_tx_clone.send(s);
                    }
                    Err(e) => {
                        let _ =
                            tx_clone.send(LspMessage::Error(format!("LSP serialize error: {}", e)));
                    }
                }
                let _ = tx_clone.send(LspMessage::Initialized);
                continue;
            }

            // レスポンス種別に応じて LspMessage に変換
            if let Some(items) = parse_completion_response(&json) {
                let _ = tx_clone.send(LspMessage::CompletionItems(items));
            } else if let Some(diags) = parse_diagnostics(&json) {
                let _ = tx_clone.send(LspMessage::Diagnostics(diags));
            } else if json.get("method").and_then(|m| m.as_str()) == Some("window/logMessage") {
                // ログは無視
            }
        }
    });

    // ─── initialize リクエスト送信 ────────────────────────
    let init_msg = json!({
        "jsonrpc": "2.0",
        "id": next_id(),
        "method": "initialize",
        "params": {
            "processId": std::process::id(),
            "rootUri": format!("file:///{}", workspace.to_string_lossy().replace('\\', "/")),
            "capabilities": {
                "textDocument": {
                    "completion": {
                        "completionItem": {
                            "snippetSupport": true,
                            "documentationFormat": ["plaintext"]
                        }
                    },
                    "publishDiagnostics": {}
                }
            },
            "workspaceFolders": [{
                "uri": format!("file:///{}", workspace.to_string_lossy().replace('\\', "/")),
                "name": workspace.file_name()
                    .and_then(|n| n.to_str()).unwrap_or("workspace")
            }]
        }
    });
    if let Ok(s) = serde_json::to_string(&init_msg) {
        stdin_tx.send(s).ok()?;
    } else {
        return None;
    }

    Some(LspClient {
        stdin_tx,
        response_rx: crossbeam_channel::unbounded().1,
        ui_tx: tx_to_ui.clone(),
        _child: child,
    })
}

// ── ヘルパー: LspClient のメソッド ────────────────────────

impl LspClient {
    /// テキスト変更を LSP に通知（`textDocument/didOpen`）
    /// ファイルを開いた際に呼び出す（将来のLSP完全統合で使用）
    pub fn did_open(&self, uri: &str, text: &str) {
        let msg = json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": uri,
                    "languageId": "rust",
                    "version": 1,
                    "text": text
                }
            }
        });
        match serde_json::to_string(&msg) {
            Ok(s) => {
                let _ = self.stdin_tx.send(s);
            }
            Err(e) => {
                let _ = self
                    .ui_tx
                    .send(LspMessage::Error(format!("LSP serialize error: {}", e)));
            }
        }
    }

    pub fn did_change(&self, uri: &str, version: i32, text: &str) {
        let msg = json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didChange",
            "params": {
                "textDocument": { "uri": uri, "version": version },
                "contentChanges": [{ "text": text }]
            }
        });
        match serde_json::to_string(&msg) {
            Ok(s) => {
                let _ = self.stdin_tx.send(s);
            }
            Err(e) => {
                let _ = self
                    .ui_tx
                    .send(LspMessage::Error(format!("LSP serialize error: {}", e)));
            }
        }
    }

    /// カーソル位置の補完リクエスト（`textDocument/completion`）
    pub fn request_completion(&self, uri: &str, line: u32, character: u32) {
        let msg = json!({
            "jsonrpc": "2.0",
            "id": next_id(),
            "method": "textDocument/completion",
            "params": {
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": character }
            }
        });
        match serde_json::to_string(&msg) {
            Ok(s) => {
                let _ = self.stdin_tx.send(s);
            }
            Err(e) => {
                let _ = self
                    .ui_tx
                    .send(LspMessage::Error(format!("LSP serialize error: {}", e)));
            }
        }
    }
}

/// completion レスポンスをパース
fn parse_completion_response(json: &Value) -> Option<Vec<CompletionItem>> {
    let items_json = json
        .get("result")
        .and_then(|r| r.get("items").or(Some(r)))
        .and_then(|v| v.as_array())?;

    let items = items_json
        .iter()
        .map(|item| {
            let label = item["label"].as_str().unwrap_or("").to_string();
            let detail = item["detail"].as_str().map(|s| s.to_string());
            let documentation = item["documentation"]
                .as_str()
                .or_else(|| item["documentation"]["value"].as_str())
                .map(|s| s.to_string());
            let insert_text = item["insertText"]
                .as_str()
                .or_else(|| item["textEdit"]["newText"].as_str())
                .map(|s| s.to_string());
            let kind = match item["kind"].as_u64().unwrap_or(0) {
                3 => CompletionKind::Function,
                2 => CompletionKind::Method,
                7 => CompletionKind::Struct,
                13 => CompletionKind::Enum,
                21 => CompletionKind::Constant,
                9 => CompletionKind::Module,
                14 => CompletionKind::Keyword,
                15 => CompletionKind::Snippet,
                _ => CompletionKind::Other,
            };
            CompletionItem {
                label,
                detail,
                documentation,
                insert_text,
                kind,
            }
        })
        .collect();

    Some(items)
}

/// diagnostics をパース
fn parse_diagnostics(json: &Value) -> Option<Vec<Diagnostic>> {
    let method = json.get("method")?.as_str()?;
    if method != "textDocument/publishDiagnostics" {
        return None;
    }
    let diags_json = json["params"]["diagnostics"].as_array()?;

    let diags = diags_json
        .iter()
        .map(|d| {
            let line = d["range"]["start"]["line"].as_u64().unwrap_or(0) as u32;
            let col = d["range"]["start"]["character"].as_u64().unwrap_or(0) as u32;
            let message = d["message"].as_str().unwrap_or("").to_string();
            let severity = match d["severity"].as_u64().unwrap_or(1) {
                1 => DiagSeverity::Error,
                2 => DiagSeverity::Warning,
                3 => DiagSeverity::Info,
                _ => DiagSeverity::Hint,
            };
            Diagnostic {
                line,
                col,
                message,
                severity,
            }
        })
        .collect();

    Some(diags)
}
