// SPDX-License-Identifier: GPL-3.0-only
use crate::components::ui::{badge_class, overview_tab, quote_card};
use crate::constants::{PAD_CARD, SPACING_ROW};
use crate::marketwatch::{MarketQuote, ScreensTab};
use crate::utils::{SkeletonCfg, render_card_state};
use cosmic::Theme;
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;
use std::collections::HashMap;

use crate::app::Message;

pub fn overviewcard<'a>(
    theme: &Theme,
    market_quotes: &'a [MarketQuote],
    error_message: Option<&'a String>,
    quotes_loading: bool,
    show_logos: bool,
    current_tab: ScreensTab,
    logo_handles: &'a HashMap<String, widget::image::Handle>,
    pulse: f32,
) -> Element<'a, Message> {
    let body = render_card_state(
        market_quotes,
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
        widget::column![overview_tab(current_tab), body]
            .spacing(1)
            .width(Length::Fill),
    )
    .padding(PAD_CARD)
    .width(Length::Fill)
    .into()
}

fn skeleton_bar<'a>(width: f32, height: f32, pulse: f32) -> Element<'a, Message> {
    // pulse vai de 0.0 a 1.0 -> opacidade entre 0.10 e 0.28
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

pub fn quote_card_skeleton<'a>(show_logo: bool, pulse: f32, theme: &Theme) -> Element<'a, Message> {
    let mut content = widget::row![]
        .spacing(SPACING_ROW)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    if show_logo {
        content = content.push(skeleton_bar(42.0, 42.0, pulse));
    }

    // nome + símbolo
    content = content.push(
        widget::column![
            skeleton_bar(120.0, 14.0, pulse),
            skeleton_bar(60.0, 12.0, pulse),
        ]
        .width(Length::Fill)
        .spacing(6),
    );

    // preço + variação
    content = content.push(
        widget::column![
            skeleton_bar(64.0, 14.0, pulse),
            skeleton_bar(48.0, 12.0, pulse),
        ]
        .align_x(Alignment::End)
        .spacing(6),
    );

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
        .into()
}

pub fn quotes_skeleton<'a>(show_logo: bool, pulse: f32, theme: &Theme) -> Element<'a, Message> {
    let mut list = widget::column![].spacing(1).width(Length::Fill);
    for _ in 0..6 {
        list = list.push(quote_card_skeleton(show_logo, pulse, theme));
    }
    list.into()
}
