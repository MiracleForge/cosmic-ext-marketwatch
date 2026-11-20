use std::collections::HashMap;

// SPDX-License-Identifier: GPL-3.0-only
use crate::app::Message;
use crate::config::Config;
use crate::marketwatch::MarketQuote;
use crate::marketwatch::QuoteFormat;
use cosmic::iced::{Alignment, Font, Length};
use cosmic::theme::Text;
use cosmic::{Element, Theme, widget};

//TODO: INSTALL ICON WITH JUST INSTALL
const ICON: &[u8] = include_bytes!(
    "../../../resources/icons/hicolor/scalable/apps/com.github.MiracleForge.cosmic-ext-marketwatch-panel.svg"
);

pub fn build_applet_content<'a>(
    config: &Config,
    market_quotes: &'a [MarketQuote],
    current_index: usize,
    logo_handles: &'a HashMap<String, widget::image::Handle>,
    is_horizontal: bool,
    error_message: Option<&String>,
    core: &cosmic::Core,
) -> Element<'a, Message> {
    let size = core.applet.suggested_size(true).0;
    if error_message.is_some() {
        return build_error_display().into();
    }
    if config.show_only_icon {
        return build_icon_only(size).into();
    }

    let theme = core.system_theme();

    match market_quotes.get(current_index) {
        Some(quote) => {
            let logo_handle = config
                .show_panel_logos
                .then(|| logo_handles.get(&quote.symbol))
                .flatten();

            if is_horizontal {
                build_quote_display(quote, logo_handle, theme).into()
            } else {
                build_vertical_quote(quote, theme)
            }
        }
        None => build_loading_display().into(),
    }
}

fn base_row() -> widget::Row<'static, Message, cosmic::Theme> {
    widget::row![].align_y(Alignment::Center)
}

fn app_icon(icon_size: u16) -> widget::Icon {
    // symbolic(true) makes COSMIC recolor `currentColor` in the SVG to the
    // theme's text color, so the icon adapts to light/dark themes.
    widget::icon(widget::icon::from_svg_bytes(ICON).symbolic(true)).size(icon_size)
}

fn build_icon_only(icon_size: u16) -> widget::Row<'static, Message, cosmic::Theme> {
    base_row().push(app_icon(icon_size))
}

fn build_quote_display<'a>(
    quote: &'a MarketQuote,
    logo_handle: Option<&'a widget::image::Handle>,
    theme: &Theme,
) -> widget::Row<'a, Message, cosmic::Theme> {
    let color = quote.variation_color(theme);

    let price = quote.formatted_price();
    let variation = quote.formatted_variation();

    let mut row = base_row().spacing(8);

    if let Some(handle) = logo_handle {
        row = row.push(
            widget::image(handle.clone())
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0))
                .content_fit(cosmic::iced::ContentFit::Contain),
        );
    }

    row.width(Length::Shrink)
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

    widget::column![]
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

fn build_loading_display() -> widget::Row<'static, Message, cosmic::Theme> {
    base_row()
        .spacing(12)
        .push(widget::icon::from_name("process-working-symbolic").symbolic(true))
        .push(widget::text(crate::fl!("display-loading")))
}

fn build_error_display() -> widget::Row<'static, Message, cosmic::Theme> {
    base_row()
        .spacing(8)
        .push(widget::icon::from_name("dialog-error-symbolic").symbolic(true))
        .push(widget::text(crate::fl!("display-error")))
}
