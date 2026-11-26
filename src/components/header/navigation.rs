use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget,
};

use crate::{app::Message, components::buttons::back_button, config::PopupTab};

pub fn nav_icon(name: &'static str, message: Message, selected: bool) -> Element<'static, Message> {
    widget::button::icon(widget::icon::from_name(name))
        .on_press(message)
        .selected(selected)
        .into()
}

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
        return back_button(crate::fl!("back-button"), message);
    }

    let spacing = cosmic::theme::spacing();

    let tabs = widget::row::with_capacity(4)
        .push(nav_icon(
            "go-home-symbolic",
            Message::NavigateTo(Some(PopupTab::Overview)),
            active_tab == PopupTab::Overview,
        ))
        .push(nav_icon(
            "payment-card-symbolic",
            Message::NavigateTo(Some(PopupTab::Portfolio)),
            active_tab == PopupTab::Pinned,
        ))
        .push(nav_icon(
            "x-office-document-symbolic",
            Message::NavigateTo(Some(PopupTab::News)),
            false,
        ))
        .push(nav_icon(
            "emblem-system-symbolic",
            Message::NavigateTo(Some(PopupTab::Settings)),
            false,
        ))
        .spacing(spacing.space_xxs);

    widget::row::with_capacity(3)
        .push(nav_icon(
            "dialog-information-symbolic",
            Message::NavigateTo(Some(PopupTab::Info)),
            false,
        ))
        .push(widget::Space::new().width(Length::Fill))
        .push(tabs)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .into()
}
