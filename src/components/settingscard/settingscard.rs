// SPDX-License-Identifier: GPL-3.0-only
use cosmic::Theme;
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;

use crate::app::Message;
use crate::components::ui::{input_card, segmented_card, textlabel, toggle_card};
use crate::constants::{PAD_CARD, SPACING_ROW};

use crate::config::{Config, RefreshInterval};

pub fn settingscard<'a>(config: &'a Config, theme: &Theme) -> Element<'a, Message> {
    let news_count = config.count_news_by_simbol.to_string();

    let toggle_show_icon = toggle_card(
        crate::fl!("settings-show-icon-only").to_string(),
        config.show_only_icon,
        Message::ToggleShowOnlyIcon,
        theme,
    );

    let toggle_show_logo = toggle_card(
        crate::fl!("settings-show-logos").to_string(),
        config.show_logos,
        Message::ToggleShowLogos,
        theme,
    );

    let toggle_alert_enable = toggle_card(
        crate::fl!("settings-disable-custom-alerts").to_string(),
        config.alerts_enabled,
        Message::ToggleAlertsEnabled,
        theme,
    );

    let toggle_show_news = toggle_card(
        crate::fl!("settings-display-news").to_string(),
        config.show_news,
        Message::ToggleShowNews,
        theme,
    );

    let input_news = input_card(
        crate::fl!("settings-news-per-asset").to_string(),
        "1",
        news_count,
        Message::SetNumberOfNewsBySymbols,
        theme,
    );

    let panel_rotation_t = input_card(
        crate::fl!("settings-panel-rotation-interval").to_string(),
        "20",
        config.panel_stoke_rotation_interval.to_string(),
        Message::SetStokeRotationInterval,
        theme,
    );

    let intervals = [
        ("5 min", RefreshInterval::FiveMinutes),
        ("10 min", RefreshInterval::TenMinutes),
        ("15 min", RefreshInterval::FifteenMinutes),
        ("30 min", RefreshInterval::ThirtyMinutes),
        ("1h", RefreshInterval::OneHour),
    ];

    let refresh = segmented_card(
        crate::fl!("settings-refresh-interval").to_string(),
        &intervals,
        config.refresh_interval,
        Message::SetRefreshInterval,
        theme,
    );

    widget::container(
        widget::column![
            widget::column![
                textlabel(crate::fl!("settings-panel")),
                widget::column![toggle_show_icon, toggle_alert_enable]
                    .spacing(1)
                    .width(Length::Fill),
            ]
            .spacing(SPACING_ROW)
            .width(Length::Fill)
            .align_x(Alignment::Start),
            widget::column![
                textlabel(crate::fl!("settings-display")),
                widget::column![toggle_show_logo, toggle_show_news, input_news]
                    .spacing(1)
                    .width(Length::Fill),
            ]
            .spacing(SPACING_ROW)
            .width(Length::Fill)
            .align_x(Alignment::Start),
            widget::column![
                textlabel(crate::fl!("settings-refresh")),
                widget::column![panel_rotation_t, refresh]
                    .spacing(1)
                    .width(Length::Fill),
            ]
            .spacing(SPACING_ROW)
            .width(Length::Fill)
            .align_x(Alignment::Start),
        ]
        .spacing(SPACING_ROW)
        .width(Length::Fill),
    )
    .padding(PAD_CARD)
    .into()
}
