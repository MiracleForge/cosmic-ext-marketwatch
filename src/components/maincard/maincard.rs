// SPDX-License-Identifier: GPL-3.0-only
use crate::marketwatch::{
    AlertCondition, MarketQuote, PriceAlert, ScreensTab, YahooNews, format_publish_time,
    user_friendly_error_message,
};
use cosmic::Theme;
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::theme::Text;
use cosmic::widget;

use crate::app::{MAX_ASSETS_PER_WALLET, Message};
use crate::config::{Config, PopupTab, RefreshInterval};

// =====================================================================
// Layout constants — change here to affect the entire UI
// =====================================================================
const PAD_TAB: [u16; 2] = [8, 12]; // outer padding for all tabs
const PAD_CARD: [u16; 2] = [10, 12]; // inner padding for cards
const PAD_ROW: [u16; 2] = [4, 8]; // padding for icon buttons in rows
const SPACING_TAB: u16 = 12; // spacing between sections within a tab
const SPACING_ROW: u16 = 8; // spacing between items in a row
const SPACING_COL: u16 = 6; // spacing between items in a column
const TEXT_SMALL: u16 = 11; // labels, badges
const TEXT_BODY: u16 = 12; // secondary body text
const TEXT_NORMAL: u16 = 13; // primary body text
const TEXT_LABEL: u16 = 14; // emphasis labels
const NEWS_PREVIEW_COUNT: usize = 3;
const HARD_CODED_WIDTH: f32 = 300.0;

#[allow(clippy::too_many_arguments)]
pub fn maincard<'a>(
    theme: &Theme,
    active_tab: PopupTab,
    current_wallet_index: usize,
    wallet_symbols: &'a [String],
    market_quotes: &'a [MarketQuote],
    news_items: &'a [YahooNews],
    news_expanded: bool,
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
    news_input: &'a str,
    current_screen_tab: ScreensTab,
) -> Element<'a, Message> {
    match active_tab {
        PopupTab::Settings => render_settings_tab(config, news_input),
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
                    news_items,
                    news_expanded,
                    config,
                    error_message,
                    theme,
                    current_screen_tab,
                )
            } else {
                render_wallet(
                    wallet_symbols,
                    market_quotes,
                    news_items,
                    news_expanded,
                    config,
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
    news_items: &'a [YahooNews],
    news_expanded: bool,
    config: &'a Config,
    search_input: &'a str,
    search_results: &'a [String],
    search_loading: bool,
    asset_limit_reached: bool,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut col = widget::column()
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

    if config.show_news {
        col = col.push(render_news_section(news_items, news_expanded));
    }

    col.into()
}

fn render_add_asset_section<'a>(
    search_input: &'a str,
    search_results: &'a [String],
    search_loading: bool,
    asset_limit_reached: bool,
) -> Element<'a, Message> {
    let mut col = widget::column()
        .spacing(SPACING_TAB)
        .push(category_header("ADD ASSET"));

    if asset_limit_reached {
        return col
            .push(
                widget::text(format!(
                    "Asset limit reached ({MAX_ASSETS_PER_WALLET} max)."
                ))
                .size(TEXT_BODY)
                .class(cosmic::theme::Text::Accent),
            )
            .into();
    }

    col = col.push(
        widget::text_input("Search by symbol (e.g. AAPL, PETR4)", search_input)
            .on_input(Message::StockSearchInput)
            .width(Length::Fill),
    );

    if search_loading {
        return col
            .push(
                widget::text("Searching...")
                    .size(TEXT_BODY)
                    .class(cosmic::theme::Text::Accent),
            )
            .into();
    }

    if !search_results.is_empty() {
        let results_col = widget::column()
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
            widget::text("No results found.")
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
    let mut col = widget::column().spacing(SPACING_COL);

    if symbols.is_empty() && quotes.is_empty() {
        return col
            .push(category_divider())
            .push(
                widget::text("Your portfolio is empty.")
                    .size(TEXT_BODY)
                    .class(cosmic::theme::Text::Accent),
            )
            .into();
    }

    col = col
        .push(category_header("Your Portfolio"))
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
    widget::row()
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .push(
            widget::container(widget::text::heading(symbol).class(Text::Default))
                .width(Length::FillPortion(2))
                .align_x(Alignment::Start),
        )
        .push(
            widget::container(widget::text("Loading..."))
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

    widget::row()
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

fn render_quotes<'a>(
    market_quotes: &'a [MarketQuote],
    news_items: &'a [YahooNews],
    news_expanded: bool,
    config: &'a Config,
    error_message: Option<&'a String>,
    theme: &Theme,
    current_screen_tab: ScreensTab,
) -> Element<'a, Message> {
    let content = widget::column()
        .spacing(SPACING_TAB)
        .width(Length::Fill)
        .padding(PAD_TAB);

    match derive_state(market_quotes, error_message) {
        QuotesState::Loading => content.push(widget::text("Loading market data...")).into(),

        QuotesState::Error(err) => {
            let friendly = user_friendly_error_message(err);
            content.push(widget::text(friendly)).into()
        }

        QuotesState::Ready(quotes) => {
            let col = content;
            let col = render_quotes_list(col, quotes, theme, current_screen_tab);

            if config.show_news {
                col.push(render_news_section(news_items, news_expanded))
                    .into()
            } else {
                col.into()
            }
        }
    }
}

fn render_news_section<'a>(news: &'a [YahooNews], expanded: bool) -> Element<'a, Message> {
    let has_more = news.len() > NEWS_PREVIEW_COUNT;

    let header_row = widget::row()
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .push(
            widget::text("Latest News")
                .size(TEXT_BODY)
                .class(Text::Accent),
        )
        .push(widget::horizontal_space())
        .push_maybe(if has_more {
            Some(
                widget::button::standard(if expanded {
                    "Collapse ▲"
                } else {
                    "View all ▼"
                })
                .on_press(Message::ToggleNewsExpanded),
            )
        } else {
            None
        });

    let news_content: Element<'a, Message> = if news.is_empty() {
        widget::text("No news available at the moment.")
            .size(TEXT_BODY)
            .class(Text::Accent)
            .into()
    } else {
        let visible = if expanded {
            news
        } else {
            &news[..NEWS_PREVIEW_COUNT.min(news.len())]
        };

        let cards = widget::column()
            .spacing(SPACING_COL)
            .width(Length::Fill)
            .extend(visible.iter().map(news_card));

        if expanded {
            widget::scrollable(cards)
                .height(Length::Fixed(HARD_CODED_WIDTH))
                .into()
        } else {
            cards.into()
        }
    };

    widget::column()
        .spacing(SPACING_ROW)
        .width(Length::Fill)
        .push(category_divider())
        .push(header_row)
        .push(news_content)
        .into()
}

fn news_card(item: &YahooNews) -> Element<'_, Message> {
    let publisher = item.publisher.as_deref().unwrap_or("Unknown source");

    let time_str = item
        .publish_time
        .map(format_publish_time)
        .unwrap_or_default();

    let meta_text = if time_str.is_empty() {
        publisher.to_string()
    } else {
        format!("{publisher} · {time_str}")
    };

    let content = widget::column()
        .spacing(4)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .push(widget::text(&item.title).size(TEXT_NORMAL))
        .push(widget::text(meta_text).size(TEXT_SMALL).class(Text::Accent));

    widget::button::custom(content)
        .class(cosmic::theme::Button::MenuItem)
        .width(Length::Fill)
        .on_press(Message::OpenNewsLink(item.link.clone()))
        .into()
}

fn screens_tab(current: ScreensTab) -> Element<'static, Message> {
    let content = widget::row()
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Shrink)
        .push(tab_button(
            "Most Active",
            Message::SetTab(ScreensTab::MostActive),
            current == ScreensTab::MostActive,
        ))
        .push(tab_button(
            "Gainers",
            Message::SetTab(ScreensTab::Gainers),
            current == ScreensTab::Gainers,
        ))
        .push(tab_button(
            "Losers",
            Message::SetTab(ScreensTab::Losers),
            current == ScreensTab::Losers,
        ));

    widget::container(content).padding(10).into()
}

fn render_quotes_list<'a>(
    mut content: widget::Column<'a, Message>,
    market_quotes: &'a [MarketQuote],
    theme: &Theme,
    current_tab: ScreensTab,
) -> widget::Column<'a, Message> {
    content = content.push(screens_tab(current_tab));
    for quote in market_quotes {
        let color = quote.variation_color(theme);

        let row = widget::row()
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .push(
                widget::container(widget::text::heading(&quote.symbol).class(Text::Default))
                    .width(Length::FillPortion(2))
                    .align_x(Alignment::Start),
            )
            .push(
                widget::container(widget::text(quote.formatted_price()).class(Text::Color(color)))
                    .width(Length::FillPortion(2))
                    .align_x(Alignment::Center),
            )
            .push(
                widget::container(
                    widget::text(quote.formatted_variation()).class(Text::Color(color)),
                )
                .width(Length::FillPortion(1))
                .align_x(Alignment::End),
            );

        content = content.push(row).push(item_divider());
    }

    content
}

fn category_divider<'a>() -> Element<'a, Message> {
    widget::container(widget::horizontal_space())
        .width(Length::Fill)
        .height(1)
        .style(|theme: &cosmic::Theme| widget::container::Style {
            background: Some(cosmic::iced::Color::from(theme.cosmic().accent_color()).into()),
            ..Default::default()
        })
        .into()
}

fn item_divider<'a>() -> Element<'a, Message> {
    widget::container(widget::horizontal_space())
        .width(Length::Fill)
        .height(1)
        .style(|theme: &cosmic::Theme| widget::container::Style {
            background: Some(cosmic::iced::Color::from(theme.cosmic().palette.neutral_5).into()),
            ..Default::default()
        })
        .into()
}

fn category_header(label: &str) -> Element<'_, Message> {
    widget::container(widget::text::heading(label).size(TEXT_BODY + 1))
        .style(|theme| widget::container::Style {
            text_color: Some(cosmic::iced::Color::from(theme.cosmic().accent_color())),
            ..Default::default()
        })
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
    let mut col = widget::column()
        .spacing(SPACING_TAB)
        .padding(PAD_TAB)
        .width(Length::Fill);

    // ================= HEADER =================
    col = col.push(
        widget::row()
            .align_y(Alignment::Center)
            .spacing(SPACING_ROW)
            .push(
                widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
                    .on_press(Message::CloseAlertsTab)
                    .padding(PAD_ROW),
            )
            .push(widget::text("Alerts").size(TEXT_LABEL).class(Text::Accent)),
    );

    // ================= ASSET CARD =================
    let selected_quote =
        selected_symbol.and_then(|sym| market_quotes.iter().find(|q| q.symbol == sym));

    let asset_card = match selected_symbol {
        Some(sym) => {
            let mut content = widget::column().spacing(SPACING_COL);
            content = content.push(category_header("Selected Asset"));
            content = content.push(widget::text::heading(sym));

            if let Some(quote) = selected_quote {
                let color = quote.variation_color(theme);
                content = content.push(
                    widget::row()
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
            widget::text("Select a stock from the list to create alerts.")
                .size(TEXT_BODY)
                .class(Text::Accent),
        )
        .padding(PAD_CARD)
        .width(Length::Fill),
    };

    col = col.push(asset_card);

    // ================= ALERT FORM =================
    if selected_symbol.is_some() {
        let condition_options = &[
            "Price Above",
            "Price Below",
            "Variation Above",
            "Variation Below",
            "Turns Positive",
            "Turns Negative",
        ];

        let condition_idx = Some(condition_to_index(selected_condition));
        let input_for_closure = input_value.to_string();

        let needs_value = !matches!(
            selected_condition,
            AlertCondition::TurnPositive | AlertCondition::TurnNegative
        );

        let form = widget::column()
            .spacing(SPACING_TAB)
            .push(category_header("Condition"))
            .push(
                widget::dropdown(condition_options, condition_idx, move |idx| {
                    Message::AlertSelectCondition(index_to_condition(idx, &input_for_closure))
                })
                .width(Length::Fill),
            )
            .push(
                widget::row()
                    .spacing(SPACING_ROW)
                    .align_y(Alignment::Center)
                    .push_maybe(if needs_value {
                        Some(
                            widget::text_input("value", input_value)
                                .on_input(Message::AlertSetValue)
                                .width(Length::Fill),
                        )
                    } else {
                        None
                    })
                    .push(widget::button::suggested("Add").on_press_maybe(
                        build_add_alert_message(
                            wallet_index,
                            selected_symbol,
                            selected_condition,
                            input_value,
                        ),
                    )),
            );

        col = col.push(
            widget::container(form)
                .padding(PAD_CARD)
                .width(Length::Fill),
        );
    }

    // ================= ALERT LIST =================
    col = col.push(category_divider());
    col = col.push(category_header("Your Alerts"));

    if alerts.is_empty() {
        col = col.push(
            widget::text("No alerts yet.")
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

    widget::row()
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
    match condition {
        AlertCondition::PriceAbove(v) => format!("Price > {v:.2}"),
        AlertCondition::PriceBelow(v) => format!("Price < {v:.2}"),
        AlertCondition::VariationAbove(v) => format!("Variation > {v:.2}%"),
        AlertCondition::VariationBelow(v) => format!("Variation < {v:.2}%"),
        AlertCondition::TurnPositive => "Turns Positive".to_string(),
        AlertCondition::TurnNegative => "Turns Negative".to_string(),
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

fn render_settings_tab<'a>(config: &'a Config, news_input: &'a str) -> Element<'a, Message> {
    widget::column()
        .spacing(SPACING_TAB)
        .padding(PAD_TAB)
        .width(Length::Fill)
        .push(category_header("Panel"))
        .push(
            widget::row()
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .push(widget::text("Show icon only"))
                .push(widget::horizontal_space())
                .push(
                    widget::toggler(config.show_only_icon).on_toggle(Message::ToggleShowOnlyIcon),
                ),
        )
        .push(
            widget::row()
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .push(widget::text("Disable Custom Alerts"))
                .push(widget::horizontal_space())
                .push(
                    widget::toggler(!config.alerts_enabled)
                        .on_toggle(|val| Message::ToggleAlertsEnabled(!val)),
                ),
        )
        .push(
            widget::row()
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .push(widget::text("Display news"))
                .push(widget::horizontal_space())
                .push(widget::toggler(config.show_news).on_toggle(Message::ToggleShowNews)),
        )
        .push(
            widget::row()
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .push(widget::text("News per asset (max 5)"))
                .push(widget::horizontal_space())
                .push(
                    widget::text_input("1", news_input)
                        .on_input(Message::SetNumberOfNewsBySymbols)
                        .width(cosmic::iced::Length::Fixed(60.0)),
                ),
        )
        .push(category_header("Refresh"))
        .push(
            widget::row()
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .push(widget::text("Stock rotation interval (seconds)"))
                .push(widget::horizontal_space())
                .push(
                    widget::text_input("20", config.panel_stoke_rotation_interval.to_string())
                        .on_input(Message::SetStokeRotationInterval)
                        .width(cosmic::iced::Length::Fixed(60.0)),
                ),
        )
        .push(widget::text("Refresh interval"))
        .push(refresh_row(config))
        .push(category_header("About"))
        .push(
            widget::row()
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .push(widget::text("Version: "))
                .push(widget::text(crate::app::VERSION))
                .push(widget::horizontal_space())
                .push(
                    widget::button::standard(crate::fl!("settings-tip-kofi")).on_press(
                        Message::OpenNewsLink("https://ko-fi.com/paulorosado".to_string()),
                    ),
                ),
        )
        .into()
}

fn refresh_row(config: &Config) -> Element<'_, Message> {
    let intervals = [
        ("5 min", RefreshInterval::FiveMinutes),
        ("10 min", RefreshInterval::TenMinutes),
        ("15 min", RefreshInterval::FifteenMinutes),
        ("30 min", RefreshInterval::ThirtyMinutes),
        ("1h", RefreshInterval::OneHour),
    ];

    let mut row = widget::row()
        .spacing(4)
        .width(Length::Fill)
        .height(Length::Shrink);

    for (label, value) in intervals {
        row = row.push(
            widget::container(refresh_button(label, value, config.refresh_interval))
                .width(Length::FillPortion(1)),
        );
    }

    row.into()
}

fn refresh_button<'a>(
    label: &'static str,
    value: RefreshInterval,
    current: RefreshInterval,
) -> Element<'a, Message> {
    let selected = value == current;

    let content = widget::container(widget::text(label))
        .width(Length::Fill)
        .align_x(cosmic::iced::alignment::Horizontal::Center);

    widget::button::custom(content)
        .class(if selected {
            cosmic::theme::Button::Suggested
        } else {
            cosmic::theme::Button::Standard
        })
        .width(Length::Fill)
        .padding([8, 0])
        .on_press(Message::SetRefreshInterval(value))
        .into()
}

fn tab_button<'a>(label: &'static str, msg: Message, selected: bool) -> Element<'a, Message> {
    let text = widget::text(label);
    let underline = widget::container(widget::Space::new(0, 0))
        .height(2)
        .width(Length::Fill)
        .style(move |theme: &cosmic::Theme| {
            let color = if selected {
                cosmic::iced::Color::from(theme.cosmic().accent_color())
            } else {
                cosmic::iced::Color::TRANSPARENT
            };
            widget::container::Style {
                background: Some(color.into()),
                ..Default::default()
            }
        });
    let content = widget::column()
        .align_x(cosmic::iced::alignment::Horizontal::Center)
        .spacing(4)
        .push(text)
        .push(underline);
    widget::button::custom(content)
        .width(Length::FillPortion(1))
        .padding([8, 0])
        .class(cosmic::theme::Button::Text)
        .on_press(msg)
        .into()
}
