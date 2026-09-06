// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 ALLoIDE contributors

#![allow(dead_code)]

use crate::core::board::BoardKind;
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct SimulationRequest {
    pub board: BoardKind,
    pub artifact: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSupport {
    Supported {
        platform: &'static str,
        gpio: &'static str,
        pin: u8,
    },
    Unsupported(&'static str),
}

pub fn support(board: &BoardKind) -> SimulationSupport {
    use BoardKind::*;
    match board {
        Samd21 => SimulationSupport::Supported {
            platform: "platforms/cpus/atsamd21j17d-aft.repl",
            gpio: "gpio_a",
            pin: 17,
        },
        Stm32F1 => SimulationSupport::Supported {
            platform: "platforms/cpus/stm32f103.repl",
            gpio: "gpioPortC",
            pin: 13,
        },
        Stm32F7 => SimulationSupport::Supported {
            platform: "platforms/cpus/stm32f746.repl",
            gpio: "gpioPortB",
            pin: 7,
        },
        Stm32H7 => SimulationSupport::Supported {
            platform: "platforms/cpus/stm32h743.repl",
            gpio: "gpioPortB",
            pin: 14,
        },
        Stm32G0 => SimulationSupport::Supported {
            platform: "platforms/cpus/stm32g0.repl",
            gpio: "gpioPortA",
            pin: 5,
        },
        NrF52840 => SimulationSupport::Supported {
            platform: "platforms/cpus/nrf52840.repl",
            gpio: "gpio0",
            pin: 13,
        },
        Samd51 => SimulationSupport::Unsupported(
            "Renode's ATSAMD51G19A CPU model has no GPIO peripheral model",
        ),
        ArduinoUno | ArduinoNano | ArduinoMega | ArduinoLeonardo | RpiPico | RpiPico2 | RpiZero
        | ArduinoDue | NrF51822 | Stm32F4 | Stm32L4 | MicroBitV2 | Teensy4 | Esp32 | Esp32S2
        | Esp32S3 | Esp32C3 | Esp32C6 | Esp32H2 | Gd32Vf103 | Ch32V003 => {
            SimulationSupport::Unsupported("Renode has no matching official MCU and GPIO model")
        }
    }
}

pub fn is_supported(board: &BoardKind) -> bool {
    matches!(support(board), SimulationSupport::Supported { .. })
}

pub fn launch(req: &SimulationRequest) -> Result<PathBuf> {
    if !is_supported(&req.board) {
        bail!("Renode simulation is not supported for {:?}", req.board);
    }
    if !req.artifact.is_file() {
        bail!(
            "simulation artifact is not a file: {}",
            req.artifact.display()
        );
    }

    let renode = find_renode().context("Renode executable was not found in PATH")?;
    let script_path = req
        .artifact
        .parent()
        .context("simulation artifact has no parent directory")?
        .join(".alloide-sim.resc");
    std::fs::write(&script_path, script(&req.board, &req.artifact)?)
        .with_context(|| format!("failed to write {}", script_path.display()))?;

    crate::core::no_window(&mut Command::new(renode))
        .arg(&script_path)
        .spawn()
        .context("failed to launch Renode")?;
    Ok(script_path)
}

fn find_renode() -> Option<PathBuf> {
    which::which("renode").ok().or_else(|| {
        if cfg!(windows) {
            which::which("Renode").ok()
        } else {
            None
        }
    })
}

fn script(board: &BoardKind, artifact: &Path) -> Result<String> {
    let SimulationSupport::Supported {
        platform,
        gpio,
        pin,
    } = support(board)
    else {
        bail!("Renode simulation is not supported for {:?}", board);
    };
    let artifact = renode_path(artifact);
    if artifact.contains(['\r', '\n']) {
        bail!("simulation artifact path contains a newline");
    }
    Ok(format!(
        "mach create\nmachine LoadPlatformDescription @{platform}\nmachine LoadPlatformDescriptionFromString \"led: Miscellaneous.LED @ {gpio} {pin}\"\nsysbus LoadELF @\"{artifact}\"\nstart\n"
    ))
}

fn renode_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_has_an_explicit_support_decision() {
        let mut supported = 0;
        for preset in crate::core::board::BOARD_PRESETS {
            match support(&preset.kind) {
                SimulationSupport::Supported { .. } => supported += 1,
                SimulationSupport::Unsupported(reason) => assert!(!reason.is_empty()),
            }
        }
        assert_eq!(supported, 6);
    }

    #[test]
    fn supported_scripts_use_the_exact_platform_and_gpio() {
        let expected = [
            (BoardKind::Samd21, "atsamd21j17d-aft.repl", "gpio_a 17"),
            (BoardKind::Stm32F1, "stm32f103.repl", "gpioPortC 13"),
            (BoardKind::Stm32F7, "stm32f746.repl", "gpioPortB 7"),
            (BoardKind::Stm32H7, "stm32h743.repl", "gpioPortB 14"),
            (BoardKind::Stm32G0, "stm32g0.repl", "gpioPortA 5"),
            (BoardKind::NrF52840, "nrf52840.repl", "gpio0 13"),
        ];
        for (board, platform, gpio) in expected {
            let generated = script(&board, Path::new("C:\\firmware\\build \"one\".elf")).unwrap();
            assert!(generated.contains(platform));
            assert!(generated.contains(gpio));
            assert!(generated.contains(r#"sysbus LoadELF @"C:/firmware/build \"one\".elf""#));
        }
    }

    #[test]
    fn unsupported_boards_explain_why() {
        assert!(
            matches!(support(&BoardKind::Stm32F4), SimulationSupport::Unsupported(reason) if reason.contains("no matching"))
        );
        assert!(
            matches!(support(&BoardKind::Samd51), SimulationSupport::Unsupported(reason) if reason.contains("no GPIO"))
        );
    }

    #[test]
    fn rejects_newline_in_artifact_path() {
        assert!(script(&BoardKind::Stm32F1, Path::new("firmware\nquit.elf")).is_err());
    }
}
