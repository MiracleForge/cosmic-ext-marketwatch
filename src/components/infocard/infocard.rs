// SPDX-License-Identifier: GPL-3.0-only
use crate::app::Message;
use cosmic::iced::{self, Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget::{self, settings};

use crate::components::ui::external_link_card;

pub fn infocard<'a>() -> Element<'a, Message> {
    let spacing = cosmic::theme::spacing();

    let app_icon = widget::icon::icon(widget::icon::from_svg_bytes(include_bytes!(
        "../../../resources/icons/hicolor/scalable/apps/com.github.MiracleForge.cosmic-ext-marketwatch.svg"
    )))
    .size(256)
    .width(Length::Fixed(128.))
    .height(Length::Fixed(128.))
    .content_fit(iced::ContentFit::Contain);

    let header_children: Vec<Element<Message>> = [
        app_icon.into(),
        widget::text::title3("MarketWatch").into(),
        widget::text::body("MiracleForge").into(),
        widget::button::standard(crate::app::VERSION)
            .apply(widget::container)
            .padding([spacing.space_xxs, 0, 0, 0])
            .into(),
    ]
    .into_iter()
    .collect();

    let header = widget::column::with_children(header_children)
        .width(Length::Fill)
        .align_x(Alignment::Center);

    let links_section = settings::section().title(crate::fl!("info-links")).extend([
        external_link_card(
            crate::fl!("info-repo"),
            "https://github.com/MiracleForge/cosmic-ext-marketwatch",
        ),
        external_link_card(
            crate::fl!("info-sup"),
            "https://github.com/MiracleForge/cosmic-ext-marketwatch/issues",
        ),
    ]);

    let developer_section =
        settings::section()
            .title(crate::fl!("info-dev"))
            .add(external_link_card(
                crate::fl!("info-tip-kofi"),
                "https://ko-fi.com/paulorosado",
            ));

    let license_section = settings::section()
        .title(crate::fl!("info-license-label"))
        .add(external_link_card(
            crate::fl!("info-license"),
            "https://github.com/MiracleForge/cosmic-ext-marketwatch/blob/main/LICENSE",
        ));

    widget::column::with_capacity(4)
        .push(header)
        .push(links_section)
        .push(developer_section)
        .push(license_section)
        .spacing(spacing.space_m)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
}
