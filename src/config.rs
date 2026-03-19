// SPDX-License-Identifier: GPL-3.0-only
use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PopupTab {
    #[default]
    Overview,
    Settings,
    Trending,
    News,
    Alerts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RefreshInterval {
    #[serde(rename = "5min")]
    FiveMinutes,
    #[serde(rename = "10min")]
    TenMinutes,
    #[serde(rename = "15min")]
    #[default]
    FifteenMinutes,
    #[serde(rename = "30min")]
    ThirtyMinutes,
    #[serde(rename = "60min")]
    OneHour,
}

impl RefreshInterval {
    pub fn as_minutes(self) -> u64 {
        match self {
            RefreshInterval::FiveMinutes => 5,
            RefreshInterval::TenMinutes => 10,
            RefreshInterval::FifteenMinutes => 15,
            RefreshInterval::ThirtyMinutes => 30,
            RefreshInterval::OneHour => 60,
        }
    }

    pub fn as_seconds(self) -> u64 {
        self.as_minutes() * 60
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AlertCondition {
    // Price
    PriceAbove(f64),
    PriceBelow(f64),
    // Percentual Variation
    VariationAbove(f64),
    VariationBelow(f64),
    // Variation flip
    TurnPositive,
    TurnNegative,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PriceAlert {
    pub id: u64,
    pub symbol: String,
    pub condition: AlertCondition,
    pub triggered: bool,
    pub enabled: bool,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, CosmicConfigEntry, PartialEq, Serialize, Deserialize)]
#[version = 1]
pub struct Config {
    pub alerts_enabled: bool,
    pub count_stokes_at_once: u64,
    pub count_news_by_simbol: u64,
    #[serde(default)]
    pub default_tab: PopupTab,
    pub is_using_system_colors: bool,
    pub last_wallet_index: usize,
    pub panel_stoke_rotation_interval: u64,
    pub refresh_interval: RefreshInterval,
    #[serde(default = "default_show_news")]
    pub show_news: bool,
    pub show_only_icon: bool,
}

fn default_show_news() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            alerts_enabled: false,
            count_stokes_at_once: 5,
            count_news_by_simbol: 5,
            default_tab: PopupTab::default(),
            is_using_system_colors: false,
            last_wallet_index: 0,
            panel_stoke_rotation_interval: 20,
            refresh_interval: RefreshInterval::default(),
            show_only_icon: false,
            show_news: true,
        }
    }
}
