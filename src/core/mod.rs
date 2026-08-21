// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

pub mod agent;
pub mod board;
pub mod build;
pub mod editor;
pub mod event;
pub mod inspect;
pub mod process;
pub mod serial;
pub mod simulator;

pub use board::{detector, pinout};
pub use build::{analyzer as build_analyzer, compiler, flasher, toolchain};
pub use editor::{lsp, project, snippets};
pub use inspect::{debugger, elf as elf_analyzer, stack as stack_analyzer, svd as svd_parser};
pub use process::no_window;
