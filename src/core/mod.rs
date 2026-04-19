// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

pub mod board;
pub mod compiler;
pub mod build_analyzer;
pub mod flasher;
pub mod serial;
pub mod detector;
pub mod config;
pub mod snippets;
pub mod lsp;
pub mod toolchain;
pub mod project;
pub mod debugger;
pub mod svd_parser;
pub mod elf_analyzer;
pub mod pinout;
pub mod stack_analyzer;

/// Windows でコンソールウィンドウを表示しないよう CREATE_NO_WINDOW フラグを設定する。
/// 他OSでは何もしない。
pub fn no_window(cmd: &mut std::process::Command) -> &mut std::process::Command {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}
