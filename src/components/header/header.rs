use crate::app::{MAX_WALLETS, Message};
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;

fn icon_button(icon: &str, message: Message) -> Element<'static, Message> {
    widget::button::icon(widget::icon::from_name(icon))
        .on_press(message)
        .padding([8, 12])
        .into()
}

pub fn header<'a>(
    current_index: usize,
    wallet_name: Option<&'a str>,
    rename_mode: bool,
    rename_input: &'a str,
    wallet_count: usize,
) -> Element<'a, Message> {
    let title: Element<'a, Message> = if current_index == 0 {
        widget::text::heading("Trending").into()
    } else if rename_mode {
        widget::row()
            .spacing(4)
            .align_y(Alignment::Center)
            .push(
                widget::text_input("Nome da carteira", rename_input)
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
        let name = wallet_name.unwrap_or("Carteira");
        widget::row()
            .spacing(4)
            .align_y(Alignment::Center)
            .push(widget::text::heading(name))
            .push(icon_button(
                "document-edit-symbolic",
                Message::ToggleRenameMode,
            ))
            .into()
    };

    let add_wallet_btn: Element<'_, Message> = if wallet_count >= MAX_WALLETS {
        widget::button::icon(widget::icon::from_name("list-add-symbolic"))
            .padding([8, 12])
            .into()
    } else {
        icon_button("list-add-symbolic", Message::AddWallet)
    };

    widget::row()
        .spacing(8)
        .padding([8, 12])
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .push(icon_button("go-previous-symbolic", Message::PreviusWallet))
        .push(icon_button("go-next-symbolic", Message::NextWallet))
        .push(title)
        .push(widget::horizontal_space())
        .push_maybe(if current_index > 0 {
            Some(icon_button(
                "user-trash-symbolic",
                Message::DeleteCurrentWallet,
            ))
        } else {
            None
        })
        .push(add_wallet_btn)
        .push(icon_button("view-refresh-symbolic", Message::RefreshMarket))
        .push(icon_button(
            "emblem-system-symbolic",
            Message::OpenConfigBUtton,
        ))
        .into()
}
