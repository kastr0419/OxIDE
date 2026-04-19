// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// Named constants to avoid magic numbers
pub const DEFAULT_BAUD_RATE: u32 = 115_200;
pub const LEFT_PANEL_WIDTH: f32 = 250.0;
pub const RIGHT_PANEL_WIDTH: f32 = 300.0;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub last_board: Option<String>,
    pub last_port: Option<String>,
    pub workspace: Option<PathBuf>,
    pub workspace_dir: PathBuf,
    pub theme: Option<String>,
    /// rust-analyzer のカスタムパス（手動指定時）
    #[serde(default)]
    pub rust_analyzer_path: Option<PathBuf>,
}

impl AppConfig {
    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("rust-embedded-ide").join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        if let Some(p) = Self::path() {
            if p.exists() {
                let s = fs::read_to_string(p)?;
                match toml::from_str::<AppConfig>(&s) {
                    Ok(cfg) => return Ok(cfg),
                    Err(e) => {
                        eprintln!("Warning: config.toml parse error (falling back to default): {}", e);
                        return Ok(AppConfig::default());
                    }
                }
            }
        }
        Ok(AppConfig::default())
    }

    pub fn save(&self) -> Result<()> {
        if let Some(p) = Self::path() {
            if let Some(dir) = p.parent() {
                fs::create_dir_all(dir)?;
            }
            let s = toml::to_string_pretty(self)?;
            fs::write(p, s)?;
        }
        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            last_board: None,
            last_port: None,
            workspace: None,
            workspace_dir: dirs::document_dir()
                .or_else(dirs::home_dir)
                .map(|d| d.join("rust-embedded-projects"))
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default()),
            theme: None,
            rust_analyzer_path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let cfg = AppConfig::default();
        assert!(cfg.last_board.is_none());
        assert!(cfg.last_port.is_none());
    }

    #[test]
    fn test_config_serialize_deserialize() {
        let cfg = AppConfig {
            last_board: Some("ArduinoUno".to_string()),
            last_port: Some("COM3".to_string()),
            workspace: None,
            theme: Some("dark".to_string()),
            ..Default::default()
        };
        let toml_str = toml::to_string_pretty(&cfg).expect("serialize failed");
        assert!(toml_str.contains("ArduinoUno"));
        let cfg2: AppConfig = toml::from_str(&toml_str).expect("deserialize failed");
        assert_eq!(cfg2.last_board.as_deref(), Some("ArduinoUno"));
        assert_eq!(cfg2.last_port.as_deref(), Some("COM3"));
    }
}
