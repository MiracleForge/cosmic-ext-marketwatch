// SPDX-License-Identifier: GPL-3.0-only
use cosmic::Theme;
use cosmic::iced::Alignment;
use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::theme::Text;
use cosmic::widget;

use crate::app::Message;
use crate::components::ui::friendly_message;
use crate::components::ui::news_frame;
use crate::components::ui::news_tab;
use crate::components::ui::textlabel;
use crate::constants::SPACING_COL;
use crate::constants::{PAD_CARD, SPACING_ROW};

use crate::marketwatch::YahooNews;
pub const NEWS_PREVIEW_COUNT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewsTab {
    USMARKET,
    PORTIFOIO,
}
pub fn news_card<'a>(
    news: &'a [YahooNews],
    show_news: bool,
    theme: &Theme,
) -> Element<'a, Message> {
    let cards: Element<'a, Message> = if show_news {
        widget::column![]
            .spacing(SPACING_COL)
            .width(Length::Fill)
            .extend(news.iter().map(|item| news_frame(item, theme)))
            .into()
    } else {
        friendly_message(
            crate::fl!("news-disabled-title").to_string(),
            crate::fl!("news-disabled-description").to_string(),
        )
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
