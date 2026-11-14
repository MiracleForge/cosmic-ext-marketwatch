use cosmic::iced::{Border, Length};
use cosmic::theme::Text;
use cosmic::widget::button::Catalog;
use cosmic::{Element, iced::Alignment, widget};

use cosmic::Theme;

use crate::app::Message;
use crate::components::buttons::{refresh_button, segmented_button};
use crate::components::newscard::NewsTab;
use crate::constants::{PAD_CARD, PAD_ROW, SPACING_ROW, TEXT_BODY, TEXT_SMALL};

use crate::config::RefreshInterval;
use crate::marketwatch::{MarketQuote, ScreensTab, YahooNews, format_publish_time};

pub fn badge_class(theme: &Theme) -> cosmic::theme::Container<'static> {
    let btn = theme.active(false, false, &cosmic::theme::Button::Standard);

    let badge_bg = btn.background;
    let badge_text = btn.text_color;
    let radius = btn.border_radius;

    cosmic::theme::Container::custom(move |_| widget::container::Style {
        background: badge_bg,
        text_color: badge_text,
        border: Border {
            radius: radius.into(),
            ..Default::default()
        },
        ..Default::default()
    })
}
pub fn external_link_card<'a>(text: String, url: &str, theme: &Theme) -> Element<'a, Message> {
    let content = widget::row![
        widget::text::heading(text)
            .size(TEXT_BODY)
            .width(Length::Fill),
        widget::button::icon(widget::icon::from_name("link-symbolic"))
            .on_press(Message::OpenExternalLink(url.to_string()))
            .padding(PAD_ROW),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill);

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
        .into()
}

pub fn toggle_card<'a>(
    text: String,
    value: bool,
    message: fn(bool) -> Message,
    theme: &Theme,
) -> Element<'a, Message> {
    let content = widget::row![
        widget::text::heading(text)
            .size(TEXT_BODY)
            .width(Length::Fill),
        widget::toggler(value).on_toggle(message),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill);

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
        .into()
}

pub fn input_card<'a>(
    text: String,
    placeholder: &'a str,
    input: String,
    message: fn(String) -> Message,
    theme: &Theme,
) -> Element<'a, Message> {
    let content = widget::row![
        widget::text::heading(text)
            .size(TEXT_BODY)
            .width(Length::Fill),
        widget::text_input(placeholder, input)
            .on_input(message)
            .width(cosmic::iced::Length::Fixed(60.0)),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill);

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
        .into()
}
pub fn segmented_card<'a>(
    text: String,
    options: &[(&'a str, RefreshInterval)],
    selected: RefreshInterval,
    message: fn(RefreshInterval) -> Message,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut row = widget::row![].spacing(4).width(Length::Fill);

    for (label, value) in options {
        row = row.push(
            widget::container(refresh_button(label, *value, selected, message))
                .width(Length::FillPortion(1)),
        );
    }

    let content = widget::column![widget::text::heading(text).size(TEXT_BODY), row,]
        .spacing(SPACING_ROW)
        .width(Length::Fill);

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
        .into()
}

pub fn quote_card<'a>(
    quote: &'a MarketQuote,
    show_logo: bool,
    logo_handle: Option<&'a widget::image::Handle>,
    theme: &Theme,
) -> Element<'a, Message> {
    let color = quote.variation_color(theme);

    let mut content = widget::row![]
        .spacing(SPACING_ROW)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    if show_logo {
        if let Some(handle) = logo_handle {
            content = content.push(
                widget::image(handle.clone())
                    .width(Length::Fixed(42.0))
                    .height(Length::Fixed(42.0))
                    .content_fit(cosmic::iced::ContentFit::Cover),
            );
        }
    }

    content = content.push(
        widget::column![
            widget::text::heading(&quote.name).size(TEXT_BODY),
            widget::text::caption_heading(&quote.symbol).size(TEXT_BODY),
        ]
        .width(Length::Fill)
        .spacing(2),
    );

    content = content.push(
        widget::column![
            widget::text::caption_heading(quote.formatted_price())
                .size(14)
                .class(Text::Color(color)),
            widget::text::caption_heading(quote.formatted_variation())
                .size(TEXT_BODY)
                .class(Text::Color(color)),
        ]
        .align_x(Alignment::End)
        .spacing(2),
    );

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
        .into()
}

pub fn textlabel(label: impl Into<String>) -> Element<'static, Message> {
    let label: String = label.into();
    widget::container(widget::text::heading(label).size(TEXT_BODY + 1))
        .style(|theme| widget::container::Style {
            text_color: Some(cosmic::iced::Color::from(theme.cosmic().accent_color())),
            ..Default::default()
        })
        .into()
}

pub fn news_frame<'a>(news: &'a YahooNews, theme: &Theme) -> Element<'a, Message> {
    let publisher = news.publisher.as_deref().unwrap_or("Unknown source");

    let time_str = news
        .publish_time
        .map(format_publish_time)
        .unwrap_or_default();

    let meta_text = if time_str.is_empty() {
        publisher.to_string()
    } else {
        format!("{publisher} · {time_str}")
    };

    let content = widget::row![
        widget::column![
            widget::text::heading(&news.title).size(TEXT_BODY),
            widget::row![widget::text(meta_text).size(TEXT_SMALL).class(Text::Accent),]
                .spacing(8)
                .align_y(Alignment::Center),
        ]
        .spacing(4)
        .width(Length::Fill),
        widget::button::icon(widget::icon::from_name("link-symbolic"))
            .on_press(Message::OpenExternalLink(news.link.to_string()))
            .padding(PAD_ROW),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill);

    widget::container(content)
        .padding(PAD_CARD)
        .width(Length::Fill)
        .class(badge_class(theme))
        .into()
}

pub fn overview_tab(current: ScreensTab) -> Element<'static, Message> {
    let content = widget::row![]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Shrink)
        .push(segmented_button(
            crate::fl!("screeners-tab-active"),
            Message::SetTab(ScreensTab::MostActive),
            current == ScreensTab::MostActive,
        ))
        .push(segmented_button(
            crate::fl!("screeners-tab-gainers"),
            Message::SetTab(ScreensTab::Gainers),
            current == ScreensTab::Gainers,
        ))
        .push(segmented_button(
            crate::fl!("screeners-tab-losers"),
            Message::SetTab(ScreensTab::Losers),
            current == ScreensTab::Losers,
        ));

    widget::container(content).padding([8, 0]).into()
}

pub fn news_tab(current_tab: NewsTab) -> Element<'static, Message> {
    let content = widget::row![]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Shrink)
        .push(segmented_button(
            crate::fl!("news-tab-us"),
            Message::SetTab(ScreensTab::MostActive),
            current_tab == NewsTab::USMARKET,
        ))
        .push(segmented_button(
            crate::fl!("news-tab-folio"),
            Message::SetTab(ScreensTab::Gainers),
            current_tab == NewsTab::PORTIFOIO,
        ));

    widget::container(content).padding(10).into()
}

// TEMP ///
pub fn render_loading_row(symbol: &str) -> Element<'_, Message> {
    widget::row![]
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .push(
            widget::container(widget::text::heading(symbol).class(Text::Default))
                .width(Length::FillPortion(2))
                .align_x(Alignment::Start),
        )
        .push(
            widget::container(widget::text(crate::fl!("render-loading")))
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
