// SPDX-License-Identifier: GPL-3.0-only
use cosmic::Theme;
use cosmic::iced::Alignment;
use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::widget;

use crate::app::Message;
use crate::components::ui::friendly_message;
use crate::components::ui::news_frame;
use crate::components::ui::news_tab;
use crate::components::ui::textlabel;
use crate::constants::SPACING_COL;
use crate::constants::{PAD_CARD, SPACING_ROW};

use crate::marketwatch::YahooNews;
use crate::utils::render_card_state;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewsTab {
    USMARKET,
    PORTIFOIO,
}
pub fn news_card<'a>(
    news: &'a [YahooNews],
    show_news: bool,
    error_message: Option<&'a String>,
    news_loading: bool,
    theme: &Theme,
) -> Element<'a, Message> {
    let cards: Element<'a, Message> = if !show_news {
        friendly_message(
            crate::fl!("news-disabled-title").to_string(),
            crate::fl!("news-disabled-description").to_string(),
        )
    } else {
        render_card_state(news, error_message, news_loading, None, |items| {
            widget::column![]
                .spacing(SPACING_COL)
                .width(Length::Fill)
                .extend(items.iter().map(|item| news_frame(item, theme)))
                .into()
        })
    };

    widget::container(
        widget::column![
            textlabel(crate::fl!("news-last")),
            news_tab(NewsTab::USMARKET),
            widget::scrollable(cards).width(Length::Shrink),
        ]
        .spacing(SPACING_ROW)
        .width(Length::Fill)
        .align_x(Alignment::Start),
    )
    .padding(PAD_CARD)
    .into()
}
