// SPDX-License-Identifier: GPL-3.0-only
use crate::components::buttons::{icon_button, stock_search_input};
use crate::marketwatch::MarketQuote;
use cosmic::Theme;
use cosmic::prelude::*;
use cosmic::widget::{self, Space};

use crate::app::Message;

use crate::components::ui::{friendly_message, quote_card};
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
) -> Element<'a, Message> {
    let search_input = stock_search_input(crate::fl!("assets-search-by").to_string(), search_value);

    let has_selected = number_of_port > 0 && current_index > 0;

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
                .width(Length::Fixed(140.0))
                .into()
        } else {
            widget::text::heading(portfolio_name.unwrap_or("Portfolio")).into()
        }
    });

    // None = habilitado (sem flag de desabilitar)
    let delete_button = has_selected
        .then(|| icon_button(None, "user-trash-symbolic", Message::DeleteCurrentPortfolio));

    // A navegação agora só anda entre 1..=N, então as pontas ficam desabilitadas
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

    let body: Element<'a, Message> = if symbols.is_empty() {
        friendly_message(
            crate::fl!("port-not-found").to_string(),
            crate::fl!("port-warning").to_string(),
        )
        .into()
    } else {
        render_card_state(
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

                for quote in quotes.iter().filter(|q| symbols.contains(&q.symbol)) {
                    let logo_handle = logo_handles.get(&quote.symbol);
                    list = list.push(quote_card(quote, show_logos, logo_handle, theme));
                }

                list.into()
            },
        )
    };
    widget::container(
        widget::column![search_input, portfolio_bar, body,]
            .spacing(1)
            .width(Length::Fill),
    )
    .padding(PAD_CARD)
    .width(Length::Fill)
    .into()
}
