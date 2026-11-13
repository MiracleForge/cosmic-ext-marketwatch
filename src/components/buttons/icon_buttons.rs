use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget,
};

use crate::{
    app::Message,
    constants::{PAD_TAB, SPACING_ROW},
};

use crate::config::RefreshInterval;
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

pub fn refresh_button<'a>(
    label: &'a str,
    value: RefreshInterval,
    current: RefreshInterval,
    message: fn(RefreshInterval) -> Message,
) -> Element<'a, Message> {
    let selected = value == current;

    let content = widget::container(widget::text(label))
        .width(Length::Fill)
        .align_x(cosmic::iced::alignment::Horizontal::Center);

    widget::button::custom(content)
        .class(if selected {
            cosmic::theme::Button::Suggested
        } else {
            cosmic::theme::Button::Standard
        })
        .width(Length::Fill)
        .padding([8, 0])
        .on_press(message(value))
        .into()
}
