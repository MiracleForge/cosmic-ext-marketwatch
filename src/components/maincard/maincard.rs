// SPDX-License-Identifier: GPL-3.0-only
use std::collections::HashMap;

use crate::components::ui::{overview_tab, textlabel};
use crate::constants::{
    PAD_CARD, PAD_ROW, PAD_TAB, SPACING_COL, SPACING_ROW, SPACING_TAB, TEXT_BODY, TEXT_LABEL,
    TEXT_NORMAL, VALUE_WIDTH,
};
use crate::marketwatch::{
    AlertCondition, MarketQuote, PriceAlert, ScreensTab, YahooNews, user_friendly_error_message,
};
use cosmic::Theme;
use cosmic::iced::font::Weight;
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::theme::Text;
use cosmic::widget;

use crate::app::{MAX_ASSETS_PER_WALLET, Message};
use crate::config::{Config, PopupTab};

#[allow(clippy::too_many_arguments)]
pub fn maincard<'a>(
    theme: &Theme,
    active_tab: PopupTab,
    current_wallet_index: usize,
    wallet_symbols: &'a [String],
    market_quotes: &'a [MarketQuote],
    config: &'a Config,
    error_message: Option<&'a String>,
    stock_search_input: &'a str,
    stock_search_results: &'a [String],
    stock_search_loading: bool,
    asset_limit_reached: bool,
    wallet_alerts: &'a [PriceAlert],
    alert_selected_symbol: Option<&'a str>,
    alert_selected_condition: &'a AlertCondition,
    alert_input_value: &'a str,
    current_screen_tab: ScreensTab,
    logo_handles: &'a HashMap<String, widget::image::Handle>,
) -> Element<'a, Message> {
    match active_tab {
        PopupTab::Alerts => render_alerts_tab(
            wallet_alerts,
            alert_selected_symbol,
            alert_selected_condition,
            alert_input_value,
            current_wallet_index,
            market_quotes,
            theme,
        ),
        _ => {
            if current_wallet_index == 0 {
                render_quotes(
                    market_quotes,
                    config,
                    error_message,
                    theme,
                    current_screen_tab,
                    logo_handles,
                )
            } else {
                render_wallet(
                    wallet_symbols,
                    market_quotes,
                    stock_search_input,
                    stock_search_results,
                    stock_search_loading,
                    asset_limit_reached,
                    theme,
                )
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_wallet<'a>(
    symbols: &'a [String],
    quotes: &'a [MarketQuote],
    search_input: &'a str,
    search_results: &'a [String],
    search_loading: bool,
    asset_limit_reached: bool,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut col = widget::column![]
        .spacing(SPACING_TAB)
        .width(Length::Fill)
        .padding(PAD_TAB);

    col = col.push(render_add_asset_section(
        search_input,
        search_results,
        search_loading,
        asset_limit_reached,
    ));

    col = col.push(render_portfolio_section(symbols, quotes, theme));

    col.into()
}

fn render_add_asset_section<'a>(
    search_input: &'a str,
    search_results: &'a [String],
    search_loading: bool,
    asset_limit_reached: bool,
) -> Element<'a, Message> {
    let mut col = widget::column![]
        .spacing(SPACING_TAB)
        .push(textlabel(crate::fl!("assets-add")));

    if asset_limit_reached {
        let args = HashMap::from([("max", MAX_ASSETS_PER_WALLET)]);
        return col
            .push(
                widget::text(crate::fl!("assets-limit", args))
                    .size(TEXT_BODY)
                    .class(cosmic::theme::Text::Accent),
            )
            .into();
    }

    col = col.push(
        widget::text_input(crate::fl!("assets-search-by"), search_input)
            .on_input(Message::StockSearchInput)
            .width(Length::Fill),
    );

    if search_loading {
        return col
            .push(
                widget::text(crate::fl!("assets-loading"))
                    .size(TEXT_BODY)
                    .class(cosmic::theme::Text::Accent),
            )
            .into();
    }

    if !search_results.is_empty() {
        let results_col = widget::column![]
            .spacing(2)
            .extend(search_results.iter().map(|label| {
                widget::button::standard(label.as_str())
                    .on_press(Message::AddStockToWallet(label.clone()))
                    .width(Length::Fill)
                    .into()
            }));

        return col.push(results_col).into();
    }

    if search_input.len() >= 2 {
        col = col.push(
            widget::text(crate::fl!("assets-not-found"))
                .size(TEXT_BODY)
                .class(cosmic::theme::Text::Accent),
        );
    }

    col.into()
}

fn render_portfolio_section<'a>(
    symbols: &'a [String],
    quotes: &'a [MarketQuote],
    theme: &Theme,
) -> Element<'a, Message> {
    let mut col = widget::column![].spacing(SPACING_COL);

    if symbols.is_empty() && quotes.is_empty() {
        return col
            .push(category_divider())
            .push(
                widget::text(crate::fl!("portfolio-empty"))
                    .size(TEXT_BODY)
                    .class(cosmic::theme::Text::Accent),
            )
            .into();
    }

    col = col
        .push(textlabel(crate::fl!("portfolio-your")))
        .push(category_divider());

    if quotes.is_empty() {
        for symbol in symbols {
            col = col.push(render_loading_row(symbol)).push(item_divider());
        }
    } else {
        for quote in quotes {
            col = col
                .push(render_quote_row(quote, theme))
                .push(item_divider());
        }
    }

    col.into()
}

fn render_loading_row(symbol: &str) -> Element<'_, Message> {
    widget::row![]
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .push(
            widget::container(widget::text::heading(symbol).class(Text::Default))
                .width(Length::FillPortion(2))
                .align_x(Alignment::Start),
        )
        .push(
            widget::container(widget::text(crate::fl!("render-loading")))
                .width(Length::FillPortion(2))
                .align_x(Alignment::Center),
        )
        .push(
            widget::container(
                widget::button::icon(widget::icon::from_name("list-remove-symbolic"))
                    .on_press(Message::RemoveStockFromWallet(symbol.to_string()))
                    .padding(PAD_ROW),
            )
            .width(Length::FillPortion(1))
            .align_x(Alignment::End),
        )
        .into()
}

fn render_quote_row(quote: &MarketQuote, theme: &Theme) -> Element<'static, Message> {
    let color = quote.variation_color(theme);

    widget::row![]
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .push(
            widget::container(widget::text::heading(quote.symbol.clone()).class(Text::Default))
                .width(Length::FillPortion(2))
                .align_x(Alignment::Start),
        )
        .push(
            widget::container(widget::text(quote.formatted_price()).class(Text::Color(color)))
                .width(Length::FillPortion(2))
                .align_x(Alignment::Center),
        )
        .push(
            widget::container(widget::text(quote.formatted_variation()).class(Text::Color(color)))
                .width(Length::FillPortion(1))
                .align_x(Alignment::Center),
        )
        .push(
            widget::container(
                widget::button::icon(widget::icon::from_name("alarm-symbolic"))
                    .on_press(Message::OpenAlertsTab(quote.symbol.clone()))
                    .padding(PAD_ROW),
            )
            .width(Length::FillPortion(1))
            .align_x(Alignment::End),
        )
        .push(
            widget::container(
                widget::button::icon(widget::icon::from_name("list-remove-symbolic"))
                    .on_press(Message::RemoveStockFromWallet(quote.symbol.clone()))
                    .padding(PAD_ROW),
            )
            .align_x(Alignment::End),
        )
        .into()
}

enum QuotesState<'a> {
    Loading,
    Error(&'a str),
    Ready(&'a [MarketQuote]),
}

fn derive_state<'a>(
    market_quotes: &'a [MarketQuote],
    error_message: Option<&'a String>,
) -> QuotesState<'a> {
    if let Some(err) = error_message {
        QuotesState::Error(err)
    } else if market_quotes.is_empty() {
        QuotesState::Loading
    } else {
        QuotesState::Ready(market_quotes)
    }
}

#[allow(clippy::too_many_arguments)]
fn render_quotes<'a>(
    market_quotes: &'a [MarketQuote],
    config: &'a Config,
    error_message: Option<&'a String>,
    theme: &Theme,
    current_screen_tab: ScreensTab,
    logo_handles: &'a HashMap<String, widget::image::Handle>,
) -> Element<'a, Message> {
    let content = widget::column![]
        .spacing(SPACING_TAB)
        .width(Length::Fill)
        .padding(PAD_TAB);

    match derive_state(market_quotes, error_message) {
        QuotesState::Loading => content
            .push(widget::text(crate::fl!("quotes-loading")))
            .into(),

        QuotesState::Error(err) => {
            let friendly = user_friendly_error_message(err);
            content.push(widget::text(friendly)).into()
        }

        QuotesState::Ready(quotes) => {
            let col = content;
            let col =
                render_quotes_list(col, quotes, config, theme, current_screen_tab, logo_handles);

            col.into()
        }
    }
}

fn render_quotes_list<'a>(
    mut content: widget::Column<'a, Message, cosmic::Theme>,
    market_quotes: &'a [MarketQuote],
    config: &'a Config,
    theme: &Theme,
    current_tab: ScreensTab,
    logo_handles: &'a HashMap<String, widget::image::Handle>,
) -> widget::Column<'a, Message, cosmic::Theme> {
    content = content.push(overview_tab(current_tab)).push(item_divider());

    for (i, quote) in market_quotes.iter().enumerate() {
        let color = quote.variation_color(theme);

        let logo_widget = logo_handles
            .get(&quote.symbol)
            .filter(|_| config.show_logos)
            .map(|handle| {
                widget::container(
                    widget::image(handle.clone())
                        .width(Length::Fixed(42.0))
                        .height(Length::Fixed(42.0))
                        .content_fit(cosmic::iced::ContentFit::Cover),
                )
                .width(Length::Fixed(42.0))
                .height(Length::Fixed(42.0))
                .clip(true)
                .style(|_theme: &Theme| widget::container::Style {
                    border: cosmic::iced::Border {
                        radius: 16.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
            });

        let top_row = widget::row![]
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .spacing(6)
            .push(
                widget::container(
                    widget::text::heading(&quote.name)
                        .size(14)
                        .class(Text::Default),
                )
                .width(Length::Fill)
                .align_x(Alignment::Start),
            )
            .push(item_divider_veritcal(20.0))
            .push(
                widget::container(
                    widget::text(quote.formatted_price())
                        .size(14)
                        .class(Text::Color(color))
                        .font(cosmic::iced::Font {
                            weight: Weight::Semibold,
                            ..Default::default()
                        }),
                )
                .width(Length::Fixed(VALUE_WIDTH))
                .align_x(Alignment::End),
            );

        let bottom_row = widget::row![]
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .spacing(6)
            .push(
                widget::container(widget::text(&quote.symbol).size(14).class(Text::Default))
                    .width(Length::Fill)
                    .align_x(Alignment::Start),
            )
            .push(item_divider_veritcal(20.0))
            .push(
                widget::container(
                    widget::text(quote.formatted_variation())
                        .size(12)
                        .class(Text::Color(color))
                        .font(cosmic::iced::Font {
                            weight: Weight::Semibold,
                            ..Default::default()
                        }),
                )
                .width(Length::Fixed(VALUE_WIDTH))
                .align_x(Alignment::End),
            );

        let text_column = widget::column![].spacing(2).push(top_row).push(bottom_row);

        //  monta a row corretamente (sem Option direto no push)
        let mut row = widget::row![].spacing(10).align_y(Alignment::Center);

        if let Some(logo) = logo_widget {
            row = row.push(logo);
        }

        let item = row.push(text_column);

        content = content.push(item);

        if i < market_quotes.len() - 1 {
            content = content.push(item_divider());
        }
    }

    content
}
fn category_divider<'a>() -> Element<'a, Message> {
    widget::container(widget::Space::new().width(Length::Fill))
        .width(Length::Fill)
        .height(0.5)
        .style(|theme: &cosmic::Theme| widget::container::Style {
            background: Some(cosmic::iced::Color::from(theme.cosmic().accent_color()).into()),
            ..Default::default()
        })
        .into()
}

fn item_divider<'a>() -> Element<'a, Message> {
    widget::divider::horizontal::default().into()
}

fn item_divider_veritcal<'a>(size: f32) -> Element<'a, Message> {
    widget::container(widget::divider::vertical::default())
        .height(Length::Fixed(size))
        .padding([0, 6])
        .into()
}

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
