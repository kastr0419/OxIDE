// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

// UIが期待する型を追加
pub struct BuildRequest {
    pub project_dir: std::path::PathBuf,
    pub target_triple: String,
    pub release: bool,
    pub board: Option<crate::core::board::BoardKind>,
}

pub struct BuildResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    #[allow(dead_code)]
    pub artifact_path: Option<std::path::PathBuf>,
    /// ビルド成功時に成果物をコピーした dist/ フォルダのパス
    pub dist_path: Option<std::path::PathBuf>,
}

pub fn build_async(req: BuildRequest, tx: crossbeam_channel::Sender<crate::app::AppMessage>) {
    std::thread::spawn(move || {
        let mut cmd = std::process::Command::new("cargo");
        crate::core::no_window(&mut cmd);
        cmd.current_dir(&req.project_dir).arg("build");

        // .cargo/config.toml が存在する場合はそちらの target 設定を使う
        // （テンプレート生成プロジェクトは .cargo/config.toml で target を指定している）
        let has_cargo_config = req.project_dir.join(".cargo").join("config.toml").exists()
            || req.project_dir.join(".cargo").join("config").exists();
        if !has_cargo_config {
            if !req.target_triple.is_empty() {
                cmd.arg("--target").arg(&req.target_triple);
            }
            // ボードプリセットから rustflags と memory.x を自動注入
            if let Some(ref board_kind) = req.board {
                use crate::core::board::BOARD_PRESETS;
                if let Some(preset) = BOARD_PRESETS
                    .iter()
                    .find(|p| std::mem::discriminant(&p.kind) == std::mem::discriminant(board_kind))
                {
                    // RUSTFLAGS 注入
                    if !preset.rustflags.is_empty() {
                        let existing = std::env::var("RUSTFLAGS").unwrap_or_default();
                        let injected = preset.rustflags.join(" ");
                        let merged = match (existing.is_empty(), injected.is_empty()) {
                            (true, _) => injected.clone(),
                            (_, true) => existing.clone(),
                            _ => format!("{} {}", existing, injected),
                        };
                        if !merged.is_empty() {
                            cmd.env("RUSTFLAGS", merged);
                        }
                    }
                    // memory.x 自動生成（存在しない場合のみ）
                    if let Some(mem) = &preset.memory_layout {
                        let mem_x_path = req.project_dir.join("memory.x");
                        if !mem_x_path.exists() {
                            let content = format!(
                                "MEMORY\n{{\n  FLASH : ORIGIN = 0x{:08X}, LENGTH = {}K\n  RAM   : ORIGIN = 0x{:08X}, LENGTH = {}K\n}}\n",
                                mem.flash_origin, mem.flash_length_kb,
                                mem.ram_origin, mem.ram_length_kb
                            );
                            let _ = std::fs::write(&mem_x_path, content);
                        }
                    }
                }
            }
        } else {
            // .cargo/config.toml が存在する → 設定はそちらに任せる
        }

        if req.release {
            cmd.arg("--release");
        }
        cmd.stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        match cmd.spawn() {
            Ok(mut child) => {
                let read_output =
                    |stream: Box<dyn std::io::Read + Send>,
                     tx: crossbeam_channel::Sender<crate::app::AppMessage>| {
                        std::thread::spawn(move || {
                            use std::io::BufRead;
                            let mut output = String::new();
                            for line in std::io::BufReader::new(stream)
                                .lines()
                                .map_while(Result::ok)
                            {
                                output.push_str(&line);
                                output.push('\n');
                                tx.send(crate::app::AppMessage::Build(
                                    crate::app::BuildMsg::Progress(line),
                                ))
                                .ok();
                            }
                            output
                        })
                    };
                let stdout_handle = read_output(Box::new(child.stdout.take().unwrap()), tx.clone());
                let stderr_handle = read_output(Box::new(child.stderr.take().unwrap()), tx.clone());
                let success = child.wait().map(|status| status.success()).unwrap_or(false);
                let stdout = stdout_handle.join().unwrap_or_default();
                let stderr = stderr_handle.join().unwrap_or_default();
                let artifact_path = if success {
                    let mut p = req.project_dir.clone();
                    p.push("target");
                    p.push(&req.target_triple);
                    p.push(if req.release { "release" } else { "debug" });
                    Some(p)
                } else {
                    None
                };
                let dist_path = if success {
                    copy_artifacts_to_dist(&req.project_dir, &req.target_triple, req.release)
                } else {
                    None
                };
                tx.send(crate::app::AppMessage::Build(
                    crate::app::BuildMsg::Finished(BuildResult {
                        success,
                        stdout,
                        stderr,
                        artifact_path,
                        dist_path,
                    }),
                ))
                .ok();
            }
            Err(e) => {
                tx.send(crate::app::AppMessage::Build(
                    crate::app::BuildMsg::Finished(BuildResult {
                        success: false,
                        stdout: String::new(),
                        stderr: format!("cargo not found: {}", e),
                        artifact_path: None,
                        dist_path: None,
                    }),
                ))
                .ok();
            }
        }
    });
}

/// Cargo.toml の name フィールドを読む
fn get_package_name(project_dir: &std::path::Path) -> String {
    let Ok(content) = std::fs::read_to_string(project_dir.join("Cargo.toml")) else {
        return "blink".to_string();
    };
    content
        .lines()
        .find_map(|line| {
            let line = line.trim();
            if line.starts_with("name") && line.contains('=') {
                let val = line
                    .split_once('=')
                    .map(|(_, v)| v.trim().trim_matches('"'))?;
                if !val.is_empty() {
                    Some(val.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or_else(|| "blink".to_string())
}

/// ビルド成果物を `<project>/dist/` にコピーし、フォルダパスを返す
fn copy_artifacts_to_dist(
    project_dir: &std::path::Path,
    target_triple: &str,
    release: bool,
) -> Option<std::path::PathBuf> {
    let profile = if release { "release" } else { "debug" };
    let artifact_dir = if !target_triple.is_empty() {
        project_dir.join("target").join(target_triple).join(profile)
    } else {
        project_dir.join("target").join(profile)
    };

    if !artifact_dir.exists() {
        return None;
    }

    let pkg_name = get_package_name(project_dir);
    let dist_dir = project_dir.join("dist");
    std::fs::create_dir_all(&dist_dir).ok()?;

    // Try to generate .hex from ELF for DAPLink boards (try multiple objcopy tools)
    let elf_src = artifact_dir.join(&pkg_name);
    let hex_src = artifact_dir.join(format!("{}.hex", pkg_name));
    if elf_src.exists() && !hex_src.exists() {
        for tool in &["arm-none-eabi-objcopy", "rust-objcopy", "llvm-objcopy"] {
            let mut c = std::process::Command::new(tool);
            if crate::core::no_window(&mut c)
                .arg("-O")
                .arg("ihex")
                .arg(&elf_src)
                .arg(&hex_src)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                break;
            }
        }
    }

    // 成果物の拡張子候補（空文字 = ELF）
    let candidates = [
        ("", ".elf"),
        (".elf", ".elf"),
        (".hex", ".hex"),
        (".bin", ".bin"),
        (".uf2", ".uf2"),
        (".img", ".img"),
    ];
    let mut copied = false;
    for (src_ext, dst_ext) in &candidates {
        let src = artifact_dir.join(format!("{}{}", pkg_name, src_ext));
        if src.exists() && src.is_file() {
            let dst = dist_dir.join(format!("{}{}", pkg_name, dst_ext));
            if std::fs::copy(&src, &dst).is_ok() {
                copied = true;
            }
        }
    }

    if copied {
        Some(dist_dir)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_build_request_fields() {
        let req = BuildRequest {
            project_dir: PathBuf::from("/tmp/test"),
            target_triple: "thumbv7em-none-eabihf".to_string(),
            release: true,
            board: None,
        };
        assert_eq!(req.target_triple, "thumbv7em-none-eabihf");
        assert!(req.release);
    }

    #[test]
    fn test_build_result_fields() {
        let result = BuildResult {
            success: false,
            stdout: "out".to_string(),
            stderr: "error: something".to_string(),
            artifact_path: None,
            dist_path: None,
        };
        assert!(!result.success);
        assert!(result.stderr.contains("error"));
    }

    #[test]
    fn build_skips_target_when_cargo_config_exists() {
        // .cargo/config.toml が存在するディレクトリでは --target を渡さない判定を確認する
        let tmp = std::env::temp_dir().join("test_cargo_config_check");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".cargo"))
            .expect("create .cargo directory in test failed");

        // .cargo/config.toml なし → has_cargo_config = false
        let has_config = tmp.join(".cargo").join("config.toml").exists()
            || tmp.join(".cargo").join("config").exists();
        assert!(!has_config);

        // .cargo/config.toml あり → has_cargo_config = true
        std::fs::write(
            tmp.join(".cargo").join("config.toml"),
            "[build]\ntarget = \"thumbv7em-none-eabihf\"\n",
        )
        .expect("write .cargo/config.toml in test failed");
        let has_config = tmp.join(".cargo").join("config.toml").exists()
            || tmp.join(".cargo").join("config").exists();
        assert!(has_config);

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
