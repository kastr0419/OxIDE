// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

#![allow(dead_code)]

use crate::core::board::{BoardPreset, FlashToolKind};
use crate::core::event::{CoreEvent, FlashMsg};
use crate::core::simulator::VirtualBoardEvent;
use anyhow::Result;
use crossbeam_channel::Sender;
use std::io::BufRead;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub enum FlashMessage {
    Started,
    Progress(String),
    Finished { ok: bool, log: String },
}

/// GUIアプリ起動時にPATHにcargo/binがない場合でもツールを探す。
/// which::which が失敗した場合は ~/.cargo/bin を直接確認する。
fn find_cargo_tool(name: &str) -> Option<std::path::PathBuf> {
    if let Ok(path) = which::which(name) {
        return Some(path);
    }
    let exe_name = if cfg!(windows) {
        format!("{}.exe", name)
    } else {
        name.to_string()
    };
    dirs::home_dir()
        .map(|h| h.join(".cargo").join("bin").join(&exe_name))
        .filter(|p| p.exists())
}

/// 子プロセスに ~/.cargo/bin を PATH として渡す
fn ensure_cargo_bin_in_path(cmd: &mut Command) {
    let cargo_bin = dirs::home_dir()
        .map(|h| h.join(".cargo").join("bin"))
        .filter(|p| p.exists());
    if let Some(cargo_bin) = cargo_bin {
        let mut paths: Vec<_> = std::env::var_os("PATH")
            .as_deref()
            .map(std::env::split_paths)
            .into_iter()
            .flatten()
            .collect();
        if !paths.contains(&cargo_bin) {
            paths.insert(0, cargo_bin);
            if let Ok(path) = std::env::join_paths(paths) {
                cmd.env("PATH", path);
            }
        }
    }
}

/// arm-none-eabi-objcopy / rust-objcopy / llvm-objcopy を順に試してELF→HEX/BIN変換する
fn run_objcopy(format: &str, elf: &Path, out: &Path) -> std::result::Result<(), String> {
    let candidates = [
        "arm-none-eabi-objcopy".to_string(),
        "rust-objcopy".to_string(),
        format!(
            r"{}\lib\rustlib\x86_64-pc-windows-msvc\bin\llvm-objcopy",
            std::env::var("USERPROFILE")
                .map(|h| format!(r"{}\.rustup\toolchains\stable-x86_64-pc-windows-msvc", h))
                .unwrap_or_default()
        ),
    ];
    for tool in &candidates {
        if tool.is_empty() {
            continue;
        }
        let mut c = std::process::Command::new(tool);
        let result = crate::core::no_window(&mut c)
            .arg("-O")
            .arg(format)
            .arg(elf)
            .arg(out)
            .output();
        match result {
            Ok(o) if o.status.success() => return Ok(()),
            Ok(_) => continue,
            Err(_) => continue, // tool not found
        }
    }
    Err(format!(
        "objcopy が見つかりません。以下のいずれかをインストールしてください:\n\
         - cargo install cargo-binutils && rustup component add llvm-tools-preview\n\
         - arm-none-eabi-binutils (https://developer.arm.com/downloads)\n\
         試行したツール: {:?}",
        candidates
    ))
}

pub fn flash(preset: &BoardPreset, port: &str, elf: &Path, tx: Sender<FlashMessage>) -> Result<()> {
    tx.send(FlashMessage::Started).ok();
    let preset = preset.clone();
    let elf = elf.to_owned();
    let port = port.to_string();
    thread::spawn(move || {
        let mut cmd_opt: Option<Command> = None;
        let mut log = String::new();
        let mut ok_flag = false;
        match preset.flash_tool {
            FlashToolKind::SdCard => {
                // elf -> kernel.img, then copy to SD mount (port)
                let mut kernel_img = elf.clone();
                kernel_img.set_extension("img");
                let dest = std::path::Path::new(&port).join("kernel.img");
                match run_objcopy("binary", &elf, &kernel_img) {
                    Ok(()) => match std::fs::copy(&kernel_img, &dest) {
                        Ok(_) => {
                            log = format!("kernel.img copied to {}", dest.display());
                            ok_flag = true;
                        }
                        Err(e) => {
                            log = format!("copy failed: {}", e);
                        }
                    },
                    Err(e) => {
                        log = e;
                    }
                }
            }
            FlashToolKind::Avrdude => {
                // assume hex is elf with .hex
                let mut hex = elf.clone();
                hex.set_extension("hex");
                let mut cmd = Command::new("avrdude");
                crate::core::no_window(&mut cmd);
                let mcu = preset.avrdude_mcu.unwrap_or("m328p");
                cmd.args([
                    "-p",
                    mcu,
                    "-c",
                    "arduino",
                    "-P",
                    &port,
                    "-b",
                    "115200",
                    "-U",
                    &format!("flash:w:{}:i", hex.display()),
                ]);
                cmd_opt = Some(cmd);
            }
            FlashToolKind::Esptool => {
                let mut bin = elf.clone();
                bin.set_extension("bin");
                match run_objcopy("binary", &elf, &bin) {
                    Ok(()) => {
                        let chip_str = match preset.kind {
                            crate::core::board::BoardKind::Esp32 => "esp32",
                            crate::core::board::BoardKind::Esp32S2 => "esp32s2",
                            crate::core::board::BoardKind::Esp32S3 => "esp32s3",
                            crate::core::board::BoardKind::Esp32C3 => "esp32c3",
                            crate::core::board::BoardKind::Esp32C6 => "esp32c6",
                            crate::core::board::BoardKind::Esp32H2 => "esp32h2",
                            _ => "esp32",
                        };
                        let mut cmd = Command::new("esptool.py");
                        crate::core::no_window(&mut cmd);
                        cmd.arg("--chip")
                            .arg(chip_str)
                            .arg("--port")
                            .arg(&port)
                            .arg("write_flash")
                            .arg(format!("0x{:x}", preset.flash_offset))
                            .arg(&bin);
                        cmd_opt = Some(cmd);
                    }
                    Err(e) => {
                        log = e;
                    }
                }
            }
            FlashToolKind::DaplinkHex => {
                // ポートが COM* / 空 / 存在しないパスなら DAPLink ドライブを自動検出
                let drive = if !port.is_empty()
                    && !port.to_uppercase().starts_with("COM")
                    && std::path::Path::new(&port).exists()
                {
                    port.clone()
                } else {
                    // DETAILS.TXT がある最初のドライブを DAPLink と見なす
                    ('A'..='Z')
                        .map(|c| format!("{}:\\", c))
                        .find(|d| std::path::Path::new(d).join("DETAILS.TXT").exists())
                        .unwrap_or_else(|| port.clone())
                };

                // ELF→HEX変換後、DAPLink USBドライブにコピー
                let mut hex = elf.clone();
                hex.set_extension("hex");
                match run_objcopy("ihex", &elf, &hex) {
                    Ok(()) => {
                        let hex_name = hex.file_name().unwrap_or_default();
                        let dest = std::path::Path::new(&drive).join(hex_name);
                        match std::fs::copy(&hex, &dest) {
                            Ok(_) => {
                                log = format!("HEX を {} にコピーしました", dest.display());
                                ok_flag = true;
                            }
                            Err(e) => {
                                log = format!(
                                    "DAPLinkドライブへのコピー失敗: {}\n\
                                     micro:bitが接続されているか確認してください。\n\
                                     (試みたパス: {})",
                                    e, drive
                                );
                            }
                        }
                    }
                    Err(e) => {
                        log = e;
                    }
                }
            }
            FlashToolKind::ProbeRs => {
                let mut cmd = Command::new("probe-rs");
                crate::core::no_window(&mut cmd);
                cmd.arg("download")
                    .arg("--chip")
                    .arg(preset.probe_rs_chip)
                    .arg(&elf);
                cmd_opt = Some(cmd);
            }
            FlashToolKind::OpenOcd => {
                log = "OpenOCD flashing is not supported yet".to_string();
            }
            FlashToolKind::Picotool => {
                // prefer picotool; if not available, fallback to elf2uf2-rs -d
                log.push_str(&format!("ELF: {}\n", elf.display()));
                if let Some(picotool_path) = find_cargo_tool("picotool") {
                    log.push_str(&format!("Tool: {}\n", picotool_path.display()));
                    let mut cmd = Command::new(&picotool_path);
                    crate::core::no_window(&mut cmd);
                    ensure_cargo_bin_in_path(&mut cmd);
                    cmd.arg("load").arg("-f").arg("-x").arg(&elf);
                    cmd_opt = Some(cmd);
                } else if let Some(elf2uf2_path) = find_cargo_tool("elf2uf2-rs") {
                    log.push_str(&format!("Tool: {}\n", elf2uf2_path.display()));
                    let mut cmd = Command::new(&elf2uf2_path);
                    crate::core::no_window(&mut cmd);
                    ensure_cargo_bin_in_path(&mut cmd);
                    cmd.arg("-d").arg(&elf);
                    cmd_opt = Some(cmd);
                } else {
                    let cargo_bin = dirs::home_dir()
                        .map(|h| h.join(".cargo").join("bin").to_string_lossy().to_string())
                        .unwrap_or_else(|| "~/.cargo/bin".to_string());
                    log = format!(
                        "❌ フラッシュツールが見つかりません。\n\
                         確認したパス: {}\n\
                         以下を実行してインストールしてください:\n\
                         • cargo install elf2uf2-rs",
                        cargo_bin
                    );
                }
            }
            FlashToolKind::Bossac => {
                // ELF -> BIN, then run bossac
                let mut bin = elf.clone();
                bin.set_extension("bin");
                match run_objcopy("binary", &elf, &bin) {
                    Ok(()) => {
                        let mut cmd = Command::new("bossac");
                        crate::core::no_window(&mut cmd);
                        if !port.is_empty() {
                            cmd.arg("-p").arg(&port);
                        }
                        cmd.args(["-e", "-w", "-v", "-R"]);
                        cmd.arg(&bin);
                        cmd_opt = Some(cmd);
                    }
                    Err(e) => {
                        log = e;
                    }
                }
            }
            FlashToolKind::StFlash => {
                log = "st-flash flashing is not supported yet".to_string();
            }
            FlashToolKind::NrfJprog => {
                log = "nrfjprog flashing is not supported yet".to_string();
            }
            FlashToolKind::TeensyLoader => {
                // ELF -> IHEX, then use teensy_loader_cli
                let mut hex = elf.clone();
                hex.set_extension("hex");
                match run_objcopy("ihex", &elf, &hex) {
                    Ok(()) => {
                        let mut cmd = Command::new("teensy_loader_cli");
                        crate::core::no_window(&mut cmd);
                        // default to TEENSY40 for Teensy 4 boards
                        cmd.arg("--mcu=TEENSY40").arg("-w").arg(&hex);
                        cmd_opt = Some(cmd);
                    }
                    Err(e) => {
                        log = e;
                    }
                }
            }
        }
        let final_log = if let Some(cmd) = cmd_opt {
            let (success, output) =
                run_with_initial_timeout(cmd, tx.clone(), Duration::from_secs(60));
            ok_flag = success;
            if log.is_empty() {
                output
            } else {
                format!("{}{}", log, output)
            }
        } else {
            if log.is_empty() {
                "no flashing command".to_string()
            } else {
                log
            }
        };
        tx.send(FlashMessage::Finished {
            ok: ok_flag,
            log: final_log,
        })
        .ok();
    });
    Ok(())
}

/// フラッシュコマンドを実行する。
/// stdout/stderr を収集しつつ、最大60秒でプロセス終了を待つ。
/// elf2uf2-rs のように出力なしで終了するツールにも対応。
fn run_with_initial_timeout(
    mut cmd: Command,
    progress_tx: Sender<FlashMessage>,
    timeout: Duration,
) -> (bool, String) {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => return (false, format!("フラッシャー起動失敗: {}", e)),
    };

    let (tx, rx) = crossbeam_channel::bounded::<()>(64);
    let log = Arc::new(Mutex::new(String::new()));
    let mut handles = Vec::new();

    for stream in [
        child
            .stdout
            .take()
            .map(|s| Box::new(s) as Box<dyn std::io::Read + Send>),
        child
            .stderr
            .take()
            .map(|s| Box::new(s) as Box<dyn std::io::Read + Send>),
    ]
    .into_iter()
    .flatten()
    {
        let log = log.clone();
        let tx = tx.clone();
        let progress_tx = progress_tx.clone();
        handles.push(thread::spawn(move || {
            let reader = std::io::BufReader::new(stream);
            for line in reader.lines().map_while(Result::ok) {
                tx.send(()).ok();
                progress_tx.send(FlashMessage::Progress(line.clone())).ok();
                let mut l = log.lock().unwrap_or_else(|e| e.into_inner());
                l.push_str(&line);
                l.push('\n');
            }
        }));
    }
    drop(tx); // 全 sender を落とすと rx が閉じる

    // 出力の有無にかかわらず、最大60秒プロセス終了を待つ
    let deadline = std::time::Instant::now() + timeout;
    loop {
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            for h in handles {
                let _ = h.join();
            }
            return (
                false,
                "⏱ タイムアウト (60秒): デバイスが応答しませんでした。\n\
                 Pico は BOOTSEL モードで接続されていましたか？\n\
                 (BOOTSELボタンを押しながらUSB接続 → RPI-RP2ドライブが現れてから Flash)"
                    .to_string(),
            );
        }
        match rx.recv_timeout(Duration::from_millis(500)) {
            Ok(_) => { /* 出力あり — 続けて待つ */ }
            Err(crossbeam_channel::RecvTimeoutError::Disconnected) => {
                // パイプが閉じた = プロセスが終了（出力なしでも正常）
                break;
            }
            Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
        }
    }

    for h in handles {
        let _ = h.join();
    }
    let ok = child.wait().ok().map(|s| s.success()).unwrap_or(false);
    let mut result_log = log.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if result_log.is_empty() {
        result_log = if ok {
            "✅ フラッシュ成功".to_string()
        } else {
            "❌ フラッシュ失敗（ツールがエラーで終了しました）\n\
             Pico が BOOTSEL モードで接続されているか確認してください。\n\
             (BOOTSELボタンを押しながらUSB接続 → RPI-RP2ドライブが現れてから Flash)"
                .to_string()
        };
    }
    (ok, result_log)
}

pub struct FlashRequest {
    pub board: crate::core::board::BoardKind,
    pub artifact: std::path::PathBuf,
    pub port: String,
}

pub struct FlashResult {
    pub success: bool,
    pub output: String,
}

pub fn flash_async(req: FlashRequest, tx: crossbeam_channel::Sender<CoreEvent>) {
    // 既存の flash() 関数を呼び出し、結果を CoreEvent::FlashFinished に変換
    // board kind から preset を探してflashを呼ぶ
    use crate::core::board::BOARD_PRESETS;
    std::thread::spawn(move || {
        if req.port == crate::core::serial::VIRTUAL_PORT_NAME {
            tx.send(CoreEvent::Flash(FlashMsg::Started)).ok();
            tx.send(CoreEvent::VirtualBoard(VirtualBoardEvent::FlashStarted))
                .ok();
            if !req.artifact.is_file() {
                tx.send(CoreEvent::Flash(FlashMsg::Finished(FlashResult {
                    success: false,
                    output: format!("Artifact not found: {}", req.artifact.display()),
                })))
                .ok();
                tx.send(CoreEvent::VirtualBoard(VirtualBoardEvent::FlashFinished(
                    false,
                )))
                .ok();
                return;
            }
            tx.send(CoreEvent::Flash(FlashMsg::Progress(
                "Flashing virtual board".into(),
            )))
            .ok();
            tx.send(CoreEvent::Flash(FlashMsg::Finished(FlashResult {
                success: true,
                output: "Virtual flash completed".into(),
            })))
            .ok();
            tx.send(CoreEvent::VirtualBoard(VirtualBoardEvent::FlashFinished(
                true,
            )))
            .ok();
            return;
        }
        let preset_opt = BOARD_PRESETS
            .iter()
            .find(|p| std::mem::discriminant(&p.kind) == std::mem::discriminant(&req.board));
        if let Some(preset) = preset_opt {
            let (ftx, frx) = crossbeam_channel::bounded(1);
            let _ = flash(preset, &req.port, &req.artifact, ftx);
            while let Ok(msg) = frx.recv() {
                match msg {
                    FlashMessage::Progress(line) => {
                        tx.send(CoreEvent::Flash(FlashMsg::Progress(line))).ok();
                    }
                    FlashMessage::Finished { ok, log } => {
                        tx.send(CoreEvent::Flash(FlashMsg::Finished(FlashResult {
                            success: ok,
                            output: log,
                        })))
                        .ok();
                        break;
                    }
                    FlashMessage::Started => {}
                }
            }
        } else {
            tx.send(CoreEvent::Flash(FlashMsg::Finished(FlashResult {
                success: false,
                output: "Board not found".to_string(),
            })))
            .ok();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streams_stdout_and_stderr() {
        let cmd = if cfg!(windows) {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", "echo stdout & echo stderr 1>&2"]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", "echo stdout; echo stderr >&2"]);
            cmd
        };
        let (tx, rx) = crossbeam_channel::unbounded();

        let (ok, log) = run_with_initial_timeout(cmd, tx, Duration::from_secs(60));
        let progress: Vec<_> = rx
            .try_iter()
            .filter_map(|message| match message {
                FlashMessage::Progress(line) => Some(line),
                _ => None,
            })
            .collect();

        assert!(ok);
        assert!(log.contains("stdout") && log.contains("stderr"));
        assert!(progress.iter().any(|line| line.trim() == "stdout"));
        assert!(progress.iter().any(|line| line.trim() == "stderr"));
    }

    #[test]
    fn times_out_even_while_output_is_streaming() {
        let mut cmd = if cfg!(windows) {
            let mut cmd = Command::new("powershell");
            cmd.args([
                "-NoProfile",
                "-Command",
                "while ($true) { Write-Output tick; Start-Sleep -Milliseconds 10 }",
            ]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", "while true; do echo tick; sleep 0.01; done"]);
            cmd
        };
        crate::core::no_window(&mut cmd);
        let (tx, _) = crossbeam_channel::unbounded();

        let (ok, log) = run_with_initial_timeout(cmd, tx, Duration::from_millis(100));

        assert!(!ok);
        assert!(log.contains("タイムアウト"));
    }
}
