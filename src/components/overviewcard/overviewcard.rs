// SPDX-License-Identifier: GPL-3.0-only
use crate::components::ui::{overview_tab, quote_card};
use crate::constants::PAD_CARD;
use crate::marketwatch::{MarketQuote, ScreensTab};
use cosmic::Theme;
use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::widget;
use std::collections::HashMap;

use crate::app::Message;

pub fn overviewcard<'a>(
    theme: &Theme,
    market_quotes: &'a [MarketQuote],
    show_logos: bool,
    current_tab: ScreensTab,
    logo_handles: &'a HashMap<String, widget::image::Handle>,
) -> Element<'a, Message> {
    let mut content = widget::column![overview_tab(current_tab),]
        .spacing(1)
        .width(Length::Fill);

    for quote in market_quotes {
        let logo_handle = logo_handles.get(&quote.symbol);

        content = content.push(quote_card(quote, show_logos, logo_handle, theme));
    }

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .into()
}
