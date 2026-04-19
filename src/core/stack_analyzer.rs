// SPDX-License-Identifier: MIT OR Apache-2.0
// Stack analysis implementation

use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function: String,
    pub stack_usage: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct StackReport {
    pub total_estimate: u64,
    pub frames: Vec<StackFrame>,
    pub warnings: Vec<String>,
}

pub fn analyze_stack(elf_path: &Path) -> anyhow::Result<StackReport> {
    // Try arm-none-eabi-nm first, then nm
    let tools = ["arm-none-eabi-nm", "nm"];
    let mut out = None;
    for t in &tools {
        let res = Command::new(t)
            .arg("-S")
            .arg("--defined-only")
            .arg(elf_path.as_os_str())
            .output();
        if let Ok(o) = res {
            if o.status.success() {
                out = Some(String::from_utf8_lossy(&o.stdout).to_string());
                break;
            }
        }
    }

    let mut warnings = Vec::new();
    let mut frames = Vec::new();
    let mut total: u64 = 0;

    if let Some(text) = out {
        for line in text.lines() {
            // Expect lines like: 00000000 0000001c T function_name
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 { continue; }
            // size may be at index 1
            if let Ok(size) = u64::from_str_radix(parts[1], 16) {
                let name = parts[3..].join(" ");
                frames.push(StackFrame { function: name.clone(), stack_usage: Some(size) });
                total = total.saturating_add(size);
            }
        }
    } else {
        warnings.push("nm not found or failed to run; stack analysis unavailable".to_string());
    }

    if total == 0 {
        warnings.push("No symbol size information found".to_string());
    }

    Ok(StackReport { total_estimate: total, frames, warnings })
}
