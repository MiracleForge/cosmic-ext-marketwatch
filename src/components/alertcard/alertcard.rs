#[allow(clippy::too_many_lines)]
fn render_alerts_tab<'a>(
    alerts: &'a [PriceAlert],
    selected_symbol: Option<&'a str>,
    selected_condition: &'a AlertCondition,
    input_value: &'a str,
    wallet_index: usize,
    market_quotes: &'a [MarketQuote],
    theme: &Theme,
) -> Element<'a, Message> {
    let mut col = widget::column![]
        .spacing(SPACING_TAB)
        .padding(PAD_TAB)
        .width(Length::Fill);

    // ================= HEADER =================
    col = col.push(
        widget::row![]
            .align_y(Alignment::Center)
            .spacing(SPACING_ROW)
            .push(
                widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
                    .on_press(Message::CloseAlertsTab)
                    .padding(PAD_ROW),
            )
            .push(
                widget::text(crate::fl!("alerts"))
                    .size(TEXT_LABEL)
                    .class(Text::Accent),
            ),
    );

    // ================= ASSET CARD =================
    let selected_quote =
        selected_symbol.and_then(|sym| market_quotes.iter().find(|q| q.symbol == sym));

    let asset_card = match selected_symbol {
        Some(sym) => {
            let mut content = widget::column![].spacing(SPACING_COL);
            content = content.push(category_divider());
            content = content.push(textlabel(crate::fl!("alerts-select")));
            content = content.push(widget::text::heading(sym));

            if let Some(quote) = selected_quote {
                let color = quote.variation_color(theme);
                content = content.push(
                    widget::row![]
                        .spacing(SPACING_TAB)
                        .align_y(Alignment::Center)
                        .push(
                            widget::text(quote.formatted_price())
                                .size(TEXT_LABEL)
                                .class(Text::Default),
                        )
                        .push(
                            widget::text(quote.formatted_variation())
                                .size(TEXT_NORMAL)
                                .class(Text::Color(color)),
                        ),
                );
            }

            widget::container(content)
                .padding(PAD_CARD)
                .width(Length::Fill)
        }

        None => widget::container(
            widget::text(crate::fl!("alerts-select-stock"))
                .size(TEXT_BODY)
                .class(Text::Accent),
        )
        .padding(PAD_CARD)
        .width(Length::Fill),
    };

    col = col.push(asset_card);

    // ================= ALERT FORM =================
    if selected_symbol.is_some() {
        let condition_options = vec![
            crate::fl!("alerts-condition-price-above"),
            crate::fl!("alerts-condition-price-below"),
            crate::fl!("alerts-condition-variation-above"),
            crate::fl!("alerts-condition-variation-below"),
            crate::fl!("alerts-condition-turns-positive"),
            crate::fl!("alerts-condition-turns-negative"),
        ];

        let condition_idx = Some(condition_to_index(selected_condition));
        let input_for_closure = input_value.to_string();

        let needs_value = !matches!(
            selected_condition,
            AlertCondition::TurnPositive | AlertCondition::TurnNegative
        );

        let form = widget::column![]
            .spacing(SPACING_TAB)
            .push(category_divider())
            .push(textlabel(crate::fl!("alerts-condition")))
            .push(
                widget::dropdown(condition_options, condition_idx, move |idx| {
                    Message::AlertSelectCondition(index_to_condition(idx, &input_for_closure))
                })
                .width(Length::Fill),
            )
            .push(
                widget::row![]
                    .spacing(SPACING_ROW)
                    .align_y(Alignment::Center)
                    .push_maybe(if needs_value {
                        Some(
                            widget::text_input(crate::fl!("alerts-value-placeholder"), input_value)
                                .on_input(Message::AlertSetValue)
                                .width(Length::Fill),
                        )
                    } else {
                        None
                    })
                    .push(
                        widget::button::suggested(crate::fl!("alerts-value-add")).on_press_maybe(
                            build_add_alert_message(
                                wallet_index,
                                selected_symbol,
                                selected_condition,
                                input_value,
                            ),
                        ),
                    ),
            );

        col = col.push(
            widget::container(form)
                .padding(PAD_CARD)
                .width(Length::Fill),
        );
    }

    // ================= ALERT LIST =================
    col = col.push(category_divider());
    col = col.push(textlabel(crate::fl!("alerts-self")));

    if alerts.is_empty() {
        col = col.push(
            widget::text(crate::fl!("alerts-not-found"))
                .size(TEXT_BODY)
                .class(Text::Accent),
        );
    } else {
        for alert in alerts {
            col = col
                .push(widget::container(render_alert_row(alert, wallet_index)).padding(PAD_ROW))
                .push(item_divider());
        }
    }

    col.into()
}

fn render_alert_row(alert: &PriceAlert, wallet_index: usize) -> Element<'_, Message> {
    let label = alert_condition_label(&alert.condition);
    let toggle_icon = if alert.enabled {
        "media-playback-pause-symbolic"
    } else {
        "media-playback-start-symbolic"
    };

    widget::row![]
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .spacing(SPACING_ROW)
        .push(widget::text(&alert.symbol).width(Length::FillPortion(1)))
        .push(
            widget::text(label)
                .size(TEXT_BODY)
                .width(Length::FillPortion(3)),
        )
        .push(
            widget::button::icon(widget::icon::from_name(toggle_icon))
                .on_press(Message::ToggleAlert {
                    wallet_index,
                    alert_id: alert.id,
                })
                .padding(PAD_ROW),
        )
        .push(
            widget::button::icon(widget::icon::from_name("list-remove-symbolic"))
                .on_press(Message::RemoveAlert {
                    wallet_index,
                    alert_id: alert.id,
                })
                .padding(PAD_ROW),
        )
        .into()
}

fn alert_condition_label(condition: &AlertCondition) -> String {
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

fn condition_to_index(condition: &AlertCondition) -> usize {
    match condition {
        AlertCondition::PriceAbove(_) => 0,
        AlertCondition::PriceBelow(_) => 1,
        AlertCondition::VariationAbove(_) => 2,
        AlertCondition::VariationBelow(_) => 3,
        AlertCondition::TurnPositive => 4,
        AlertCondition::TurnNegative => 5,
    }
}

fn index_to_condition(idx: usize, input_value: &str) -> AlertCondition {
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

fn build_add_alert_message(
    wallet_index: usize,
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
        wallet_index,
        symbol,
        condition: final_condition,
    })
}
