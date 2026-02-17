use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::theme::Text;
use cosmic::widget;

use crate::app::Message;
use crate::marketwatch::MarketQuote;

pub fn maincard(market_quotes: &[MarketQuote]) -> Element<'static, Message> {
    let mut content = widget::column()
        .spacing(12)
        .width(Length::Fill)
        .padding([8, 12]);

    if market_quotes.is_empty() {
        return content.push(widget::text("Loading market data...")).into();
    }

    for quote in market_quotes {
        let color = quote.variation_color();

        let row = widget::row()
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
                widget::container(
                    widget::text(quote.formatted_variation()).class(Text::Color(color)),
                )
                .width(Length::FillPortion(1))
                .align_x(Alignment::End),
            );

        content = content.push(row).push(
            widget::container(widget::horizontal_space())
                .width(Length::Fill)
                .height(1)
                .style(|theme: &cosmic::Theme| widget::container::Style {
                    background: Some(
                        cosmic::iced::Color::from(theme.cosmic().accent_color()).into(),
                    ),
                    ..Default::default()
                }),
        );
    }
    content.into()
}
