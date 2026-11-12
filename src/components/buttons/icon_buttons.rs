use cosmic::{Element, iced::Alignment, widget};

use crate::{
    app::Message,
    constants::{PAD_TAB, SPACING_ROW},
};
pub fn icon_button(flag: Option<bool>, icon: &str, message: Message) -> Element<'static, Message> {
    let button = widget::button::icon(widget::icon::from_name(icon)).padding(PAD_TAB);

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
        .spacing(SPACING_ROW)
        .align_y(Alignment::Center),
    )
    .on_press(message)
    .class(cosmic::theme::Button::Text)
    .into()
}
