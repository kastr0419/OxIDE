// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

pub mod agent_panel;
pub mod board_picker;
pub mod build_panel;
pub mod editor;
pub mod file_explorer;
pub mod fonts;
pub mod help_panel;
pub mod inspect;
pub mod pinout_panel;
pub mod serial_monitor;
pub mod serial_plotter;
pub mod settings;
pub mod virtual_board_panel;
pub mod workbench;

pub use inspect::{debug_panel, elf_panel, rtt_panel, stack_panel, svd_panel};
