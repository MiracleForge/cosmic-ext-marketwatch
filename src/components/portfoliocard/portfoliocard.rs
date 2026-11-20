// SPDX-License-Identifier: GPL-3.0-only
use crate::components::buttons::stock_search_input;
use crate::components::portfoliocard::{portfolio_bar, portfolio_body};
use crate::marketwatch::MarketQuote;
use cosmic::Theme;
use cosmic::prelude::*;
use cosmic::widget::{self};

use crate::app::Message;

use crate::components::ui::render_search_card;
use crate::constants::PAD_CARD;
use cosmic::iced::Length;
use std::collections::HashMap;
pub struct PortfolioView<'a> {
    pub name: Option<&'a str>,
    pub symbols: &'a [String],
    pub quotes: &'a [MarketQuote],
    pub index: usize,
    pub count: usize,
    pub rename_input: Option<&'a str>,
    pub error: Option<&'a String>,
    pub loading: bool,
    pub pinned: usize,
}

pub struct SearchView<'a> {
    pub value: &'a str,
    pub results: &'a [String],
    pub loading: bool,
    pub limit_reached: bool,
}

pub struct QuoteStyle<'a> {
    pub show_logos: bool,
    pub logo_handles: &'a HashMap<String, widget::image::Handle>,
    pub pulse: f32,
}

pub fn portfoliocard<'a>(
    theme: &Theme,
    portfolio: &PortfolioView<'a>,
    search: &SearchView<'a>,
    style: &QuoteStyle<'a>,
) -> Element<'a, Message> {
    let has_selected = portfolio.count > 0 && portfolio.index > 0;

    let search_input: Element<'a, Message> = if has_selected {
        stock_search_input(crate::fl!("assets-search-by").to_string(), search.value)
    } else {
        widget::text_input(crate::fl!("assets-search-by"), "").into() // input disabled
    };

    widget::container(
        widget::scrollable(
            widget::column![
                search_input,
                portfolio_bar(theme, portfolio),
                render_search_card(
                    search.value,
                    search.results,
                    search.loading,
                    search.limit_reached,
                    theme,
                ),
                portfolio_body(theme, portfolio, style),
            ]
            .spacing(1)
            .width(Length::Fill),
        )
        .width(Length::Fill),
    )
    .padding(PAD_CARD)
    .width(Length::Fill)
    .into()
}
