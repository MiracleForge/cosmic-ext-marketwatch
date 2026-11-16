use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::widget;

use crate::app::Message;
use crate::constants::PADB_EXT_ROW;

pub fn stock_search_input<'a>(placeholder: String, search_input: &'a str) -> Element<'a, Message> {
    widget::text_input(placeholder, search_input)
        .on_input(Message::StockSearchInput)
        .width(Length::Fill)
        .padding(PADB_EXT_ROW)
        .into()
}
