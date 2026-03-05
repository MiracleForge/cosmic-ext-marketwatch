use crate::app::Message;
use crate::config::Config;
use crate::marketwatch::MarketQuote;
use cosmic::iced::{Alignment, Length};
use cosmic::theme::Text;
use cosmic::widget;

//TODO: INSTALL ICON WITH JUST INSTALL
const ICON: &[u8] = include_bytes!("../../../resources/icon.svg");

pub fn build_applet_content(
    config: &Config,
    market_quotes: &[MarketQuote],
    current_index: usize,
    error_message: Option<&String>,
) -> widget::Row<'static, Message> {
    if error_message.is_some() {
        return build_error_display();
    }
    if config.show_only_icon {
        return build_icon_only();
    }

    if let Some(current_quote) = market_quotes.get(current_index) {
        return build_quote_display(current_quote);
    }

    build_loading_display()
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

fn build_quote_display(quote: &MarketQuote) -> widget::Row<'static, Message> {
    let color = quote.variation_color();

    base_row()
        .spacing(12)
        .width(Length::Fixed(280.0))
        .push(build_icon_only())
        .push(widget::horizontal_space().width(Length::Fill))
        .push(widget::text::heading(quote.symbol.clone()))
        .push(widget::horizontal_space().width(Length::Fill))
        .push(widget::text(quote.formatted_price()).class(Text::Color(color)))
        .push(widget::horizontal_space().width(Length::Fill))
        .push(widget::text(quote.formatted_variation()).class(Text::Color(color)))
}

fn build_loading_display() -> widget::Row<'static, Message> {
    base_row()
        .spacing(12)
        .width(Length::Fixed(280.0))
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
