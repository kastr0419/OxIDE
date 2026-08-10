// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

//! 自動検出ロジックの統合テスト

use oxide::core::board::BOARD_PRESETS;
use oxide::core::detector::{detect_by_port_hint, detect_by_usb_id, DetectionConfidence};

#[test]
fn detect_usb_returns_valid_indices() {
    let results = detect_by_usb_id();
    for r in &results {
        assert!(
            r.board_index < BOARD_PRESETS.len(),
            "board_index {} out of range (len={})",
            r.board_index,
            BOARD_PRESETS.len()
        );
    }
}

#[test]
fn detect_port_hint_returns_valid_indices() {
    let results = detect_by_port_hint();
    for r in &results {
        assert!(r.board_index < BOARD_PRESETS.len());
        assert!(!r.port_name.is_empty());
    }
}

#[test]
fn confidence_ord_is_correct() {
    assert!(DetectionConfidence::Exact > DetectionConfidence::High);
    assert!(DetectionConfidence::High > DetectionConfidence::Medium);
    assert!(DetectionConfidence::Medium > DetectionConfidence::Low);
}
