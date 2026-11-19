// SPDX-License-Identifier: GPL-3.0-only
use crate::components::alertcard::{condition_to_index, index_to_condition};
use crate::components::toolscard::{SelectedAsset, alert_inputs, build_add_alert_message};
use crate::components::ui::{
    alert_card, asset_frame, badge_class, friendly_message, textlabel, toggle_card,
};
use crate::constants::{PAD_CARD, SPACING_ROW, SPACING_TAB, TEXT_BODY};
use crate::marketwatch::{AlertCondition, PriceAlert};

use cosmic::Theme;
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;

use crate::app::Message;

pub struct ToolsView<'a> {
    pub selected_asset: Option<&'a SelectedAsset>,
    pub portfolio_index: usize,
    pub alerts: &'a [PriceAlert],
    pub selected_condition: &'a AlertCondition,
    pub input_value: &'a str,
    pub new_alert_enabled: bool,
}

pub fn toolscard<'a>(theme: &Theme, tools: &ToolsView<'a>) -> Element<'a, Message> {
    let enable_input = toggle_card(
        crate::fl!("alerts-select-asset").to_string(),
        tools.new_alert_enabled,
        Message::AlertSetEnabled,
        theme,
    );

    let selected_symbol = tools.selected_asset.map(|asset| asset.symbol.as_str());

    let add_button = widget::row![
        widget::Space::new().width(Length::Fill),
        widget::button::suggested(crate::fl!("alerts-value-add")).on_press_maybe(
            build_add_alert_message(
                tools.portfolio_index,
                selected_symbol,
                tools.selected_condition,
                tools.input_value,
            )
        ),
    ]
    .width(Length::Fill);

    let alert_col = widget::column![
        textlabel(crate::fl!("alerts")),
        widget::column![enable_input, alert_inputs(tools, theme), add_button]
            .spacing(1)
            .width(Length::Fill),
    ]
    .spacing(SPACING_ROW)
    .width(Length::Fill)
    .align_x(Alignment::Start);

    let alerts_body: Element<'a, Message> = if tools.alerts.is_empty() {
        friendly_message(
            crate::fl!("alerts-empty").to_string(),
            crate::fl!("alerts-empty-hint").to_string(),
        )
        .into()
    } else {
        tools
            .alerts
            .iter()
            .fold(
                widget::column![].spacing(1).width(Length::Fill),
                |col, alert| col.push(alert_card(alert, tools.portfolio_index, theme)),
            )
            .into()
    };

    let user_alerts = widget::column![
        textlabel(crate::fl!("alerts-self")),
        widget::scrollable(alerts_body)
            .width(Length::Fill)
            .height(Length::Fill),
    ]
    .spacing(SPACING_TAB)
    .width(Length::Fill)
    .height(Length::Fill);

    widget::container(
        widget::column![
            asset_frame(tools.selected_asset, theme),
            alert_col,
            user_alerts
        ]
        .spacing(1)
        .width(Length::Fill),
    )
    .padding(PAD_CARD)
    .width(Length::Fill)
    .into()
}
