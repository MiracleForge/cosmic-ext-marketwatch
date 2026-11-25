use cosmic::{Element, widget};

use crate::app::Message;

pub fn icon_button(flag: Option<bool>, icon: &str, message: Message) -> Element<'static, Message> {
    let button = widget::button::icon(widget::icon::from_name(icon).size(16));

    if flag.unwrap_or(false) {
        button.into()
    } else {
        button.on_press(message).into()
    }
}

pub fn back_button(label: String, on_press: Message) -> Element<'static, Message> {
    widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
        .extra_small()
        .padding(0)
        .label(label)
        .spacing(4)
        .class(widget::button::ButtonClass::Link)
        .on_press(on_press)
        .into()
}
