// SPDX-License-Identifier: GPL-3.0-only
use crate::components::buttons::stock_search_input;
use crate::marketwatch::MarketQuote;
use cosmic::Theme;
use cosmic::prelude::*;
use cosmic::widget;

use crate::app::Message;

use crate::components::ui::quote_card;
use crate::constants::PAD_CARD;
use crate::utils::{SkeletonCfg, render_card_state};
use cosmic::iced::Length;
use std::collections::HashMap;

pub fn portfoliocard<'a>(
    theme: &Theme,
    portfolio_quotes: &'a [MarketQuote],
    error_message: Option<&'a String>,
    quotes_loading: bool,
    show_logos: bool,
    logo_handles: &'a HashMap<String, widget::image::Handle>,
    search_value: &'a str,
    pulse: f32,
) -> Element<'a, Message> {
    let search_input = stock_search_input(crate::fl!("assets-search-by").to_string(), search_value);

    let body = render_card_state(
        portfolio_quotes,
        error_message,
        quotes_loading,
        Some(SkeletonCfg {
            show_logo: show_logos,
            pulse,
            theme,
        }),
        |quotes| {
            let mut list = widget::column![].spacing(1).width(Length::Fill);
            for quote in quotes {
                let logo_handle = logo_handles.get(&quote.symbol);
                list = list.push(quote_card(quote, show_logos, logo_handle, theme));
            }
            list.into()
        },
    );

    widget::container(
        widget::column![search_input, body]
            .spacing(1)
            .width(Length::Fill),
    )
    .padding(PAD_CARD)
    .width(Length::Fill)
    .into()
}
