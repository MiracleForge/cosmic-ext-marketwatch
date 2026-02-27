use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub name: String,
    pub symbols: Vec<String>,
}

impl Wallet {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            symbols: Vec::new(),
        }
    }
}

pub fn data_path() -> PathBuf {
    let base = std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".local/share")
        });

    base.join("com.github.MiracleForge.cosmic-marketwatch")
}

pub fn wallets_file() -> PathBuf {
    data_path().join("wallets.json")
}

pub fn load_wallets() -> Vec<Wallet> {
    let path = wallets_file();
    if !path.exists() {
        return Vec::new();
    }
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_wallets(wallets: &[Wallet]) {
    let path = wallets_file();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(wallets) {
        let _ = std::fs::write(&path, json);
    }
}
