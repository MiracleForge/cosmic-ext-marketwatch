use cosmic::Element;
use cosmic::iced::{Alignment, Length};

use cosmic::widget::{self, settings};
// SP// SPDX-License-Identifier: GPL-3.0-only
use crate::app::Message;
use crate::components::alertcard::{condition_to_index, index_to_condition};
use crate::components::toolscard::ToolsView;
use crate::constants::TEXT_BODY;
use crate::marketwatch::{AlertCondition, PriceAlert};

#[derive(Debug, Clone)]
pub struct SelectedAsset {
    pub name: String,
    pub symbol: String,
    pub price: f64,
    pub currency: String,
    pub logo: Option<widget::image::Handle>,
    pub change: f64,
    pub change_percent: f64,
}

#[derive(Debug, Clone)]
pub struct NewAlert {
    pub symbol: String,
    pub condition: AlertCondition,
    pub enabled: bool,
}

impl PriceAlert {
    pub fn from_new(id: u64, new: NewAlert) -> Self {
        Self {
            id,
            symbol: new.symbol,
            condition: new.condition,
            triggered: false,
            enabled: new.enabled,
        }
    }
}

pub fn build_add_alert_message(
    portfolio_index: usize,
    selected_symbol: Option<&str>,
    condition: &AlertCondition,
    input_value: &str,
) -> Option<Message> {
    let symbol = selected_symbol?.to_string();

    let needs_value = !matches!(
        condition,
        AlertCondition::TurnPositive | AlertCondition::TurnNegative
    );

    let final_condition = if needs_value {
        let val = input_value.parse::<f64>().ok()?;

        match condition {
            AlertCondition::PriceAbove(_) => AlertCondition::PriceAbove(val),
            AlertCondition::PriceBelow(_) => AlertCondition::PriceBelow(val),
            AlertCondition::VariationAbove(_) => AlertCondition::VariationAbove(val),
            AlertCondition::VariationBelow(_) => AlertCondition::VariationBelow(val),
            other => other.clone(),
        }
    } else {
        condition.clone()
    };

    Some(Message::AddAlert {
        portfolio_index,
        alert: NewAlert {
            symbol,
            condition: final_condition,
            enabled: true,
        },
    })
}

pub fn alert_inputs<'a>(tools: &ToolsView<'a>) -> Element<'a, Message> {
    let input_for_closure = tools.input_value.to_string();

    let condition_options = vec![
        crate::fl!("alerts-condition-price-above"),
        crate::fl!("alerts-condition-price-below"),
        crate::fl!("alerts-condition-variation-above"),
        crate::fl!("alerts-condition-variation-below"),
        crate::fl!("alerts-condition-turns-positive"),
        crate::fl!("alerts-condition-turns-negative"),
    ];

    let condition_dropdown = widget::dropdown(
        condition_options,
        Some(condition_to_index(tools.selected_condition)),
        move |idx| Message::AlertSelectCondition(index_to_condition(idx, &input_for_closure)),
    )
    .width(Length::FillPortion(2));

    let condition_row = settings::item_row(vec![
        widget::text::heading(crate::fl!("alerts-condition"))
            .size(TEXT_BODY)
            .into(),
        widget::Space::new().width(Length::Fill).into(),
        condition_dropdown.into(),
    ])
    .align_y(Alignment::Center);

    let needs_value = !matches!(
        tools.selected_condition,
        AlertCondition::TurnPositive | AlertCondition::TurnNegative
    );

    let mut inputs = widget::column![condition_row].width(Length::Fill);

    if needs_value {
        let value_row = settings::item_row(vec![
            widget::text::heading("Value").size(TEXT_BODY).into(),
            widget::Space::new().width(Length::Fill).into(),
            widget::text_input("Ex. 14.50", tools.input_value)
                .on_input(Message::AlertSetValue)
                .width(Length::FillPortion(2))
                .into(),
        ])
        .align_y(Alignment::Center);

        inputs = inputs.push(value_row);
    }

    inputs.into()
}
