// SPDX-License-Identifier: GPL-3.0-only
use crate::app::Message;
use crate::config::Config;
use crate::marketwatch::MarketQuote;
use cosmic::iced::{Alignment, Font, Length};
use cosmic::theme::Text;
use cosmic::{Element, Theme, widget};

//TODO: INSTALL ICON WITH JUST INSTALL
const ICON: &[u8] = include_bytes!("../../../resources/icon.svg");

pub fn build_applet_content(
    config: &Config,
    market_quotes: &[MarketQuote],
    current_index: usize,
    is_horizontal: bool,
    error_message: Option<&String>,
    core: &cosmic::Core,
) -> Element<'static, Message> {
    let size = core.applet.suggested_size(true).0;
    if error_message.is_some() {
        return build_error_display().into();
    }
    if config.show_only_icon {
        return build_icon_only(size).into();
    }

    let theme = core.system_theme();
    match market_quotes.get(current_index) {
        Some(quote) if !is_horizontal => build_vertical_quote(quote, theme),
        Some(quote) => build_quote_display(quote, theme).into(),
        None => build_loading_display().into(),
    }
}

fn base_row() -> widget::Row<'static, Message> {
    widget::row().align_y(Alignment::Center)
}

fn app_icon(icon_size: u16) -> widget::Icon {
    widget::icon(widget::icon::from_svg_bytes(ICON)).size(icon_size)
}

fn build_icon_only(icon_size: u16) -> widget::Row<'static, Message> {
    base_row().push(app_icon(icon_size))
}

fn build_quote_display(quote: &MarketQuote, theme: &Theme) -> widget::Row<'static, Message> {
    let color = quote.variation_color(theme);

    // Thying to fix layout shift but I don't know how , whern I have like btc-usd  $80,634.41 for
    // exemple. For now I will leave like that
    let price = quote.formatted_price();
    let variation = quote.formatted_variation();

    base_row()
        .spacing(8)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .push(widget::text(quote.symbol.clone()).font(Font::MONOSPACE))
        .push(
            widget::text(price)
                .class(Text::Color(color))
                .font(Font::MONOSPACE),
        )
        .push(
            widget::text(variation)
                .class(Text::Color(color))
                .font(Font::MONOSPACE),
        )
}

pub fn build_vertical_quote(quote: &MarketQuote, theme: &Theme) -> Element<'static, Message> {
    let color = quote.variation_color(theme);

    widget::column()
        .align_x(Alignment::Center)
        .spacing(4)
        .push(widget::text(quote.symbol.clone()).size(11))
        .push(
            widget::text(quote.formatted_price())
                .size(10)
                .class(Text::Color(color)),
        )
        .push(
            widget::text(quote.formatted_variation())
                .size(10)
                .class(Text::Color(color)),
        )
        .into()
}

fn build_loading_display() -> widget::Row<'static, Message> {
    base_row()
        .spacing(12)
        .push(widget::icon::from_name("process-working-symbolic").symbolic(true))
        .push(widget::text("Loading..."))
}

fn build_error_display() -> widget::Row<'static, Message> {
    base_row()
        .spacing(8)
        .push(widget::icon::from_name("dialog-error-symbolic").symbolic(true))
        .push(widget::text("Error"))
}
