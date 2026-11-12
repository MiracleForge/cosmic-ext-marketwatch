use cosmic::{Element, iced::Alignment, widget};

use crate::app::Message;

pub fn icon_button(flag: Option<bool>, icon: &str, message: Message) -> Element<'static, Message> {
    let button = widget::button::icon(widget::icon::from_name(icon)).padding([8, 12]);

    if !flag.unwrap_or(false) {
        button.on_press(message).into()
    } else {
        button.into()
    }
}

pub fn back_button(message: Message) -> Element<'static, Message> {
    widget::button::custom(
        widget::row![
            widget::icon::from_name("go-previous-symbolic"),
            widget::text::body("Back"),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .on_press(message)
    .class(cosmic::theme::Button::Text)
    .into()
}
