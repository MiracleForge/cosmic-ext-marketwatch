use cosmic::iced::{Alignment, Length};
use cosmic::theme::Text;
use cosmic::widget;

use crate::app::Message;
use crate::config::Config;
use crate::marketwatch::MarketQuote;

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

fn build_icon_only() -> widget::Row<'static, Message> {
    base_row().push(
        widget::icon::from_name("org.gnome.PowerStats-symbolic")
            .size(16)
            .symbolic(true),
    )
}

fn build_quote_display(quote: &MarketQuote) -> widget::Row<'static, Message> {
    let color = quote.variation_color();

    base_row()
        .spacing(12)
        .width(Length::Fixed(280.0))
        .push(
            widget::icon::from_name("org.gnome.PowerStats-symbolic")
                .size(16)
                .symbolic(true),
        )
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
