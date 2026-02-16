// SPDX-License-Identifier: GPL-3.0-only

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum StockExchange {
    #[default]
    Ibovespa,
    SP500,
}

impl StockExchange {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Ibovespa => "^BVSP",
            Self::SP500 => "^GSPC",
        }
    }

    pub fn currency(&self) -> &'static str {
        match self {
            Self::Ibovespa => "BRL",
            Self::SP500 => "USD",
        }
    }

    pub fn currency_symbol(&self) -> &'static str {
        match self {
            Self::Ibovespa => "R$",
            Self::SP500 => "$",
        }
    }

    pub fn format_price(&self, value: f64) -> String {
        match self {
            Self::Ibovespa => format!("{} {:.2}", self.currency_symbol(), value),
            Self::SP500 => format!("{} {:.2}", self.currency_symbol(), value),
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Ibovespa => "Ibovespa",
            Self::SP500 => "S&P 500",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PopupTab {
    #[default]
    Current,
    Alerts,
    Hourly,
    Forecast,
    Settings,
}

#[derive(Debug, Clone, CosmicConfigEntry, PartialEq, Serialize, Deserialize)]
#[version = 1]
pub struct Config {
    pub alerts_enabled: bool,
    pub count_stokes_at_once: u64,
    #[serde(default)]
    pub default_tab: PopupTab,
    pub is_using_system_colors: bool,
    pub panel_stoke_rotation_interval: u64,
    pub refresh_interval_minutes: u64,
    #[serde(default = "default_show_icon_in_panel")]
    pub show_icon_in_panel: bool,
    pub stock_exchange: StockExchange,
}

fn default_show_icon_in_panel() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            alerts_enabled: false,
            count_stokes_at_once: 5,
            default_tab: PopupTab::default(),
            is_using_system_colors: false,
            panel_stoke_rotation_interval: 60,
            refresh_interval_minutes: 15,
            show_icon_in_panel: true,
            stock_exchange: StockExchange::default(),
        }
    }
}
