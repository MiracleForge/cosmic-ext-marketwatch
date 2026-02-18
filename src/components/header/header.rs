use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;

use crate::app::Message;

fn icon_button(icon: &str, message: Message) -> Element<'static, Message> {
    widget::button::icon(widget::icon::from_name(icon))
        .on_press(message)
        .padding([8, 12])
        .into()
}

pub fn header() -> Element<'static, Message> {
    widget::row()
        .spacing(8)
        .padding([8, 12])
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .push(icon_button("go-previous-symbolic", Message::PreviusWallet))
        .push(icon_button("go-next-symbolic", Message::NextWallet))
        .push(widget::text::heading("Trending"))
        .push(widget::horizontal_space())
        .push(icon_button("view-refresh-symbolic", Message::RefreshMarket))
        .push(icon_button(
            "emblem-system-symbolic",
            Message::OpenConfigBUtton,
        ))
        .into()
}
