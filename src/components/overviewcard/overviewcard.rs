// SPDX-License-Identifier: GPL-3.0-only
use crate::components::portfoliocard::QuoteStyle;
use crate::components::ui::{overview_tab, quote_card};
use crate::marketwatch::MarketQuote;
use crate::utils::{SkeletonCfg, render_card_state};

use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget::settings;
use cosmic::widget::{self, segmented_button};

use crate::app::Message;

pub fn overviewcard<'a>(
    market_quotes: &'a [MarketQuote],
    error_message: Option<&'a String>,
    quotes_loading: bool,
    tabs_model: &'a segmented_button::SingleSelectModel,
    style: &QuoteStyle<'a>,
) -> Element<'a, Message> {
    let body = render_card_state(
        market_quotes,
        error_message,
        quotes_loading,
        Some(SkeletonCfg {
            show_logo: style.show_logos,
            pulse: style.pulse,
        }),
        |quotes| {
            quotes
                .iter()
                .fold(settings::section(), |section, quote| {
                    section.add(quote_card(
                        quote,
                        style.show_logos,
                        style.logo_handles.get(&quote.symbol),
                    ))
                })
                .into()
        },
    );

    widget::container(
        widget::column![
            overview_tab(tabs_model),
            widget::scrollable(body).height(Length::Fill)
        ]
        .width(Length::Fill),
    )
    .into()
}

fn skeleton_bar<'a>(width: f32, height: f32, pulse: f32) -> Element<'a, Message> {
    let alpha = 0.10 + 0.18 * pulse;

    widget::container(widget::Space::new())
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .style(move |theme: &cosmic::Theme| {
            let mut c = cosmic::iced::Color::from(theme.cosmic().on_bg_color());
            c.a = alpha;
            widget::container::Style {
                background: Some(c.into()),
                border: cosmic::iced::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
}

pub fn quote_card_skeleton<'a>(show_logo: bool, pulse: f32) -> Element<'a, Message> {
    let mut items: Vec<Element<'a, Message>> = Vec::new();

    if show_logo {
        items.push(skeleton_bar(42.0, 42.0, pulse));
    }

    items.push(
        widget::column![
            skeleton_bar(120.0, 14.0, pulse),
            skeleton_bar(60.0, 12.0, pulse),
        ]
        .spacing(6)
        .width(Length::Fill)
        .into(),
    );

    items.push(
        widget::column![
            skeleton_bar(64.0, 14.0, pulse),
            skeleton_bar(48.0, 12.0, pulse),
        ]
        .spacing(6)
        .align_x(Alignment::End)
        .into(),
    );

    settings::item_row(items).align_y(Alignment::Center).into()
}

pub fn quotes_skeleton<'a>(show_logo: bool, pulse: f32) -> Element<'a, Message> {
    (0..6)
        .fold(settings::section(), |section, _| {
            section.add(quote_card_skeleton(show_logo, pulse))
        })
        .into()
}
