// SPDX-License-Identifier: GPL-3.0-only

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PopupTab {
    #[default]
    Settings,
    Overview,
    Trending,
    News,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefreshInterval {
    #[serde(rename = "5min")]
    FiveMinutes,
    #[serde(rename = "10min")]
    TenMinutes,
    #[serde(rename = "15min")]
    FifteenMinutes,
    #[serde(rename = "30min")]
    ThirtyMinutes,
    #[serde(rename = "60min")]
    OneHour,
}

impl RefreshInterval {
    pub fn as_minutes(&self) -> u64 {
        match self {
            RefreshInterval::FiveMinutes => 5,
            RefreshInterval::TenMinutes => 10,
            RefreshInterval::FifteenMinutes => 15,
            RefreshInterval::ThirtyMinutes => 30,
            RefreshInterval::OneHour => 60,
        }
    }

    pub fn as_seconds(&self) -> u64 {
        self.as_minutes() * 60
    }
}

impl Default for RefreshInterval {
    fn default() -> Self {
        RefreshInterval::FifteenMinutes
    }
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
    pub refresh_interval: RefreshInterval,
    #[serde(default = "default_show_icon_in_panel")]
    pub show_only_icon: bool,
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
            refresh_interval: RefreshInterval::default(),
            show_only_icon: true,
        }
    }
}
