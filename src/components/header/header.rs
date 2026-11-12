// SPDX-License-Identifier: GPL-3.0-only
use crate::app::Message;
use crate::components::buttons::icon_button;
use crate::components::header::{navigation, overview_subtitle, title_dynamic};
use crate::config::PopupTab::{self};
use crate::constants::{PAD_EXT_ROW, SPACING_ROW};
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;

pub fn header(last_updated: Option<String>, active_tab: PopupTab) -> Element<'static, Message> {
    widget::column![
        widget::container(navigation(active_tab))
            .padding([8, 0])
            .width(Length::Fill),
        widget::row![
            widget::column![
                title_dynamic(active_tab),
                overview_subtitle(active_tab, last_updated),
            ]
            .width(Length::Shrink),
            widget::Space::new().width(Length::Fill),
            (active_tab != PopupTab::Settings)
                .then(|| { icon_button(None, "view-refresh-symbolic", Message::RefreshMarket,) }),
        ]
        .spacing(SPACING_ROW)
        .padding(PAD_EXT_ROW)
        .align_y(Alignment::Center)
        .width(Length::Fill),
    ]
    .width(Length::Fill)
    .into()
}

// let title: Element<'a, Message> = if current_index == 0 {
//     widget::column![
//         widget::text::heading(crate::fl!("overview-title")),
//         // top_header(last_updated),
//     ]
//     .width(Length::Shrink)
//     .into()
// } else if rename_mode {
//     widget::row![]
//         .spacing(4)
//         .align_y(Alignment::Center)
//         .push(
//             widget::text_input(crate::fl!("wallet-name"), rename_input)
//                 .on_input(Message::RenameWallet)
//                 .on_submit(|_| Message::ConfirmRenameWallet)
//                 .width(Length::Fixed(140.0)),
//         )
//         .push(icon_button(
//             None,
//             "object-select-symbolic",
//             Message::ConfirmRenameWallet,
//         ))
//         .into()
// } else {
//     let name = wallet_name.unwrap_or("Wallet");
//     widget::row![]
//         .spacing(4)
//         .align_y(Alignment::Center)
//         .push(widget::text::heading(name))
//         .push(icon_button(
//             None,
//             "accessories-text-editor-symbolic",
//             Message::ToggleRenameMode,
//         ))
//         .into()
// };
