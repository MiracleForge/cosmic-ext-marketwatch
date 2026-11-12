use cosmic::{Element, widget};

use crate::{
    app::Message,
    components::buttons::{back_button, icon_button},
    config::PopupTab,
    constants::SPACING_ROW,
};

pub fn navigation(active_tab: PopupTab) -> Element<'static, Message> {
    if active_tab != PopupTab::Overview {
        return back_button(Message::NavigateTo(None));
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
                "payment-card-symbolic",
                Message::NavigateTo(Some(PopupTab::Wallet)),
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
