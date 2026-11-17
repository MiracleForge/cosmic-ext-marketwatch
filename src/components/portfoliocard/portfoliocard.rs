// SPDX-License-Identifier: GPL-3.0-only
use crate::components::buttons::{icon_button, stock_search_input};
use crate::marketwatch::MarketQuote;
use cosmic::Theme;
use cosmic::prelude::*;
use cosmic::widget::{self, Space};

use crate::app::Message;

use crate::components::ui::{friendly_message, quote_card, render_search_card};
use crate::constants::PAD_CARD;
use crate::utils::{SkeletonCfg, render_card_state};
use cosmic::iced::{Alignment, Length};
use std::collections::HashMap;

pub fn portfoliocard<'a>(
    theme: &Theme,
    symbols: &'a [String],
    portfolio_quotes: &'a [MarketQuote],
    portfolio_name: Option<&'a str>,
    current_index: usize,
    number_of_port: usize,
    rename_mode: bool,
    rename_input: &'a str,
    error_message: Option<&'a String>,
    quotes_loading: bool,
    show_logos: bool,
    logo_handles: &'a HashMap<String, widget::image::Handle>,
    search_value: &'a str,
    pulse: f32,
    search_results: &'a [String],
    search_loading: bool,
    asset_limit_reached: bool,
) -> Element<'a, Message> {
    let has_selected = number_of_port > 0 && current_index > 0;

    let search_input: Element<'a, Message> = if has_selected {
        stock_search_input(crate::fl!("assets-search-by").to_string(), search_value).into()
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
        if rename_mode {
            widget::text_input(crate::fl!("wallet-name"), rename_input)
                .on_input(Message::RenamePortfolio)
                .on_submit(|_| Message::ConfirmRenamePortfolio)
                .width(Length::Shrink)
                .into()
        } else {
            widget::text::heading(portfolio_name.unwrap_or("Portfolio")).into()
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
        Some(!has_selected || current_index <= 1),
        "go-previous-symbolic",
        Message::PreviusPortfolio,
    );

    let next_button = icon_button(
        Some(!has_selected || current_index >= number_of_port),
        "go-next-symbolic",
        Message::NextPortfolio,
    );

    let portfolio_bar = widget::row![
        name_space,
        rename_button,
        Space::new().width(Length::Fill),
        delete_button,
        previous_button,
        widget::text::heading(format!("{current_index}/{number_of_port}")),
        next_button,
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center);

    let input_card = render_search_card(
        search_value,
        search_results,
        search_loading,
        asset_limit_reached,
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
            portfolio_quotes,
            error_message,
            quotes_loading,
            Some(SkeletonCfg {
                show_logo: show_logos,
                pulse,
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
                            list.push(quote_card(quote, show_logos, logo, theme))
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
