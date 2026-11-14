use std::collections::HashMap;

use crate::marketwatch::{AlertCondition, MarketQuote, PriceAlert};

impl AlertCondition {
    pub fn is_triggered(&self, quote: &MarketQuote) -> bool {
        const EPS: f64 = 0.001;
        match self {
            Self::PriceAbove(t) => quote.price > t + EPS,
            Self::PriceBelow(t) => quote.price < t - EPS,
            Self::VariationAbove(t) => quote.change_percent > t + EPS,
            Self::VariationBelow(t) => quote.change_percent < t - EPS,
            Self::TurnPositive => quote.change_percent > 0.0,
            Self::TurnNegative => quote.change_percent < 0.0,
        }
    }
}

pub fn notify_alert(alert: &PriceAlert, quote: &MarketQuote) {
    let body = crate::fl!(
        "alert-notification",
        HashMap::from([
            ("symbol", alert.symbol.clone()),
            ("description", alert_condition_description(&alert.condition)),
            ("price", quote.formatted_price()),
            ("change", format!("{:.2}", quote.change_percent)),
        ])
    );

    let _ = notify_rust::Notification::new()
        .summary(&format!("MarketWatch — {}", alert.symbol))
        .body(&body)
        .icon("utilities-system-monitor")
        .timeout(notify_rust::Timeout::Milliseconds(6000))
        .show();
}

pub fn alert_condition_description(condition: &AlertCondition) -> String {
    let value = |v: f64| HashMap::from([("value", format!("{v:.2}"))]);

    match condition {
        AlertCondition::PriceAbove(v) => crate::fl!("alert-description-price-above", value(*v)),
        AlertCondition::PriceBelow(v) => crate::fl!("alert-description-price-below", value(*v)),
        AlertCondition::VariationAbove(v) => {
            crate::fl!("alert-description-variation-above", value(*v))
        }
        AlertCondition::VariationBelow(v) => {
            crate::fl!("alert-description-variation-below", value(*v))
        }
        AlertCondition::TurnPositive => crate::fl!("alert-description-turned-positive"),
        AlertCondition::TurnNegative => crate::fl!("alert-description-turned-negative"),
    }
}

pub fn alert_condition_label(condition: &AlertCondition) -> String {
    let args = |v: f64| HashMap::from([("value", format!("{v:.2}"))]);

    match condition {
        AlertCondition::PriceAbove(v) => crate::fl!("alerts-label-price-above", args(*v)),
        AlertCondition::PriceBelow(v) => crate::fl!("alerts-label-price-below", args(*v)),
        AlertCondition::VariationAbove(v) => crate::fl!("alerts-label-variation-above", args(*v)),
        AlertCondition::VariationBelow(v) => crate::fl!("alerts-label-variation-below", args(*v)),
        AlertCondition::TurnPositive => crate::fl!("alerts-label-turns-positive"),
        AlertCondition::TurnNegative => crate::fl!("alerts-label-turns-negative"),
    }
}

pub fn condition_to_index(condition: &AlertCondition) -> usize {
    match condition {
        AlertCondition::PriceAbove(_) => 0,
        AlertCondition::PriceBelow(_) => 1,
        AlertCondition::VariationAbove(_) => 2,
        AlertCondition::VariationBelow(_) => 3,
        AlertCondition::TurnPositive => 4,
        AlertCondition::TurnNegative => 5,
    }
}

pub fn index_to_condition(idx: usize, input_value: &str) -> AlertCondition {
    let val = input_value.parse::<f64>().unwrap_or(0.0);
    match idx {
        0 => AlertCondition::PriceAbove(val),
        1 => AlertCondition::PriceBelow(val),
        2 => AlertCondition::VariationAbove(val),
        3 => AlertCondition::VariationBelow(val),
        4 => AlertCondition::TurnPositive,
        _ => AlertCondition::TurnNegative,
    }
}
