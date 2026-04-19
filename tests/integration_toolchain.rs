// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

//! rust-analyzer ツールチェーン管理の統合テスト

use oxide::core::toolchain::{
    check_rust_analyzer, validate_custom_path, home_cargo_bin,
};
use std::path::PathBuf;

#[test]
fn ra_status_path_exists_when_installed() {
    let s = check_rust_analyzer();
    if s.is_installed {
        let p = s.path.expect("is_installed=true but path is None");
        assert!(p.exists(), "Installed but path not found: {:?}", p);
    }
}

#[test]
fn ra_status_path_is_none_when_not_installed() {
    let s = check_rust_analyzer();
    if !s.is_installed {
        assert!(s.path.is_none());
    }
}

#[test]
fn validate_rejects_nonexistent_path() {
    assert!(!validate_custom_path(&PathBuf::from("C:/nonexistent/rust-analyzer-zzz.exe")));
}

#[test]
fn validate_rejects_directory() {
    assert!(!validate_custom_path(&std::env::temp_dir()));
}

#[test]
fn validate_accepts_real_executable() {
    if let Ok(exe) = std::env::current_exe() {
        assert!(validate_custom_path(&exe), "current exe should be valid: {:?}", exe);
    }
}

#[test]
fn home_cargo_bin_contains_cargo_segment() {
    if let Some(p) = home_cargo_bin() {
        let s = p.to_string_lossy();
        assert!(s.contains(".cargo"), "Expected .cargo in path: {}", s);
    }
}

#[test]
fn home_cargo_bin_ends_with_bin() {
    if let Some(p) = home_cargo_bin() {
        assert_eq!(
            p.file_name().and_then(|n| n.to_str()),
            Some("bin"),
            "Last segment should be 'bin': {:?}",
            p
        );
    }
}
