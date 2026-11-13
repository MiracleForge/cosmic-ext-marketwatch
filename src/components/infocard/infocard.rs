// SPDX-License-Identifier: GPL-3.0-only
use cosmic::Theme;
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;

use crate::app::Message;
use crate::components::ui::{badge_class, external_link_card, textlabel};
use crate::constants::{PAD_CARD, PAD_TAB, SPACING_ROW, TEXT_BODY};

pub fn infocard<'a>(theme: &Theme) -> Element<'a, Message> {
    let app_icon = widget::icon::icon(widget::icon::from_svg_bytes(include_bytes!(
        "../../../resources/icons/hicolor/scalable/apps/com.github.MiracleForge.cosmic-ext-marketwatch.svg"
    )))
    .size(72);

    let title = widget::text::title3("Marketwatch");

    let version_badge = widget::container(widget::text(crate::app::VERSION).size(TEXT_BODY))
        .padding(PAD_TAB)
        .class(badge_class(theme));

    let kofi_card = external_link_card(
        crate::fl!("info-tip-kofi").to_string(),
        "https://ko-fi.com/paulorosado",
        theme,
    );

    let license_card = external_link_card(
        crate::fl!("info-license").to_string(),
        "https://github.com/MiracleForge/cosmic-ext-marketwatch/blob/main/LICENSE",
        theme,
    );

    let repository_card = external_link_card(
        crate::fl!("info-repo").to_string(),
        "https://github.com/MiracleForge/cosmic-ext-marketwatch",
        theme,
    );

    let suport_card = external_link_card(
        crate::fl!("info-sup").to_string(),
        "https://github.com/MiracleForge/cosmic-ext-marketwatch/issues",
        theme,
    );

    widget::container(
        widget::column![
            widget::column![app_icon, title, version_badge]
                .spacing(SPACING_ROW)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            //
            widget::column![
                textlabel(crate::fl!("info-links")),
                widget::column![repository_card, suport_card]
                    .spacing(1)
                    .width(Length::Fill),
            ]
            .spacing(SPACING_ROW)
            .width(Length::Fill)
            .align_x(Alignment::Start),
            //
            widget::column![textlabel(crate::fl!("info-dev")), kofi_card,]
                .spacing(SPACING_ROW)
                .width(Length::Fill)
                .align_x(Alignment::Start),
            //
            widget::column![textlabel(crate::fl!("info-license-label")), license_card,]
                .spacing(SPACING_ROW)
                .width(Length::Fill)
                .align_x(Alignment::Start),
            //
            widget::container(widget::text::body(crate::fl!("info-app-desc")).size(TEXT_BODY))
                .padding([4, 0]),
        ]
        .spacing(SPACING_ROW)
        .width(Length::Fill),
    )
    .padding(PAD_CARD)
    .into()
}
