// SPDX-License-Identifier: GPL-3.0-only
use crate::app::Message;
use crate::components::buttons::icon_button;
use crate::components::header::{navigation, overview_subtitle, title_dynamic};
use crate::config::PopupTab::{self};
use crate::constants::{MAX_PORTFOLIOS, PAD_EXT_ROW, SPACING_ROW};
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;

pub fn header(
    last_updated: Option<String>,
    active_tab: PopupTab,
    portfolio_count: usize,
) -> Element<'static, Message> {
    let refresh_button = matches!(
        active_tab,
        PopupTab::Overview | PopupTab::News | PopupTab::Portfolio
    )
    .then(|| icon_button(None, "view-refresh-symbolic", Message::RefreshMarket));

    let create_port_button = (matches!(active_tab, PopupTab::Portfolio)
        && portfolio_count < MAX_PORTFOLIOS)
        .then(|| icon_button(None, "list-add-symbolic", Message::AddPortfolio));

    widget::column![
        widget::container(navigation(active_tab))
            .padding([8, PAD_EXT_ROW[1]])
            .width(Length::Fill),
        widget::row![
            widget::column![
                title_dynamic(active_tab),
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
