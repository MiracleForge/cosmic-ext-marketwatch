// SPDX-License-Identifier: GPL-3.0-only
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::theme::Text;
use cosmic::widget::{self, segmented_button, settings};

use crate::app::Message;
use crate::components::ui::news_tab;
use crate::components::ui::textlabel;
use crate::components::ui::{friendly_message, svg_accent};
use crate::constants::{TEXT_BODY, TEXT_SMALL};
use crate::marketwatch::{YahooNews, format_publish_time};
use crate::utils::render_card_state;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewsTab {
    Usmarket,
    Portfolio,
}

pub fn news_card<'a>(
    news: &'a [YahooNews],
    show_news: bool,
    error_message: Option<&'a String>,
    news_loading: bool,
    news_tabs_model: &'a segmented_button::SingleSelectModel,
) -> Element<'a, Message> {
    let cards: Element<'a, Message> = if !show_news {
        friendly_message(
            crate::fl!("news-disabled-title").to_string(),
            crate::fl!("news-disabled-description").to_string(),
        )
    } else if news.is_empty() && !news_loading && error_message.is_none() {
        friendly_message(
            crate::fl!("news-empty-title").to_string(),
            crate::fl!("news-empty-description").to_string(),
        )
    } else {
        render_card_state(news, error_message, news_loading, None, |items| {
            let section = items
                .iter()
                .map(news_frame)
                .fold(settings::section(), |section, item| section.add(item));

            section.into()
        })
    };

    widget::container(
        widget::column![news_tab(news_tabs_model), cards,]
            .width(Length::Fill)
            .align_x(Alignment::Start),
    )
    .into()
}

fn news_frame<'a>(news: &'a YahooNews) -> widget::list::ListButton<'a, Message> {
    let publisher = news.publisher.as_deref().unwrap_or("Unknown source");

    let time_str = news
        .publish_time
        .map(format_publish_time)
        .unwrap_or_default();

    let meta_text = if time_str.is_empty() {
        publisher.to_string()
    } else {
        format!("{publisher} · {time_str}")
    };

    let content = widget::row::with_capacity(2)
        .push(
            widget::column![
                widget::text::heading(&news.title).size(TEXT_BODY),
                widget::text(meta_text).size(TEXT_SMALL).class(Text::Accent),
            ]
            .spacing(4)
            .width(Length::Fill),
        )
        .push(
            widget::icon::from_name("link-symbolic")
                .icon()
                .class(svg_accent()),
        )
        .align_y(Alignment::Center);

    settings::item_row(vec![content.into()])
        .align_y(Alignment::Center)
        .apply(widget::list::button)
        .on_press(Message::OpenExternalLink(news.link.clone()))
}
