use crate::marketwatch::{MarketQuote, user_friendly_error_message};
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::theme::Text;
use cosmic::widget;

use crate::app::Message;
use crate::config::{Config, PopupTab};

pub fn maincard<'a>(
    active_tab: PopupTab,
    market_quotes: &'a [MarketQuote],
    config: &'a Config,
    error_message: &'a Option<String>,
) -> Element<'a, Message> {
    match active_tab {
        PopupTab::Settings => render_settings_tab(config),
        _ => render_quotes(market_quotes, error_message),
    }
}

enum QuotesState<'a> {
    Loading,
    Error(&'a str),
    Ready(&'a [MarketQuote]),
}

fn derive_state<'a>(
    market_quotes: &'a [MarketQuote],
    error_message: &'a Option<String>,
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
    error_message: &'a Option<String>,
) -> Element<'a, Message> {
    let content = widget::column()
        .spacing(12)
        .width(Length::Fill)
        .padding([8, 12]);

    match derive_state(market_quotes, error_message) {
        QuotesState::Loading => content.push(widget::text("Loading market data...")).into(),

        QuotesState::Error(err) => {
            let friendly = user_friendly_error_message(err);

            content
                .push(
                    widget::column()
                        .spacing(8)
                        .align_x(Alignment::Center)
                        .push(
                            widget::icon::from_name("dialog-error-symbolic")
                                .size(24)
                                .symbolic(true),
                        )
                        .push(widget::text("Connection Problem").class(Text::Accent))
                        .push(widget::text(friendly)),
                )
                .into()
        }

        QuotesState::Ready(quotes) => render_quotes_list(content, quotes),
    }
}

fn render_quotes_list<'a>(
    mut content: widget::Column<'a, Message>,
    market_quotes: &'a [MarketQuote],
) -> Element<'a, Message> {
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

        content = content.push(row).push(divider());
    }

    content.into()
}

fn divider<'a>() -> Element<'a, Message> {
    widget::container(widget::horizontal_space())
        .width(Length::Fill)
        .height(1)
        .style(|theme: &cosmic::Theme| widget::container::Style {
            background: Some(cosmic::iced::Color::from(theme.cosmic().accent_color()).into()),
            ..Default::default()
        })
        .into()
}

fn section_header<'a>(label: &'a str) -> Element<'a, Message> {
    widget::text(label).size(12).class(Text::Accent).into()
}

fn render_settings_tab<'a>(config: &'a Config) -> Element<'a, Message> {
    widget::column()
        .spacing(12)
        .padding([8, 12])
        .width(Length::Fill)
        .push(section_header("Settings"))
        .push(
            widget::row()
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .push(widget::text("Show only icon"))
                .push(widget::horizontal_space())
                .push(
                    widget::toggler(config.show_only_icon).on_toggle(Message::ToggleShowOnlyIcon),
                ),
        )
        .push(section_header("Settings2"))
        .into()
}
