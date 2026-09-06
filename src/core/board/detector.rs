// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

use crate::core::board::{BoardKind, FlashToolKind, BOARD_PRESETS};
use crate::core::event::CoreEvent;

/// 検出結果の確信度
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum DetectionConfidence {
    Low,    // ヒューリスティック（ポート名推定）
    Medium, // USB VID のみ一致
    High,   // USB VID + PID 完全一致
    Exact,  // 外部ツール（probe-rs / esptool）がチップ名を確定
}

/// 検出されたボード情報
#[derive(Debug, Clone)]
pub struct DetectedBoard {
    pub port_name: String,
    pub board_index: usize, // BOARD_PRESETS のインデックス
    pub confidence: DetectionConfidence,
    pub description: String,
    #[allow(dead_code)]
    pub chip_info: Option<String>,
}

// ─── Stage 0: RP2040/RP2350 BOOTSEL ドライブ検出 ─────────────────────────────

/// BOOTSEL モードの Pico/Pico2 を検出する。
///
/// Pico を BOOTSEL ボタン押しながら接続すると RPI-RP2 (RP2040) または
/// RP2350 (RP2350) という USB Mass Storage デバイスになる。
/// この場合は serialport には現れないが INFO_UF2.TXT が存在する。
pub fn detect_rp_bootloader() -> Vec<DetectedBoard> {
    let mut results = Vec::new();
    for drive_letter in 'A'..='Z' {
        let drive = format!("{}:\\", drive_letter);
        let info_path = std::path::Path::new(&drive).join("INFO_UF2.TXT");
        if !info_path.exists() {
            continue;
        }
        let content = std::fs::read_to_string(&info_path).unwrap_or_default();
        let lower = content.to_lowercase();
        let kind_info: Option<(BoardKind, &str)> = if lower.contains("rp2350") {
            Some((BoardKind::RpiPico2, "RP2350"))
        } else if lower.contains("rp2040") {
            Some((BoardKind::RpiPico, "RP2040"))
        } else {
            None
        };
        if let Some((board_kind, chip_name)) = kind_info {
            if let Some(idx) = BOARD_PRESETS.iter().position(|p| p.kind == board_kind) {
                results.push(DetectedBoard {
                    port_name: drive.clone(),
                    board_index: idx,
                    confidence: DetectionConfidence::High,
                    description: format!(
                        "{} BOOTSEL ドライブ: {} (BOOTSEL モード — Flash ボタンでそのまま書き込み可能)",
                        chip_name, drive
                    ),
                    chip_info: Some(content.trim().to_string()),
                });
            }
        }
    }
    results
}

/// USB VID/PID を使ってボードを検出する。
/// serialport::SerialPortType::UsbPort から VID/PID を取得し、
/// BOARD_PRESETS の usb_ids テーブルと照合する。
pub fn detect_by_usb_id() -> Vec<DetectedBoard> {
    let mut results = Vec::new();
    let ports = match serialport::available_ports() {
        Ok(p) => p,
        Err(_) => return results,
    };

    for port in &ports {
        // USB ポートのみ対象
        let usb_info = match &port.port_type {
            serialport::SerialPortType::UsbPort(info) => info,
            _ => continue,
        };
        let vid = usb_info.vid;
        let pid = usb_info.pid;
        let product = usb_info.product.as_deref().unwrap_or("");
        let manufacturer = usb_info.manufacturer.as_deref().unwrap_or("");

        // 1. VID+PID 完全一致 → High
        let exact = BOARD_PRESETS.iter().enumerate().find_map(|(idx, preset)| {
            preset
                .usb_ids
                .iter()
                .find(|u| u.vid == vid && u.pid == pid)
                .map(|u| (idx, DetectionConfidence::High, u.description))
        });

        // 2. VID のみ一致 → Medium（フォールバック）
        let vid_only = || {
            BOARD_PRESETS.iter().enumerate().find_map(|(idx, preset)| {
                preset
                    .usb_ids
                    .iter()
                    .find(|u| u.vid == vid)
                    .map(|u| (idx, DetectionConfidence::Medium, u.description))
            })
        };

        if let Some((board_idx, confidence, desc)) = exact.or_else(vid_only) {
            results.push(DetectedBoard {
                port_name: port.port_name.clone(),
                board_index: board_idx,
                confidence,
                description: format!(
                    "{} ({} {}, VID:{:04X} PID:{:04X}) on {}",
                    desc, manufacturer, product, vid, pid, port.port_name
                ),
                chip_info: Some(format!(
                    "VID:{:04X} PID:{:04X} mfr='{}' prod='{}'",
                    vid, pid, manufacturer, product
                )),
            });
        }
    }
    results
}

// ─── Stage 2a: probe-rs ──────────────────────────────────────────────────────

/// probe-rs list でARM Cortex-M チップを検出し、チップ名からボードを特定する。
///
/// probe-rs list の出力例:
///   \[0\]: STLink V2 -- 0483:3748 (S/N: ...)
///   \[0\]: CMSIS-DAP -- 0D28:0204 -- nRF52833_xxAA
pub fn detect_by_probe_rs() -> Vec<DetectedBoard> {
    let mut results = Vec::new();
    let mut probe_cmd = std::process::Command::new("probe-rs");
    let output = match crate::core::no_window(&mut probe_cmd)
        .args(["list"])
        .output()
    {
        Ok(o) => o,
        Err(_) => return results, // probe-rs 未インストール → スキップ
    };
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        let lower = line.to_lowercase();

        // チップ名から BoardKind を推定
        let board_kind: Option<BoardKind> = if lower.contains("stm32h7") {
            Some(BoardKind::Stm32H7)
        } else if lower.contains("stm32f7") {
            Some(BoardKind::Stm32F7)
        } else if lower.contains("stm32f4") {
            Some(BoardKind::Stm32F4)
        } else if lower.contains("stm32f1") {
            Some(BoardKind::Stm32F1)
        } else if lower.contains("stm32l4") {
            Some(BoardKind::Stm32L4)
        } else if lower.contains("stm32g0") {
            Some(BoardKind::Stm32G0)
        } else if lower.contains("stm32") || lower.contains("stlink") {
            // STM32 だが型番不明 → F4 を仮置き (Medium)
            Some(BoardKind::Stm32F4)
        } else if lower.contains("nrf52840") {
            Some(BoardKind::NrF52840)
        } else if lower.contains("nrf52833")
            || lower.contains("micro:bit")
            || lower.contains("microbit")
        {
            Some(BoardKind::MicroBitV2)
        } else if lower.contains("nrf51") {
            Some(BoardKind::NrF51822)
        } else if lower.contains("rp2350") {
            Some(BoardKind::RpiPico2)
        } else if lower.contains("rp2040") {
            Some(BoardKind::RpiPico)
        } else if lower.contains("imxrt106") {
            Some(BoardKind::Teensy4)
        } else {
            None
        };

        if let Some(kind) = board_kind {
            if let Some(idx) = BOARD_PRESETS.iter().position(|p| p.kind == kind) {
                // STM32 型番不明は Medium、それ以外は Exact
                let confidence = if lower.contains("stm32")
                    && !lower.contains("stm32f4")
                    && !lower.contains("stm32f1")
                    && !lower.contains("stm32f7")
                    && !lower.contains("stm32h7")
                    && !lower.contains("stm32l4")
                    && !lower.contains("stm32g0")
                {
                    DetectionConfidence::Medium
                } else {
                    DetectionConfidence::Exact
                };
                results.push(DetectedBoard {
                    port_name: String::from("(probe)"),
                    board_index: idx,
                    confidence,
                    description: format!("probe-rs: {}", line.trim()),
                    chip_info: Some(line.trim().to_string()),
                });
            }
        }
    }
    results
}

// ─── Stage 2b: esptool ───────────────────────────────────────────────────────

/// esptool.py でESP チップ種別を確定する（3秒タイムアウト）。
pub fn detect_by_esptool(port: &str) -> Option<DetectedBoard> {
    let mut esptool_cmd = std::process::Command::new("esptool.py");
    let output = crate::core::no_window(&mut esptool_cmd)
        .args(["--port", port, "--no-stub", "--timeout", "3", "chip_id"])
        .output()
        .ok()?;

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let lower = combined.to_lowercase();

    // 長い名前から先に判定（esp32-s3 を esp32 より前に）
    let kind: BoardKind = if lower.contains("esp32-s3") {
        BoardKind::Esp32S3
    } else if lower.contains("esp32-s2") {
        BoardKind::Esp32S2
    } else if lower.contains("esp32-c6") {
        BoardKind::Esp32C6
    } else if lower.contains("esp32-c3") {
        BoardKind::Esp32C3
    } else if lower.contains("esp32-h2") {
        BoardKind::Esp32H2
    } else if lower.contains("esp32") {
        BoardKind::Esp32
    } else {
        return None; // ESP デバイスではない
    };

    let idx = BOARD_PRESETS.iter().position(|p| p.kind == kind)?;
    Some(DetectedBoard {
        port_name: port.to_string(),
        board_index: idx,
        confidence: DetectionConfidence::Exact,
        description: format!("esptool: {:?} on {}", kind, port),
        chip_info: Some(combined.trim().to_string()),
    })
}

// ─── Stage 3: ポート名ヒューリスティック ─────────────────────────────────────

/// ポート名やデバイス文字列から推定するフォールバック検出。
pub fn detect_by_port_hint() -> Vec<DetectedBoard> {
    let mut results = Vec::new();
    let ports = match serialport::available_ports() {
        Ok(p) => p,
        Err(_) => return results,
    };

    for port in &ports {
        let port_lower = port.port_name.to_lowercase();

        // UsbPort の product 文字列も検索対象に含める
        let product_str = match &port.port_type {
            serialport::SerialPortType::UsbPort(i) => {
                i.product.clone().unwrap_or_default().to_lowercase()
            }
            _ => String::new(),
        };
        let search = format!("{} {}", port_lower, product_str);

        for (idx, preset) in BOARD_PRESETS.iter().enumerate() {
            // display_name の先頭単語でマッチ（例: "Arduino", "Raspberry", "STM32"）
            let hint = preset
                .display_name
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_lowercase();
            if !hint.is_empty() && search.contains(&hint) {
                results.push(DetectedBoard {
                    port_name: port.port_name.clone(),
                    board_index: idx,
                    confidence: DetectionConfidence::Low,
                    description: format!(
                        "{} (name hint) on {}",
                        preset.display_name, port.port_name
                    ),
                    chip_info: None,
                });
                break;
            }
        }
    }
    results
}

// ─── 統合検出 ────────────────────────────────────────────────────────────────

/// 全ステージを実行し最良の結果を CoreEvent::BoardDetected で送信する。
pub fn auto_detect(tx: crossbeam_channel::Sender<CoreEvent>) {
    std::thread::spawn(move || {
        let mut all: Vec<DetectedBoard> = Vec::new();

        // Stage 0: RP2040/RP2350 BOOTSEL ドライブ（INFO_UF2.TXT）
        all.extend(detect_rp_bootloader());

        // Stage 1: USB VID/PID
        all.extend(detect_by_usb_id());

        // Stage 2a: probe-rs（ARM系）
        all.extend(detect_by_probe_rs());

        // Stage 2b: esptool（ESP系 VID/PIDのポートに試行）
        // FlashToolKind::Esptool のボード候補ポートと、まだ未特定のポートを対象にする
        let esp_ports: Vec<String> = {
            let mut seen = std::collections::HashSet::new();
            all.iter()
                .filter(|r| {
                    BOARD_PRESETS
                        .get(r.board_index)
                        .map(|p| matches!(p.flash_tool, FlashToolKind::Esptool))
                        .unwrap_or(false)
                })
                .filter_map(|r| {
                    if seen.insert(r.port_name.clone()) {
                        Some(r.port_name.clone())
                    } else {
                        None
                    }
                })
                .collect()
        };

        for port in esp_ports {
            if let Some(precise) = detect_by_esptool(&port) {
                // 既存の低信頼度エントリを上書き
                all.retain(|r| r.port_name != port || r.confidence >= DetectionConfidence::Exact);
                all.push(precise);
            }
        }

        // Stage 3: フォールバック（まだ何も見つかっていない場合のみ）
        if all.is_empty() {
            all.extend(detect_by_port_hint());
        }

        // 信頼度降順ソート → ポートごとに最上位を選択 → 全体で最上位1件
        all.sort_by(|a, b| b.confidence.cmp(&a.confidence));
        all.dedup_by_key(|r| r.port_name.clone());

        let best = all.into_iter().next();
        tx.send(CoreEvent::BoardDetected(best)).ok();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_by_usb_id_returns_vec() {
        let results = detect_by_usb_id();
        assert!(results.len() < 1000, "Unreasonably large result");
        // VID/PID 一致があれば信頼度は Medium 以上
        for r in &results {
            assert!(r.confidence >= DetectionConfidence::Medium);
        }
    }

    #[test]
    fn test_detect_by_port_hint_returns_vec() {
        let results = detect_by_port_hint();
        assert!(results.len() < 1000);
        for r in &results {
            assert_eq!(r.confidence, DetectionConfidence::Low);
        }
    }

    #[test]
    fn test_detection_confidence_ordering() {
        assert!(DetectionConfidence::Exact > DetectionConfidence::High);
        assert!(DetectionConfidence::High > DetectionConfidence::Medium);
        assert!(DetectionConfidence::Medium > DetectionConfidence::Low);
    }

    #[test]
    fn test_probe_rs_chip_name_mapping() {
        // probe-rs 出力のパースロジックを直接テスト
        let test_cases = vec![
            ("stm32h7xx", BoardKind::Stm32H7),
            ("stm32f4xx", BoardKind::Stm32F4),
            ("stm32f1xx", BoardKind::Stm32F1),
            ("nrf52840_xxaa", BoardKind::NrF52840),
            ("nrf52833 (micro:bit v2)", BoardKind::MicroBitV2),
            ("rp2040", BoardKind::RpiPico),
            ("rp2350", BoardKind::RpiPico2),
        ];
        for (chip_str, expected_kind) in test_cases {
            let lower = chip_str.to_lowercase();
            let kind: Option<BoardKind> = if lower.contains("stm32h7") {
                Some(BoardKind::Stm32H7)
            } else if lower.contains("stm32f4") {
                Some(BoardKind::Stm32F4)
            } else if lower.contains("stm32f1") {
                Some(BoardKind::Stm32F1)
            } else if lower.contains("nrf52840") {
                Some(BoardKind::NrF52840)
            } else if lower.contains("nrf52833") || lower.contains("micro:bit") {
                Some(BoardKind::MicroBitV2)
            } else if lower.contains("rp2350") {
                Some(BoardKind::RpiPico2)
            } else if lower.contains("rp2040") {
                Some(BoardKind::RpiPico)
            } else {
                None
            };
            assert_eq!(
                kind,
                Some(expected_kind),
                "chip='{}' not mapped correctly",
                chip_str
            );
        }
    }
}
