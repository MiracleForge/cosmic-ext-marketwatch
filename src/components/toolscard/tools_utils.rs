use cosmic::iced::{Alignment, Length};
use cosmic::{Element, Theme};

use cosmic::widget;

// SP// SPDX-License-Identifier: GPL-3.0-only
use crate::app::Message;
use crate::components::alertcard::{condition_to_index, index_to_condition};
use crate::components::toolscard::ToolsView;
use crate::components::ui::badge_class;
use crate::constants::{PAD_CARD, TEXT_BODY};
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

pub fn alert_inputs<'a>(tools: &ToolsView<'a>, theme: &Theme) -> Element<'a, Message> {
    let input_value = tools.input_value;
    let input_for_closure = input_value.to_string();

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

    let condition_card = widget::container(
        widget::row![
            widget::text::heading(crate::fl!("alerts-condition"))
                .size(TEXT_BODY)
                .width(Length::FillPortion(1)),
            widget::Space::new().width(Length::Fill),
            condition_dropdown,
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill),
    )
    .padding(PAD_CARD)
    .width(Length::Fill)
    .class(badge_class(theme));

    let needs_value = !matches!(
        tools.selected_condition,
        AlertCondition::TurnPositive | AlertCondition::TurnNegative
    );

    let value_card = needs_value.then(|| {
        widget::container(
            widget::row![
                widget::text::heading("Value")
                    .size(TEXT_BODY)
                    .width(Length::FillPortion(1)),
                widget::Space::new().width(Length::Fill),
                widget::text_input("Ex. 14.50", input_value)
                    .on_input(Message::AlertSetValue)
                    .width(Length::FillPortion(2)),
            ]
            .align_y(Alignment::Center)
            .width(Length::Fill),
        )
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
    });

    widget::column![condition_card]
        .push_maybe(value_card)
        .spacing(1)
        .width(Length::Fill)
        .into()
}
