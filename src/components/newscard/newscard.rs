// SPDX-License-Identifier: GPL-3.0-only
use cosmic::Theme;
use cosmic::iced::Alignment;
use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::theme::Text;
use cosmic::widget;

use crate::app::Message;
use crate::components::ui::news_frame;
use crate::components::ui::news_tab;
use crate::components::ui::textlabel;
use crate::config::Config;
use crate::constants::SPACING_COL;
use crate::constants::TEXT_BODY;
use crate::constants::TEXT_SMALL;
use crate::constants::{PAD_CARD, SPACING_ROW};

use crate::marketwatch::YahooNews;
pub const NEWS_PREVIEW_COUNT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewsTab {
    USMARKET,
    PORTIFOIO,
}
pub fn news_card<'a>(news: &'a [YahooNews], theme: &Theme) -> Element<'a, Message> {
    let cards: Element<'a, Message> = if news.len() > 0 {
        widget::column![]
            .spacing(SPACING_COL)
            .width(Length::Fill)
            .extend(news.iter().map(|item| news_frame(item, theme)))
            .into()
    } else {
        widget::container(
            widget::column![
                widget::icon::from_name("x-office-news-symbolic").size(32),
                widget::text::heading(crate::fl!("news-disabled-title"))
                    .size(TEXT_BODY)
                    .align_x(Alignment::Center),
                widget::text::body(crate::fl!("news-disabled-description"))
                    .size(TEXT_SMALL)
                    .align_x(Alignment::Center)
                    .class(Text::default())
            ]
            .spacing(8)
            .align_x(Alignment::Center)
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .padding([32, 16])
        .center_y(Length::Fill)
        .into()
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
