use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget,
};

use crate::{
    app::Message,
    constants::{PAD_TAB, SPACING_ROW, TEXT_BODY},
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

pub fn segmented_button<'a>(
    label: impl Into<String>,
    msg: Message,
    selected: bool,
) -> Element<'a, Message> {
    let label: String = label.into();
    let text = widget::text::caption_heading(label);

    // linha: 2px quando ativo, 1px quando inativo
    let line = widget::container(widget::Space::new())
        .height(if selected { 2.0 } else { 1.5 })
        .width(Length::Fill)
        .style(move |theme: &cosmic::Theme| {
            let color = if selected {
                cosmic::iced::Color::from(theme.cosmic().accent_color())
            } else {
                let mut c = cosmic::iced::Color::from(theme.cosmic().on_bg_color());
                c.a = 0.9;
                c
            };
            widget::container::Style {
                background: Some(color.into()),
                ..Default::default()
            }
        });

    let underline = widget::container(line)
        .height(Length::Fixed(2.0))
        .width(Length::Fill)
        .align_y(cosmic::iced::alignment::Vertical::Bottom);

    let content = widget::column(vec![text.into(), underline.into()])
        .align_x(cosmic::iced::alignment::Horizontal::Center)
        .spacing(4);

    widget::button::custom(content)
        .width(Length::FillPortion(1))
        .padding([8, 0])
        .class(if selected {
            cosmic::theme::Button::Standard
        } else {
            cosmic::theme::Button::Text
        })
        .on_press(msg)
        .into()
}
