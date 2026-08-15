// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use crate::core::event::{CoreEvent, ToolchainMsg};
use std::path::PathBuf;
use std::process::Command;

/// rust-analyzer のインストール状態
#[derive(Debug, Clone)]
pub struct RustAnalyzerStatus {
    pub is_installed: bool,
    pub path: Option<PathBuf>,
    pub version: Option<String>,
}

/// PATH と ~/.cargo/bin を確認して rust-analyzer の状態を返す
pub fn check_rust_analyzer() -> RustAnalyzerStatus {
    // which で PATH 上を探す
    if let Ok(path) = which::which("rust-analyzer") {
        let version = {
            let mut c = Command::new(&path);
            crate::core::no_window(&mut c)
                .arg("--version")
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
        };
        return RustAnalyzerStatus {
            is_installed: true,
            path: Some(path),
            version,
        };
    }
    // ~/.cargo/bin を直接確認（PATH が通っていない場合）
    if let Some(bin_dir) = home_cargo_bin() {
        let candidate = bin_dir.join(rust_analyzer_bin_name());
        if candidate.exists() {
            return RustAnalyzerStatus {
                is_installed: true,
                path: Some(candidate),
                version: None,
            };
        }
    }
    RustAnalyzerStatus {
        is_installed: false,
        path: None,
        version: None,
    }
}

/// 指定パスが実行可能ファイルとして有効か検証する
pub fn validate_custom_path(path: &std::path::Path) -> bool {
    path.exists() && path.is_file()
}

/// `rustup component add rust-analyzer` をバックグラウンドで実行する
/// 結果は CoreEvent::Toolchain(...) として tx へ送信される
pub fn install_rust_analyzer_async(tx: crossbeam_channel::Sender<CoreEvent>) {
    std::thread::spawn(move || {
        let _ = tx.send(CoreEvent::Toolchain(ToolchainMsg::InstallStarted));

        let mut rustup_cmd = Command::new("rustup");
        let result = crate::core::no_window(&mut rustup_cmd)
            .args(["component", "add", "rust-analyzer"])
            .output();

        match result {
            Ok(output) if output.status.success() => {
                let _ = tx.send(CoreEvent::Toolchain(ToolchainMsg::InstallFinished(Ok(
                    check_rust_analyzer(),
                ))));
            }
            Ok(output) => {
                let err = String::from_utf8_lossy(&output.stderr).to_string();
                let _ = tx.send(CoreEvent::Toolchain(ToolchainMsg::InstallFinished(Err(
                    err,
                ))));
            }
            Err(e) => {
                let _ = tx.send(CoreEvent::Toolchain(ToolchainMsg::InstallFinished(Err(
                    format!("rustup not found: {}", e),
                ))));
            }
        }
    });
}

/// ~/.cargo/bin ディレクトリパスを返す
pub fn home_cargo_bin() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".cargo").join("bin"))
}

fn rust_analyzer_bin_name() -> &'static str {
    if cfg!(windows) {
        "rust-analyzer.exe"
    } else {
        "rust-analyzer"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_returns_consistent_status() {
        let s = check_rust_analyzer();
        if s.is_installed {
            let p = s.path.expect("is_installed=true but path is None");
            assert!(p.exists(), "Reported path does not exist: {:?}", p);
        } else {
            assert!(s.path.is_none());
        }
    }

    #[test]
    fn validate_rejects_nonexistent_path() {
        assert!(!validate_custom_path(&PathBuf::from(
            "/nonexistent/rust-analyzer-fake-xyz"
        )));
    }

    #[test]
    fn validate_rejects_directory() {
        assert!(!validate_custom_path(&std::env::temp_dir()));
    }

    #[test]
    fn validate_accepts_real_executable() {
        if let Ok(exe) = std::env::current_exe() {
            assert!(
                validate_custom_path(&exe),
                "current exe should be valid: {:?}",
                exe
            );
        }
    }

    #[test]
    fn home_cargo_bin_contains_cargo() {
        if let Some(p) = home_cargo_bin() {
            let s = p.to_string_lossy();
            assert!(s.contains(".cargo"), "Expected .cargo in path: {}", s);
        }
    }
}
