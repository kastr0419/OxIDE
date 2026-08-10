// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

#![allow(dead_code)]

use crate::core::board::BoardKind;
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct SimulationRequest {
    pub board: BoardKind,
    pub artifact: PathBuf,
}

pub fn is_supported(board: &BoardKind) -> bool {
    matches!(board, BoardKind::Stm32F1)
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
        .join(".oxide-sim.resc");
    std::fs::write(&script_path, script(&req.artifact)?)
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

fn script(artifact: &Path) -> Result<String> {
    let artifact = renode_path(artifact);
    if artifact.contains(['\r', '\n']) {
        bail!("simulation artifact path contains a newline");
    }
    Ok(format!(
        "mach create\nmachine LoadPlatformDescription @platforms/cpus/stm32f103.repl\nmachine LoadPlatformDescriptionFromString \"led: Miscellaneous.LED @ gpioPortC 13\"\nsysbus LoadELF @\"{}\"\nstart\n",
        artifact
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
    fn supports_only_stm32f1() {
        assert!(is_supported(&BoardKind::Stm32F1));
        assert!(!is_supported(&BoardKind::Stm32F4));
    }

    #[test]
    fn script_escapes_portable_artifact_path() {
        let generated = script(Path::new("C:\\firmware\\build \"one\".elf")).unwrap();
        assert!(generated.contains(r#"sysbus LoadELF @"C:/firmware/build \"one\".elf""#));
        assert!(generated.contains("gpioPortC 13"));
    }

    #[test]
    fn rejects_newline_in_artifact_path() {
        assert!(script(Path::new("firmware\nquit.elf")).is_err());
    }
}
