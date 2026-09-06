// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

/// Result from arm-none-eabi-size or avr-size
#[derive(Clone, Debug)]
pub struct BuildStats {
    pub flash_used: u64, // text + data
    pub ram_used: u64,   // data + bss
    pub flash_total: u64,
    pub ram_total: u64,
}

impl BuildStats {
    /// Parse `arm-none-eabi-size` or `avr-size` Berkeley format output:
    ///    text    data     bss     dec     hex filename
    ///    1234     100     200    1534     5fe  target/.../blink
    pub fn parse(size_output: &str, flash_total: u64, ram_total: u64) -> Option<Self> {
        // Find header line "text" then get the values line below it
        let mut lines = size_output.lines();
        // skip header lines until we find data
        for line in &mut lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                if let (Ok(text), Ok(data), Ok(bss)) = (
                    parts[0].parse::<u64>(),
                    parts[1].parse::<u64>(),
                    parts[2].parse::<u64>(),
                ) {
                    return Some(BuildStats {
                        flash_used: text + data,
                        ram_used: data + bss,
                        flash_total,
                        ram_total,
                    });
                }
            }
        }
        None
    }

    pub fn flash_percent(&self) -> f32 {
        if self.flash_total == 0 {
            return 0.0;
        }
        (self.flash_used as f32 / self.flash_total as f32).min(1.0)
    }

    pub fn ram_percent(&self) -> f32 {
        if self.ram_total == 0 {
            return 0.0;
        }
        (self.ram_used as f32 / self.ram_total as f32).min(1.0)
    }
}

/// Run the size tool for the given ELF and board, return BuildStats
pub fn analyze_elf(
    elf_path: &std::path::Path,
    board: &crate::core::board::BoardPreset,
) -> anyhow::Result<BuildStats> {
    use crate::core::board::BoardKind;

    let tool = match board.kind {
        BoardKind::ArduinoUno | BoardKind::ArduinoNano => "avr-size",
        _ => "arm-none-eabi-size",
    };
    let mut size_cmd = std::process::Command::new(tool);
    let output = crate::core::no_window(&mut size_cmd)
        .arg(elf_path)
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Derive totals: prefer explicit preset values, fallback to known sizes for common boards
    let (mut flash_total, mut ram_total) = (board.flash_bytes(), board.ram_bytes());
    if flash_total == 0 || ram_total == 0 {
        let (f, r) = match board.kind {
            BoardKind::ArduinoUno | BoardKind::ArduinoNano => (32_768u64, 2_048u64),
            BoardKind::Esp32 => (4_194_304u64, 532_480u64),
            BoardKind::MicroBitV2 | BoardKind::Stm32F4 => (524_288u64, 131_072u64),
            _ => (flash_total, ram_total),
        };
        if flash_total == 0 {
            flash_total = f;
        }
        if ram_total == 0 {
            ram_total = r;
        }
    }

    BuildStats::parse(&stdout, flash_total, ram_total)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse size output"))
}

pub fn find_elf(
    target_dir: &std::path::Path,
    board: &crate::core::board::BoardPreset,
) -> Option<std::path::PathBuf> {
    // Look in common target directories for the board's target triple and debug folders
    let candidates = [
        target_dir.join(board.target_triple).join("debug"),
        target_dir
            .join(board.target_triple)
            .join("debug")
            .join("deps"),
        target_dir.join("debug"),
        target_dir.join("debug").join("deps"),
    ];
    for dir in &candidates {
        if dir.exists() && dir.is_dir() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for e in entries.flatten() {
                    let p = e.path();
                    if p.extension().map(|x| x == "elf").unwrap_or(false) {
                        return Some(p);
                    }
                    // Some toolchains produce files without extension
                    if p.extension().is_none() && p.metadata().is_ok_and(|m| m.is_file()) {
                        return Some(p);
                    }
                }
            }
        }
    }
    // Fallback to recursive scan
    if let Ok(entries) = std::fs::read_dir(target_dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                if let Some(found) = find_elf(&p, board) {
                    return Some(found);
                }
            } else if p.extension().map(|x| x == "elf").unwrap_or(false) {
                return Some(p);
            }
        }
    }
    None
}
