// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

#![allow(dead_code)]

use crate::core::board::{BoardPreset, FlashToolKind};
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
    Finished { ok: bool, log: String },
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
        if tool.is_empty() { continue; }
        let mut c = std::process::Command::new(tool);
        let result = crate::core::no_window(&mut c)
            .arg("-O").arg(format)
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
                    Ok(()) => {
                        match std::fs::copy(&kernel_img, &dest) {
                            Ok(_) => {
                                log = format!("kernel.img copied to {}", dest.display());
                                ok_flag = true;
                            }
                            Err(e) => {
                                log = format!("copy failed: {}", e);
                            }
                        }
                    }
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
                cmd.args(["-p", mcu, "-c", "arduino", "-P", &port, "-b", "115200", "-U", &format!("flash:w:{}:i", hex.display())]);
                cmd_opt = Some(cmd);
            }
            FlashToolKind::Esptool => {
                let mut bin = elf.clone();
                bin.set_extension("bin");
                match run_objcopy("binary", &elf, &bin) {
                    Ok(()) => {
                        let chip_str = match preset.kind {
                            crate::core::board::BoardKind::Esp32   => "esp32",
                            crate::core::board::BoardKind::Esp32S2 => "esp32s2",
                            crate::core::board::BoardKind::Esp32S3 => "esp32s3",
                            crate::core::board::BoardKind::Esp32C3 => "esp32c3",
                            crate::core::board::BoardKind::Esp32C6 => "esp32c6",
                            crate::core::board::BoardKind::Esp32H2 => "esp32h2",
                            _ => "esp32",
                        };
                        let mut cmd = Command::new("esptool.py");
                        crate::core::no_window(&mut cmd);
                        cmd.arg("--chip").arg(chip_str)
                            .arg("--port").arg(&port)
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
                cmd.arg("download").arg("--chip").arg(preset.probe_rs_chip).arg(&elf);
                cmd_opt = Some(cmd);
            }
            FlashToolKind::OpenOcd => {
                // stub
            }
            FlashToolKind::Picotool => {
                // stub: picotool for RP2040 UF2 programming
            }
            FlashToolKind::Bossac => {
                // stub: bossac (SAM core USB bootloader)
            }
            FlashToolKind::StFlash => {
                // stub: st-flash for STM32 via ST-Link
            }
            FlashToolKind::NrfJprog => {
                // stub: nrfjprog for Nordic chips
            }
            FlashToolKind::TeensyLoader => {
                // stub: teensy_loader_cli for Teensy boards
            }
        }
        let final_log = if let Some(cmd) = cmd_opt {
            let (success, output) = run_with_initial_timeout(cmd);
            ok_flag = success;
            output
        } else {
            if log.is_empty() { "no flashing command".to_string() } else { log }
        };
        tx.send(FlashMessage::Finished { ok: ok_flag, log: final_log }).ok();
    });
    Ok(())
}

/// フラッシュコマンドを実行する。
/// 最初の3秒間に一切応答がなければタイムアウトエラーを返す。
/// 3秒以内に応答があった場合は、完了するまで待ち続ける。
fn run_with_initial_timeout(mut cmd: Command) -> (bool, String) {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => return (false, format!("フラッシャー起動失敗: {}", e)),
    };

    let (tx, rx) = crossbeam_channel::bounded::<()>(1);
    let log = Arc::new(Mutex::new(String::new()));
    let mut handles = Vec::new();

    for stream in [
        child.stdout.take().map(|s| Box::new(s) as Box<dyn std::io::Read + Send>),
        child.stderr.take().map(|s| Box::new(s) as Box<dyn std::io::Read + Send>),
    ]
    .into_iter()
    .flatten()
    {
        let log = log.clone();
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            let reader = std::io::BufReader::new(stream);
            for line in reader.lines().map_while(Result::ok) {
                tx.send(()).ok();
                let mut l = log.lock().unwrap_or_else(|e| e.into_inner());
                l.push_str(&line);
                l.push('\n');
            }
        }));
    }
    drop(tx); // 全 sender を落とすと rx が閉じる

    match rx.recv_timeout(Duration::from_secs(3)) {
        Ok(_) => {
            // 応答あり — 書き込みが完了するまで待つ
            let status = child.wait().ok();
            for h in handles {
                let _ = h.join();
            }
            let ok = status.map(|s| s.success()).unwrap_or(false);
            (ok, log.lock().unwrap_or_else(|e| e.into_inner()).clone())
        }
        Err(_) => {
            // タイムアウト — デバイスが応答しない
            let _ = child.kill();
            let _ = child.wait();
            for h in handles {
                let _ = h.join();
            }
            (
                false,
                "⏱ タイムアウト: 3秒以内にデバイスからの応答がありませんでした。\n\
                 マイコンが接続されているか、ポート設定を確認してください。"
                    .to_string(),
            )
        }
    }
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

pub fn flash_async(req: FlashRequest, tx: crossbeam_channel::Sender<crate::app::AppMessage>) {
    // 既存の flash() 関数を呼び出し、結果を AppMessage::FlashFinished に変換
    // board kind から preset を探してflashを呼ぶ
    use crate::core::board::BOARD_PRESETS;
    std::thread::spawn(move || {
        let preset_opt = BOARD_PRESETS.iter().find(|p| std::mem::discriminant(&p.kind) == std::mem::discriminant(&req.board));
        if let Some(preset) = preset_opt {
            let (ftx, frx) = crossbeam_channel::bounded(1);
            let _ = flash(preset, &req.port, &req.artifact, ftx);
            while let Ok(msg) = frx.recv() {
                if let FlashMessage::Finished { ok, log } = msg {
                    tx.send(crate::app::AppMessage::Flash(crate::app::FlashMsg::Finished(FlashResult { success: ok, output: log }))).ok();
                    break;
                }
            }
        } else {
            tx.send(crate::app::AppMessage::Flash(crate::app::FlashMsg::Finished(FlashResult { success: false, output: "Board not found".to_string() }))).ok();
        }
    });
}
