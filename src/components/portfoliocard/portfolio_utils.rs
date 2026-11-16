// SPDX-License-Identifier: GPL-3.0-only
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::marketwatch::PriceAlert;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortFolio {
    pub name: String,
    pub symbols: Vec<String>,
    #[serde(default)]
    pub alerts: Vec<PriceAlert>,
}

impl PortFolio {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            symbols: Vec::new(),
            alerts: Vec::new(),
        }
    }
}

pub fn data_path() -> PathBuf {
    let base = std::env::var("XDG_DATA_HOME").map_or_else(
        |_| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".local/share")
        },
        PathBuf::from,
    );
    base.join("com.github.MiracleForge.cosmic-ext-marketwatch")
}

pub fn portfolio_file() -> PathBuf {
    let new_path = data_path().join("portfolios.json");
    let old_path = data_path().join("wallet.json");

    // old file name
    if !new_path.exists() && old_path.exists() {
        if let Err(error) = std::fs::rename(&old_path, &new_path) {
            eprintln!("Failed to migrate wallet data: {error}");
        }
    }

    new_path
}

pub fn load_portfolio() -> Vec<PortFolio> {
    let path = portfolio_file();
    if !path.exists() {
        return Vec::new();
    }
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_portfolios(portfolio: &[PortFolio]) {
    let path = portfolio_file();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(portfolio) {
        let _ = std::fs::write(&path, json);
    }
}
