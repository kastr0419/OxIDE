// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use anyhow::Result;
use std::fs;
use std::path::Path;

pub mod blink;

/// ボードに対応したLチカプロジェクトをワークスペースに生成する
pub fn create_blink_project(workspace: &Path, board: &crate::core::board::BoardKind) -> Result<()> {
    let tmpl = blink::get_blink_template(board)
        .ok_or_else(|| anyhow::anyhow!("No blink template for {:?}", board))?;

    // ディレクトリ構成を作成
    fs::create_dir_all(workspace.join("src"))?;
    fs::create_dir_all(workspace.join(".cargo"))?;

    // ファイルを書き出す
    fs::write(workspace.join("src/main.rs"),        tmpl.main_rs)?;
    fs::write(workspace.join("Cargo.toml"),         tmpl.cargo_toml)?;
    fs::write(workspace.join(".cargo/config.toml"), tmpl.cargo_config)?;
    fs::write(workspace.join("rust-toolchain.toml"), tmpl.rust_toolchain)?;

    if let Some(mem) = tmpl.memory_x {
        fs::write(workspace.join("memory.x"), mem)?;
    }
    if let Some(build) = tmpl.build_rs {
        fs::write(workspace.join("build.rs"), build)?;
    }
    if let Some(linker) = tmpl.linker_ld {
        fs::write(workspace.join("linker.ld"), linker)?;
    }
    if let Some((filename, content)) = tmpl.target_json {
        fs::write(workspace.join(filename), content)?;
    }

    Ok(())
}

/// 後方互換のため残す（旧API）
#[allow(dead_code)]
pub fn create_template(workspace: &Path, board: &str) -> Result<()> {
    // board文字列からBoardKindを解決して委譲
    use crate::core::board::{BOARD_PRESETS, BoardKind};
    let kind = BOARD_PRESETS.iter()
        .find(|p| p.display_name.to_lowercase().contains(board))
        .map(|p| p.kind.clone())
        .unwrap_or(BoardKind::ArduinoUno);
    create_blink_project(workspace, &kind)
}
