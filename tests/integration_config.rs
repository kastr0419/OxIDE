// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

//! 設定の統合テスト

use oxide::core::config::AppConfig;

#[test]
fn config_roundtrip_toml() {
    let cfg = AppConfig {
        last_board: Some("Esp32".to_string()),
        last_port: Some("COM5".to_string()),
        workspace: Some(std::path::PathBuf::from("D:/projects/my_app")),
        theme: Some("dark".to_string()),
        ..Default::default()
    };
    let s = toml::to_string_pretty(&cfg).unwrap();
    let cfg2: AppConfig = toml::from_str(&s).unwrap();
    assert_eq!(cfg.last_board, cfg2.last_board);
    assert_eq!(cfg.last_port,  cfg2.last_port);
    assert_eq!(cfg.workspace,  cfg2.workspace);
    assert_eq!(cfg.theme,      cfg2.theme);
}

#[test]
fn config_default_is_empty() {
    let cfg = AppConfig::default();
    assert!(cfg.last_board.is_none());
    assert!(cfg.last_port.is_none());
    assert!(cfg.theme.is_none());
}
