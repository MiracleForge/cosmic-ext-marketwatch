// SPDX-License-Identifier: GPL-3.0-only
use crate::app::{MAX_WALLETS, Message};
use crate::config::PopupTab;
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;

fn icon_button(icon: &str, message: Message) -> Element<'static, Message> {
    widget::button::icon(widget::icon::from_name(icon))
        .on_press(message)
        .padding([8, 12])
        .into()
}

fn icon_button_disabled(icon: &str) -> Element<'static, Message> {
    widget::button::icon(widget::icon::from_name(icon))
        .padding([8, 12])
        .into()
}

pub fn header<'a>(
    current_index: usize,
    wallet_name: Option<&'a str>,
    rename_mode: bool,
    rename_input: &'a str,
    wallet_count: usize,
    last_updated: Option<String>,
    active_tab: PopupTab,
) -> Element<'a, Message> {
    let in_alerts = active_tab == PopupTab::Alerts;

    let title: Element<'a, Message> = if current_index == 0 {
        widget::text::heading("Market Overview").into()
    } else if rename_mode {
        widget::row()
            .spacing(4)
            .align_y(Alignment::Center)
            .push(
                widget::text_input("Wallet Name", rename_input)
                    .on_input(Message::RenameWallet)
                    .on_submit(|_| Message::ConfirmRenameWallet)
                    .width(Length::Fixed(140.0)),
            )
            .push(icon_button(
                "object-select-symbolic",
                Message::ConfirmRenameWallet,
            ))
            .into()
    } else {
        let name = wallet_name.unwrap_or("Wallet");
        widget::row()
            .spacing(4)
            .align_y(Alignment::Center)
            .push(widget::text::heading(name))
            .push(if in_alerts {
                icon_button_disabled("document-edit-symbolic")
            } else {
                icon_button("document-edit-symbolic", Message::ToggleRenameMode)
            })
            .into()
    };

    let add_wallet_btn: Element<'_, Message> = if wallet_count >= MAX_WALLETS || in_alerts {
        icon_button_disabled("list-add-symbolic")
    } else {
        icon_button("list-add-symbolic", Message::AddWallet)
    };

    widget::column()
        .width(Length::Fill)
        .push(top_header(last_updated))
        .push(
            widget::row()
                .spacing(8)
                .padding([8, 12])
                .align_y(Alignment::Center)
                .width(Length::Fill)
                .push(if in_alerts {
                    icon_button_disabled("go-previous-symbolic")
                } else {
                    icon_button("go-previous-symbolic", Message::PreviusWallet)
                })
                .push(if in_alerts {
                    icon_button_disabled("go-next-symbolic")
                } else {
                    icon_button("go-next-symbolic", Message::NextWallet)
                })
                .push(title)
                .push(widget::horizontal_space())
                .push_maybe(if current_index > 0 {
                    Some(if in_alerts {
                        icon_button_disabled("user-trash-symbolic")
                    } else {
                        icon_button("user-trash-symbolic", Message::DeleteCurrentWallet)
                    })
                } else {
                    None
                })
                .push(add_wallet_btn)
                .push(if in_alerts {
                    icon_button_disabled("view-refresh-symbolic")
                } else {
                    icon_button("view-refresh-symbolic", Message::RefreshMarket)
                })
                .push(icon_button(
                    "emblem-system-symbolic",
                    Message::OpenConfigBUtton,
                )),
        )
        .into()
}

fn top_header(last_updated: Option<String>) -> Element<'static, Message> {
    let text = last_updated
        .map(|t| format!("Updated at {t}"))
        .unwrap_or_default();
    widget::container(
        widget::text(text)
            .size(12)
            .class(cosmic::theme::Text::Default),
    )
    .padding([6, 12])
    .width(Length::Fill)
    .into()
}
