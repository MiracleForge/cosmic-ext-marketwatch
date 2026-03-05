use crate::marketwatch::{
    MarketQuote, YahooNews, format_publish_time, user_friendly_error_message,
};
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::theme::Text;
use cosmic::widget;

use crate::app::{MAX_ASSETS_PER_WALLET, Message};
use crate::config::{Config, PopupTab, RefreshInterval};

const NEWS_PREVIEW_COUNT: usize = 3;

#[allow(clippy::too_many_arguments)]
pub fn maincard<'a>(
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
) -> Element<'a, Message> {
    match active_tab {
        PopupTab::Settings => render_settings_tab(config),
        _ => {
            if current_wallet_index == 0 {
                render_quotes(
                    market_quotes,
                    news_items,
                    news_expanded,
                    config,
                    error_message,
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
) -> Element<'a, Message> {
    let mut col = widget::column()
        .spacing(12)
        .width(Length::Fill)
        .padding([8, 12]);

    if asset_limit_reached {
        col = col.push(category_header("ADD ASSET")).push(
            widget::text(format!(
                "Asset limit reached ({} max).",
                MAX_ASSETS_PER_WALLET
            ))
            .size(12)
            .class(cosmic::theme::Text::Accent),
        );
    } else {
        col = col.push(category_header("ADD ASSET")).push(
            widget::text_input("Search by symbol (e.g. AAPL, PETR4)", search_input)
                .on_input(Message::StockSearchInput)
                .width(Length::Fill),
        );

        if search_loading {
            col = col.push(
                widget::text("Searching...")
                    .size(12)
                    .class(cosmic::theme::Text::Accent),
            );
        } else if !search_results.is_empty() {
            let results_col = widget::column()
                .spacing(2)
                .extend(search_results.iter().map(|label| {
                    widget::button::standard(label.as_str())
                        .on_press(Message::AddStockToWallet(label.clone()))
                        .width(Length::Fill)
                        .into()
                }));
            col = col.push(results_col);
        } else if search_input.len() >= 2 {
            col = col.push(
                widget::text("No results found.")
                    .size(12)
                    .class(cosmic::theme::Text::Accent),
            );
        }
    }

    if symbols.is_empty() && quotes.is_empty() {
        col = col.push(category_divider()).push(
            widget::text("Your portfolio is empty.")
                .size(12)
                .class(cosmic::theme::Text::Accent),
        );
    } else {
        col = col
            .push(category_header("Your Portfolio"))
            .push(category_divider());

        if quotes.is_empty() {
            for symbol in symbols {
                let row = widget::row()
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
                                .on_press(Message::RemoveStockFromWallet(symbol.clone()))
                                .padding([4, 8]),
                        )
                        .width(Length::FillPortion(1))
                        .align_x(Alignment::End),
                    );
                col = col.push(row).push(item_divider());
            }
        } else {
            for quote in quotes {
                let color = quote.variation_color();
                let row = widget::row()
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                    .push(
                        widget::container(
                            widget::text::heading(&quote.symbol).class(Text::Default),
                        )
                        .width(Length::FillPortion(2))
                        .align_x(Alignment::Start),
                    )
                    .push(
                        widget::container(
                            widget::text(quote.formatted_price()).class(Text::Color(color)),
                        )
                        .width(Length::FillPortion(2))
                        .align_x(Alignment::Center),
                    )
                    .push(
                        widget::container(
                            widget::text(quote.formatted_variation()).class(Text::Color(color)),
                        )
                        .width(Length::FillPortion(1))
                        .align_x(Alignment::Center),
                    )
                    .push(
                        widget::container(
                            widget::button::icon(widget::icon::from_name("list-remove-symbolic"))
                                .on_press(Message::RemoveStockFromWallet(quote.symbol.clone()))
                                .padding([4, 8]),
                        )
                        .width(Length::FillPortion(1))
                        .align_x(Alignment::End),
                    );
                col = col.push(row).push(item_divider());
            }
        }
    }

    if config.show_news {
        col = col.push(render_news_section(news_items, news_expanded));
    }

    col.into()
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
) -> Element<'a, Message> {
    let content = widget::column()
        .spacing(12)
        .width(Length::Fill)
        .padding([8, 12]);

    match derive_state(market_quotes, error_message) {
        QuotesState::Loading => content.push(widget::text("Loading market data...")).into(),

        QuotesState::Error(err) => {
            let friendly = user_friendly_error_message(err);
            content.push(widget::text(friendly)).into()
        }

        QuotesState::Ready(quotes) => {
            let col = content
                .push(category_header("Market Overview"))
                .push(category_divider());

            let col = render_quotes_list(col, quotes);

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
        .push(widget::text("Latest News").size(12).class(Text::Accent))
        .push(widget::horizontal_space())
        // FIX: labels corretos — expanded = já expandido = mostrar "Collapse"
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
            .size(12)
            .class(Text::Accent)
            .into()
    } else {
        let visible = if expanded {
            news
        } else {
            &news[..NEWS_PREVIEW_COUNT.min(news.len())]
        };

        let cards = widget::column()
            .spacing(6)
            .width(Length::Fill)
            .extend(visible.iter().map(|item| news_card(item)));

        if expanded {
            widget::scrollable(cards)
                .height(Length::Fixed(300.0))
                .into()
        } else {
            cards.into()
        }
    };

    widget::column()
        .spacing(8)
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
        .padding([8, 10])
        .width(Length::Fill)
        .push(widget::text(&item.title).size(13))
        .push(widget::text(meta_text).size(11).class(Text::Accent));

    widget::button::custom(content)
        .class(cosmic::theme::Button::MenuItem)
        .width(Length::Fill)
        .on_press(Message::OpenNewsLink(item.link.clone()))
        .into()
}

fn render_quotes_list<'a>(
    mut content: widget::Column<'a, Message>,
    market_quotes: &'a [MarketQuote],
) -> widget::Column<'a, Message> {
    for quote in market_quotes {
        let color = quote.variation_color();

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

// FIX: elidable_lifetime_names
fn category_header(label: &str) -> Element<'_, Message> {
    widget::text(label).size(12).class(Text::Accent).into()
}

// FIX: elidable_lifetime_names
fn render_settings_tab(config: &Config) -> Element<'_, Message> {
    widget::column()
        .spacing(12)
        .padding([8, 12])
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
                .push(widget::text("Display news"))
                .push(widget::horizontal_space())
                .push(widget::toggler(config.show_news).on_toggle(Message::ToggleShowNews)),
        )
        .push(
            widget::row()
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .push(widget::text("News per asset"))
                .push(widget::horizontal_space())
                .push(
                    widget::text_input("5", config.count_news_by_simbol.to_string())
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
                .push(widget::text("VERSION"))
                .push(widget::horizontal_space())
                .push(widget::button::standard(crate::fl!("settings-tip-kofi"))),
        )
        .into()
}

// FIX: elidable_lifetime_names
fn refresh_row(config: &Config) -> Element<'_, Message> {
    let intervals = [
        ("5 min", RefreshInterval::FiveMinutes),
        ("10 min", RefreshInterval::TenMinutes),
        ("15 min", RefreshInterval::FifteenMinutes),
        ("30 min", RefreshInterval::ThirtyMinutes),
        ("1h", RefreshInterval::OneHour),
    ];

    let mut row = widget::row()
        .spacing(0)
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
