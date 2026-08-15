// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use super::IdeApp;
use std::path::{Path, PathBuf};

pub(crate) fn write_or_log(path: &Path, content: &str, log: &mut String) {
    if let Err(e) = std::fs::write(path, content) {
        *log = format!("[ERROR] ファイル保存失敗 {}: {}", path.display(), e);
    }
}

pub(super) fn reload_clean_tabs(tabs: &mut Vec<FileTab>) {
    tabs.retain(|tab| tab.is_dirty || tab.path.is_file());
    for tab in tabs.iter_mut().filter(|tab| !tab.is_dirty) {
        if let Ok(content) = std::fs::read_to_string(&tab.path) {
            tab.content = content;
        }
    }
}

/// 開いているファイルタブ1つ分の状態
#[derive(Clone)]
pub struct FileTab {
    pub path: PathBuf,
    pub content: String,
    pub is_dirty: bool,
}

impl IdeApp {
    /// 現在のタブ内容を open_tabs に保存する
    pub(crate) fn sync_active_tab(&mut self) {
        if let Some(tab) = self.open_tabs.get_mut(self.active_tab) {
            tab.content = self.editor_text.clone();
            tab.is_dirty = self.is_dirty;
        }
    }

    /// ファイルを新しいタブで開く（既に開いていればそのタブに切り替え）
    pub(crate) fn open_file_in_tab(&mut self, path: PathBuf) {
        // 既に開いていたらそのタブに切り替え
        if let Some(idx) = self.open_tabs.iter().position(|t| t.path == path) {
            self.switch_to_tab(idx);
            return;
        }
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        self.sync_active_tab();
        self.open_tabs.push(FileTab {
            path: path.clone(),
            content: content.clone(),
            is_dirty: false,
        });
        self.active_tab = self.open_tabs.len() - 1;
        // LSP に did_open を送る（初期化完了後でないと rust-analyzer が無視する）
        {
            let uri = crate::core::lsp::file_uri(&path);
            if self.lsp_initialized {
                if let Some(ref lsp) = self.lsp_client {
                    lsp.did_open(&uri, &content);
                }
            } else if self.lsp_client.is_some() {
                // 初期化待ちのためバッファに追加
                self.pending_did_opens.push((uri, content.clone()));
            }
        }
        self.editor_text = content;
        self.file_path = Some(path);
        self.is_dirty = false;
    }

    /// タブを切り替える
    pub(crate) fn switch_to_tab(&mut self, idx: usize) {
        if idx == self.active_tab && !self.open_tabs.is_empty() {
            return;
        }
        self.sync_active_tab();
        self.active_tab = idx;
        if let Some(tab) = self.open_tabs.get(idx) {
            self.editor_text = tab.content.clone();
            self.file_path = Some(tab.path.clone());
            self.is_dirty = tab.is_dirty;
        }
    }

    /// タブを閉じる（dirty なら先に保存）
    pub(crate) fn close_tab(&mut self, idx: usize) {
        if idx >= self.open_tabs.len() {
            return;
        }
        // 閉じる前に保存
        let tab = &self.open_tabs[idx];
        if tab.is_dirty {
            write_or_log(&tab.path, &tab.content, &mut self.build_log);
        }
        self.open_tabs.remove(idx);
        if self.open_tabs.is_empty() {
            self.editor_text = String::new();
            self.file_path = None;
            self.is_dirty = false;
            self.active_tab = 0;
        } else {
            self.active_tab = self.active_tab.min(self.open_tabs.len().saturating_sub(1));
            if let Some(tab) = self.open_tabs.get(self.active_tab).cloned() {
                self.editor_text = tab.content;
                self.file_path = Some(tab.path);
                self.is_dirty = tab.is_dirty;
            } else {
                self.editor_text = String::new();
                self.file_path = None;
                self.is_dirty = false;
            }
        }
    }

    /// ワークスペースのファイル一覧を更新する
    pub(crate) fn refresh_workspace_files(&mut self) {
        self.workspace_files = scan_workspace_files(&self.config.workspace_dir);
    }
}

/// ワークスペース内の編集対象ファイルを収集する
fn scan_workspace_files(workspace: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if !workspace.exists() {
        return files;
    }
    // src/ 以下を再帰収集
    let src_dir = workspace.join("src");
    if let Ok(entries) = std::fs::read_dir(&src_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                files.push(p);
            }
        }
    }
    // .cargo/ ディレクトリ
    let cargo_dir = workspace.join(".cargo");
    if let Ok(entries) = std::fs::read_dir(&cargo_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                files.push(p);
            }
        }
    }
    // ルートファイル
    for name in &["Cargo.toml", "memory.x", "build.rs", "rust-toolchain.toml"] {
        let p = workspace.join(name);
        if p.exists() {
            files.push(p);
        }
    }
    files.sort();
    files
}
