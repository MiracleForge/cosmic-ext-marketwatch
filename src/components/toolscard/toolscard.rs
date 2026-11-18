use crate::constants::PAD_CARD;
// SPDX-License-Identifier: GPL-3.0-only
use crate::marketwatch::{AlertCondition, PriceAlert};
use cosmic::Theme;
use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::widget;

use crate::app::Message;

#[derive(Debug, Clone)]
pub struct SelectedAsset {
    pub name: String,
    pub symbol: String,
    pub price: f64,
    pub variation: f64,
    pub currency: String,
    pub logo: Option<widget::image::Handle>,
}

pub fn toolscard<'a>(
    // alerts: &'a [PriceAlert],
    selected_asset: Option<&'a SelectedAsset>,
    // selected_condition: &'a AlertCondition,
    // input_value: &'a str,
    // portfolio_index: usize,
    theme: &Theme,
) -> Element<'a, Message> {
    widget::container(widget::column![].spacing(1).width(Length::Fill))
        .padding(PAD_CARD)
        .width(Length::Fill)
        .into()
}
