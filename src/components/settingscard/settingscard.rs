// SPDX-License-Identifier: GPL-3.0-only
use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::widget::{self, settings};

use crate::app::Message;
use crate::config::{Config, RefreshInterval};
use crate::constants::REFRESH_OPTIONS;

pub fn settingscard(config: &Config) -> Element<'_, Message> {
    let news_count = config.count_news_by_simbol.to_string();
    let panel_rotation = config.panel_stoke_rotation_interval.to_string();

    let refresh_values = [
        RefreshInterval::FiveMinutes,
        RefreshInterval::TenMinutes,
        RefreshInterval::FifteenMinutes,
        RefreshInterval::ThirtyMinutes,
        RefreshInterval::OneHour,
    ];

    let selected_refresh = refresh_values
        .iter()
        .position(|value| *value == config.refresh_interval)
        .unwrap_or(0);

    let refresh_input = widget::dropdown(&REFRESH_OPTIONS, Some(selected_refresh), move |index| {
        Message::SetRefreshInterval(
            refresh_values
                .get(index)
                .copied()
                .unwrap_or(RefreshInterval::FiveMinutes),
        )
    })
    .width(Length::Fill);

    let panel = settings::section()
        .title(crate::fl!("settings-panel"))
        .add(
            settings::item::builder(crate::fl!("settings-show-icon-only"))
                .toggler(config.show_only_icon, Message::ToggleShowOnlyIcon),
        )
        .add(
            settings::item::builder(crate::fl!("settings-show-logos-panel"))
                .toggler(config.show_panel_logos, Message::ToggleShowLogoPanel),
        )
        .add(
            settings::item::builder(crate::fl!("settings-disable-custom-alerts"))
                .toggler(config.alerts_enabled, Message::ToggleAlertsEnabled),
        );

    let display = settings::section()
        .title(crate::fl!("settings-display"))
        .add(
            settings::item::builder(crate::fl!("settings-show-logos"))
                .toggler(config.show_logos, Message::ToggleShowLogos),
        )
        .add(
            settings::item::builder(crate::fl!("settings-display-news"))
                .toggler(config.show_news, Message::ToggleShowNews),
        )
        .add(
            settings::item::builder(crate::fl!("settings-news-per-asset")).control(
                widget::text_input("1", news_count)
                    .on_input(Message::SetNumberOfNewsBySymbols)
                    .width(Length::Fill),
            ),
        );

    let refresh = settings::section()
        .title(crate::fl!("settings-refresh"))
        .add(
            settings::item::builder(crate::fl!("settings-panel-rotation-interval")).control(
                widget::text_input("20", panel_rotation)
                    .on_input(Message::SetStokeRotationInterval)
                    .width(Length::Fill),
            ),
        )
        .add(
            settings::item::builder(crate::fl!("settings-refresh-interval")).control(refresh_input),
        );

    settings::view_column(vec![panel.into(), display.into(), refresh.into()]).into()
}
