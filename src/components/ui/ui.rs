use cosmic::iced::{Border, Length};
use cosmic::widget::button::Catalog;
use cosmic::{Element, iced::Alignment, widget};

use cosmic::Theme;

use crate::app::Message;
use crate::components::buttons::refresh_button;
use crate::constants::{PAD_CARD, PAD_ROW, SPACING_ROW, TEXT_BODY};

use crate::config::RefreshInterval;

pub fn badge_class(theme: &Theme) -> cosmic::theme::Container<'static> {
    let btn = theme.active(false, false, &cosmic::theme::Button::Standard);

    let badge_bg = btn.background;
    let badge_text = btn.text_color;
    let radius = btn.border_radius;

    cosmic::theme::Container::custom(move |_| widget::container::Style {
        background: badge_bg,
        text_color: badge_text,
        border: Border {
            radius: radius.into(),
            ..Default::default()
        },
        ..Default::default()
    })
}
pub fn external_link_card<'a>(text: String, url: &str, theme: &Theme) -> Element<'a, Message> {
    let content = widget::row![
        widget::text::heading(text)
            .size(TEXT_BODY)
            .width(Length::Fill),
        widget::button::icon(widget::icon::from_name("link-symbolic"))
            .on_press(Message::OpenExternalLink(url.to_string()))
            .padding(PAD_ROW),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill);

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
        .into()
}

pub fn toggle_card<'a>(
    text: String,
    value: bool,
    message: fn(bool) -> Message,
    theme: &Theme,
) -> Element<'a, Message> {
    let content = widget::row![
        widget::text::heading(text)
            .size(TEXT_BODY)
            .width(Length::Fill),
        widget::toggler(value).on_toggle(message),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill);

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
        .into()
}

pub fn input_card<'a>(
    text: String,
    placeholder: &'a str,
    input: String,
    message: fn(String) -> Message,
    theme: &Theme,
) -> Element<'a, Message> {
    let content = widget::row![
        widget::text::heading(text)
            .size(TEXT_BODY)
            .width(Length::Fill),
        widget::text_input(placeholder, input)
            .on_input(message)
            .width(cosmic::iced::Length::Fixed(60.0)),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill);

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
        .into()
}
pub fn segmented_card<'a>(
    text: String,
    options: &[(&'a str, RefreshInterval)],
    selected: RefreshInterval,
    message: fn(RefreshInterval) -> Message,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut row = widget::row![].spacing(4).width(Length::Fill);

    for (label, value) in options {
        row = row.push(
            widget::container(refresh_button(label, *value, selected, message))
                .width(Length::FillPortion(1)),
        );
    }

    let content = widget::column![widget::text::heading(text).size(TEXT_BODY), row,]
        .spacing(SPACING_ROW)
        .width(Length::Fill);

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
        .into()
}

pub fn textlabel(label: impl Into<String>) -> Element<'static, Message> {
    let label: String = label.into();
    widget::container(widget::text::heading(label).size(TEXT_BODY + 1))
        .style(|theme| widget::container::Style {
            text_color: Some(cosmic::iced::Color::from(theme.cosmic().accent_color())),
            ..Default::default()
        })
        .into()
}
