use cosmic::{Element, widget};

use crate::{
    app::Message,
    components::buttons::{back_button, icon_button},
    config::PopupTab,
    constants::SPACING_ROW,
};

pub fn navigation(
    active_tab: PopupTab,
    pinned_port_exists: Option<bool>,
) -> Element<'static, Message> {
    let back = match active_tab {
        PopupTab::Overview | PopupTab::Pinned => None,
        PopupTab::Tools => Some(Message::NavigateTo(Some(PopupTab::Portfolio))),
        _ if pinned_port_exists.unwrap_or(false) => {
            Some(Message::NavigateTo(Some(PopupTab::Pinned)))
        }
        _ => Some(Message::NavigateTo(None)),
    };

    if let Some(message) = back {
        return back_button(message);
    }

    widget::row![
        icon_button(
            None,
            "dialog-information-symbolic",
            Message::NavigateTo(Some(PopupTab::Info)),
        ),
        widget::Space::new().width(cosmic::iced::Length::Fill),
        widget::row![
            icon_button(
                None,
                "go-home-symbolic",
                Message::NavigateTo(Some(PopupTab::Overview))
            ),
            icon_button(
                None,
                "payment-card-symbolic",
                Message::NavigateTo(Some(PopupTab::Portfolio)),
            ),
            icon_button(
                None,
                "x-office-document-symbolic",
                Message::NavigateTo(Some(PopupTab::News)),
            ),
            icon_button(
                None,
                "emblem-system-symbolic",
                Message::NavigateTo(Some(PopupTab::Settings)),
            ),
        ]
        .spacing(SPACING_ROW),
    ]
    .width(cosmic::iced::Length::Fill)
    .into()
}
