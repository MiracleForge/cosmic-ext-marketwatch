use crate::app::Message;
use crate::config::Config;
use crate::marketwatch::MarketQuote;
use cosmic::iced::Alignment;
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
    theme: &Theme,
) -> Element<'static, Message> {
    if error_message.is_some() {
        return build_error_display().into();
    }
    if config.show_only_icon {
        return build_icon_only().into();
    }

    match market_quotes.get(current_index) {
        Some(quote) if !is_horizontal => build_vertical_quote(quote, theme).into(),
        Some(quote) => build_quote_display(quote, theme).into(),
        None => build_loading_display().into(),
    }
}

fn base_row() -> widget::Row<'static, Message> {
    widget::row().align_y(Alignment::Center)
}

fn app_icon() -> widget::Icon {
    widget::icon(widget::icon::from_svg_bytes(ICON)).size(20)
}

fn build_icon_only() -> widget::Row<'static, Message> {
    base_row().push(app_icon())
}

fn build_quote_display(quote: &MarketQuote, theme: &Theme) -> widget::Row<'static, Message> {
    let color = quote.variation_color(theme);

    base_row()
        .spacing(24)
        .push(widget::text::heading(quote.symbol.clone()))
        .push(widget::text(quote.formatted_price()).class(Text::Color(color)))
        .push(widget::text(quote.formatted_variation()).class(Text::Color(color)))
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
        .push(
            widget::icon::from_name("process-working-symbolic")
                .size(16)
                .symbolic(true),
        )
        .push(widget::text("Loading..."))
}

fn build_error_display() -> widget::Row<'static, Message> {
    base_row()
        .spacing(8)
        .push(
            widget::icon::from_name("dialog-error-symbolic")
                .size(16)
                .symbolic(true),
        )
        .push(widget::text("Error"))
}
