// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

//! コンパイララッパーの統合テスト

use alloide::core::compiler::{BuildRequest, BuildResult};
use std::path::PathBuf;

#[test]
fn build_request_creation() {
    let req = BuildRequest {
        project_dir: PathBuf::from("."),
        target_triple: "thumbv7em-none-eabihf".to_string(),
        release: false,
        board: None,
    };
    assert_eq!(req.target_triple, "thumbv7em-none-eabihf");
    assert!(!req.release);
}

#[test]
fn build_result_failure() {
    let r = BuildResult {
        success: false,
        stdout: String::new(),
        stderr: "error[E0001]: test error".to_string(),
        artifact_path: None,
        dist_path: None,
    };
    assert!(!r.success);
    assert!(r.artifact_path.is_none());
}

#[test]
fn build_result_success() {
    let r = BuildResult {
        success: true,
        stdout: "Compiling alloide v0.1.0".to_string(),
        stderr: String::new(),
        artifact_path: Some(PathBuf::from(
            "target/thumbv7em-none-eabihf/release/alloide",
        )),
        dist_path: None,
    };
    assert!(r.success);
    assert!(r.artifact_path.is_some());
}
