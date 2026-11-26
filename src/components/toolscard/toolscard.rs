// SPDX-License-Identifier: GPL-3.0-only
use crate::components::toolscard::{SelectedAsset, alert_inputs, build_add_alert_message};
use crate::components::ui::{alert_card, asset_frame, friendly_message};
use crate::marketwatch::{AlertCondition, PriceAlert};

use cosmic::cosmic_theme::Spacing;
use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::widget::{self, settings};

use crate::app::Message;

pub struct ToolsView<'a> {
    pub selected_asset: Option<&'a SelectedAsset>,
    pub portfolio_index: usize,
    pub alerts: &'a [PriceAlert],
    pub selected_condition: &'a AlertCondition,
    pub input_value: &'a str,
    pub new_alert_enabled: bool,
}

pub fn toolscard<'a>(tools: &ToolsView<'a>) -> Element<'a, Message> {
    let selected_symbol = tools.selected_asset.map(|asset| asset.symbol.as_str());

    let spacing = cosmic::theme::spacing();

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
    .width(Length::Fill)
    .padding(cosmic::iced::Padding {
        top: spacing.space_s as f32,
        ..Default::default()
    });

    let alert_settings = settings::section()
        .title(crate::fl!("alerts"))
        .add(
            settings::item::builder(crate::fl!("alerts-select-asset"))
                .toggler(tools.new_alert_enabled, Message::AlertSetEnabled),
        )
        .add(alert_inputs(tools));

    let alert_settings = widget::column![alert_settings, add_button,].width(Length::Fill);

    let alerts_body: Element<'a, Message> = if tools.alerts.is_empty() {
        friendly_message(
            crate::fl!("alerts-empty").to_string(),
            crate::fl!("alerts-empty-hint").to_string(),
        )
    } else {
        tools
            .alerts
            .iter()
            .fold(settings::section(), |section, alert| {
                section.add(alert_card(alert, tools.portfolio_index))
            })
            .into()
    };

    let user_alerts = settings::section()
        .title(crate::fl!("alerts-self"))
        .add(alerts_body);

    let content = widget::column![
        asset_frame(tools.selected_asset),
        alert_settings,
        user_alerts,
    ]
    .width(Length::Fill);

    content.into()
}
