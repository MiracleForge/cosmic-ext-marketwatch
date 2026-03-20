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

fn default_alerts_enabled() -> bool {
    true
}
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, CosmicConfigEntry, PartialEq, Serialize, Deserialize)]
#[version = 1]
pub struct Config {
    #[serde(default = "default_alerts_enabled")]
    pub alerts_enabled: bool,
    pub count_stokes_at_once: u64,
    pub count_news_by_simbol: u64,
    #[serde(default)]
    pub default_tab: PopupTab,
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
            alerts_enabled: true,
            count_stokes_at_once: 5,
            count_news_by_simbol: 1,
            default_tab: PopupTab::default(),
            last_wallet_index: 0,
            panel_stoke_rotation_interval: 20,
            refresh_interval: RefreshInterval::default(),
            show_only_icon: true,
            show_news: true,
        }
    }
}
