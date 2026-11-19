// SPDX-License-Identifier: GPL-3.0-only
use crate::app::Message;
use crate::components::buttons::icon_button;
use crate::components::header::{navigation, overview_subtitle, title_dynamic};
use crate::components::toolscard::SelectedAsset;
use crate::config::PopupTab::{self};
use crate::constants::{MAX_PORTFOLIOS, PAD_EXT_ROW, SPACING_ROW, TEXT_BODY};
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;

pub fn header(
    last_updated: Option<String>,
    active_tab: PopupTab,
    portfolio_count: usize,
    pinned_port_tittle: Option<String>,
) -> Element<'static, Message> {
    let refresh_message = if matches!(active_tab, PopupTab::Portfolio | PopupTab::Pinned) {
        Message::RefreshPortfolio
    } else {
        Message::RefreshMarket
    };

    let refresh_button = matches!(
        active_tab,
        PopupTab::Overview | PopupTab::News | PopupTab::Portfolio | PopupTab::Pinned
    )
    .then(|| icon_button(None, "view-refresh-symbolic", refresh_message));

    let create_port_button = (matches!(active_tab, PopupTab::Portfolio)
        && portfolio_count < MAX_PORTFOLIOS)
        .then(|| icon_button(None, "list-add-symbolic", Message::AddPortfolio));

    widget::column![
        widget::container(navigation(active_tab))
            .padding([8, PAD_EXT_ROW[1]])
            .width(Length::Fill),
        widget::row![
            widget::column![
                title_dynamic(active_tab, pinned_port_tittle),
                overview_subtitle(active_tab, last_updated),
            ]
            .width(Length::Shrink),
            widget::Space::new().width(Length::Fill),
            create_port_button,
            refresh_button,
        ]
        .spacing(SPACING_ROW)
        .padding(PAD_EXT_ROW)
        .align_y(Alignment::Center)
        .width(Length::Fill),
    ]
    .width(Length::Fill)
    .into()
}

pub fn tools_header(
    last_updated: Option<String>,
    selected_asset: Option<SelectedAsset>,
) -> Element<'static, Message> {
    let (asset_name, asset_symbol, logo_handle) = selected_asset
        .as_ref()
        .map(|asset| (asset.name.clone(), asset.symbol.clone(), asset.logo.clone()))
        .unwrap_or_default();

    let logo: Element<'static, Message> = match logo_handle {
        Some(handle) => widget::image(handle)
            .width(Length::Fixed(42.0))
            .height(Length::Fixed(42.0))
            .content_fit(cosmic::iced::ContentFit::Cover)
            .into(),

        None => widget::Space::new()
            .width(Length::Fixed(42.0))
            .height(Length::Fixed(42.0))
            .into(),
    };

    widget::column![
        widget::container(navigation(PopupTab::Tools))
            .padding([8, PAD_EXT_ROW[1]])
            .width(Length::Fill),
        widget::row![
            widget::row![
                logo,
                widget::container(
                    widget::column![
                        widget::text::heading(asset_name)
                            .size(TEXT_BODY)
                            .wrapping(cosmic::iced::widget::text::Wrapping::None),
                        overview_subtitle(PopupTab::Tools, last_updated),
                    ]
                    .spacing(2)
                )
                .width(Length::Fill)
                .clip(true),
            ]
            .spacing(SPACING_ROW)
            .align_y(Alignment::Center)
            .width(Length::FillPortion(1)),
            icon_button(None, "view-refresh-symbolic", Message::RefreshPortfolio),
            icon_button(
                None,
                "edit-delete-symbolic",
                Message::RemoveStockFromPortfolio(asset_symbol),
            ),
        ]
        .spacing(SPACING_ROW)
        .padding(PAD_EXT_ROW)
        .align_y(Alignment::Center)
        .width(Length::Fill)
    ]
    .width(Length::Fill)
    .into()
}
