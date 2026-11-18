// SPDX-License-Identifier: GPL-3.0-only
use crate::components::buttons::{icon_button, stock_search_input};
use crate::marketwatch::MarketQuote;
use cosmic::Theme;
use cosmic::prelude::*;
use cosmic::widget::{self, Space};

use crate::app::Message;

use crate::components::ui::{friendly_message, port_quote_card, quote_card, render_search_card};
use crate::constants::PAD_CARD;
use crate::utils::{SkeletonCfg, render_card_state};
use cosmic::iced::{Alignment, Length};
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
    let symbols = portfolio.symbols;
    let logo_handles = style.logo_handles;
    let show_logos = style.show_logos;

    let has_selected = portfolio.count > 0 && portfolio.index > 0;
    let rename_mode = portfolio.rename_input.is_some();

    let search_input: Element<'a, Message> = if has_selected {
        stock_search_input(crate::fl!("assets-search-by").to_string(), search.value).into()
    } else {
        // input disable
        widget::text_input(crate::fl!("assets-search-by"), "").into()
    };

    let rename_button = has_selected.then(|| {
        let (icon, message) = if rename_mode {
            ("emblem-ok-symbolic", Message::ConfirmRenamePortfolio)
        } else {
            (
                "accessories-text-editor-symbolic",
                Message::ToggleRenameMode,
            )
        };

        icon_button(None, icon, message)
    });

    let name_space: Option<Element<'_, Message>> = has_selected.then(|| {
        if let Some(input) = portfolio.rename_input {
            widget::text_input(crate::fl!("wallet-name"), input)
                .on_input(Message::RenamePortfolio)
                .on_submit(|_| Message::ConfirmRenamePortfolio)
                .width(Length::Shrink)
                .into()
        } else {
            widget::text::heading(portfolio.name.unwrap_or("Portfolio")).into()
        }
    });

    let delete_button = has_selected.then(|| {
        icon_button(
            None,
            "edit-delete-symbolic",
            Message::DeleteCurrentPortfolio,
        )
    });

    let previous_button = icon_button(
        Some(!has_selected || portfolio.index <= 1),
        "go-previous-symbolic",
        Message::PreviusPortfolio,
    );

    let next_button = icon_button(
        Some(!has_selected || portfolio.index >= portfolio.count),
        "go-next-symbolic",
        Message::NextPortfolio,
    );

    let portfolio_bar = widget::row![
        name_space,
        rename_button,
        Space::new().width(Length::Fill),
        delete_button,
        previous_button,
        widget::text::heading(format!("{}/{}", portfolio.index, portfolio.count)),
        next_button,
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center);

    let input_card = render_search_card(
        search.value,
        search.results,
        search.loading,
        search.limit_reached,
        theme,
    );

    let body: Element<'a, Message> = match (has_selected, symbols.is_empty()) {
        // no portf
        (false, _) => friendly_message(
            crate::fl!("port-not-found").to_string(),
            crate::fl!("port-warning").to_string(),
        )
        .into(),

        // Portf without assets
        (true, true) => friendly_message(
            crate::fl!("port-no-assets").to_string(),
            crate::fl!("port-no-assets-hint").to_string(),
        )
        .into(),

        // Port with assets
        (true, false) => render_card_state(
            portfolio.quotes,
            portfolio.error,
            portfolio.loading,
            Some(SkeletonCfg {
                show_logo: show_logos,
                pulse: style.pulse,
                theme,
            }),
            |quotes| {
                quotes
                    .iter()
                    .filter(|q| symbols.contains(&q.symbol))
                    .fold(
                        widget::column![].spacing(1).width(Length::Fill),
                        |list, quote| {
                            let logo = logo_handles.get(&quote.symbol);
                            list.push(port_quote_card(quote, show_logos, logo, theme))
                        },
                    )
                    .into()
            },
        ),
    };

    widget::container(
        widget::column![search_input, portfolio_bar, input_card, body,]
            .spacing(1)
            .width(Length::Fill),
    )
    .padding(PAD_CARD)
    .width(Length::Fill)
    .into()
}
