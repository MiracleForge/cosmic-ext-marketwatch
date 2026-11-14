// SPDX-License-Identifier: GPL-3.0-only

use crate::components::ui::{overview_tab, textlabel};
use crate::constants::{SPACING_COL, TEXT_BODY};
use crate::marketwatch::MarketQuote;
use cosmic::Theme;
use cosmic::prelude::*;
use cosmic::widget;

use crate::app::Message;

fn render_portfolio_section<'a>(
    _symbols: &'a [String],
    _quotes: &'a [MarketQuote],
    _theme: &Theme,
) -> Element<'a, Message> {
    widget::Space::new().into()

    // let mut col = widget::column![].spacing(SPACING_COL);
    //
    // if symbols.is_empty() && quotes.is_empty() {
    //     return col
    //         .push(
    //             widget::text(crate::fl!("portfolio-empty"))
    //                 .size(TEXT_BODY)
    //                 .class(cosmic::theme::Text::Accent),
    //         )
    //         .into();
    // }
    //
    // col = col
    //     .push(textlabel(crate::fl!("portfolio-your")))
    //     .push(category_divider());
    //
    // if quotes.is_empty() {
    //     for symbol in symbols {
    //         col = col.push(render_loading_row(symbol)).push(item_divider());
    //     }
    // } else {
    //     for quote in quotes {
    //         col = col
    //             .push(render_quote_row(quote, theme))
    //             .push(item_divider());
    //     }
    // }
    //
    // col.into()
    //
    //

    // fn render_quote_row(quote: &MarketQuote, theme: &Theme) -> Element<'static, Message> {
    //     let color = quote.variation_color(theme);
    //
    //     widget::row![]
    //         .align_y(Alignment::Center)
    //         .width(Length::Fill)
    //         .push(
    //             widget::container(widget::text::heading(quote.symbol.clone()).class(Text::Default))
    //                 .width(Length::FillPortion(2))
    //                 .align_x(Alignment::Start),
    //         )
    //         .push(
    //             widget::container(widget::text(quote.formatted_price()).class(Text::Color(color)))
    //                 .width(Length::FillPortion(2))
    //                 .align_x(Alignment::Center),
    //         )
    //         .push(
    //             widget::container(widget::text(quote.formatted_variation()).class(Text::Color(color)))
    //                 .width(Length::FillPortion(1))
    //                 .align_x(Alignment::Center),
    //         )
    //         .push(
    //             widget::container(
    //                 widget::button::icon(widget::icon::from_name("alarm-symbolic"))
    //                     .on_press(Message::OpenAlertsTab(quote.symbol.clone()))
    //                     .padding(PAD_ROW),
    //             )
    //             .width(Length::FillPortion(1))
    //             .align_x(Alignment::End),
    //         )
    //         .push(
    //             widget::container(
    //                 widget::button::icon(widget::icon::from_name("list-remove-symbolic"))
    //                     .on_press(Message::RemoveStockFromWallet(quote.symbol.clone()))
    //                     .padding(PAD_ROW),
    //             )
    //             .align_x(Alignment::End),
    //         )
    //         .into()
    // }
}
