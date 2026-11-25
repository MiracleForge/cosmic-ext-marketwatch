// SPDX-License-Identifier: GPL-3.0-only

use crate::app::Message;
use crate::components::portfoliocard::{PortfolioView, QuoteStyle};
use crate::components::ui::quote_card;
use crate::constants::PAD_CARD;
use crate::utils::{SkeletonCfg, render_card_state};

use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::widget;

pub fn pinnedcard<'a>(
    portfolio: &PortfolioView<'a>,
    style: &QuoteStyle<'a>,
) -> Element<'a, Message> {
    let body = render_card_state(
        portfolio.quotes,
        portfolio.error,
        portfolio.loading,
        Some(SkeletonCfg {
            show_logo: style.show_logos,
            pulse: style.pulse,
        }),
        |quotes| {
            quotes
                .iter()
                .filter(|q| portfolio.symbols.contains(&q.symbol))
                .fold(
                    widget::column![].spacing(1).width(Length::Fill),
                    |list, quote| {
                        let logo = style.logo_handles.get(&quote.symbol);

                        list.push(quote_card(quote, style.show_logos, logo))
                    },
                )
                .into()
        },
    );

    widget::container(
        widget::scrollable(widget::column![body].spacing(1).width(Length::Fill))
            .width(Length::Fill),
    )
    .padding(PAD_CARD)
    .width(Length::Fill)
    .into()
}
