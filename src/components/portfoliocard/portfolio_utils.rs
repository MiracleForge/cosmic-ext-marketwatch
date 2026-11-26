use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, settings},
};
// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    app::Message,
    components::{
        header::nav_icon,
        portfoliocard::{PortfolioView, QuoteStyle},
        ui::{friendly_message, pin_button, port_quote_card},
    },
    marketwatch::PriceAlert,
    utils::{SkeletonCfg, render_card_state},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortFolio {
    pub name: String,
    pub symbols: Vec<String>,
    #[serde(default)]
    pub alerts: Vec<PriceAlert>,
}

impl PortFolio {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            symbols: Vec::new(),
            alerts: Vec::new(),
        }
    }
}

pub fn data_path() -> PathBuf {
    let base = std::env::var("XDG_DATA_HOME").map_or_else(
        |_| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".local/share")
        },
        PathBuf::from,
    );
    base.join("com.github.MiracleForge.cosmic-ext-marketwatch")
}

pub fn portfolio_file() -> PathBuf {
    let new_path = data_path().join("portfolios.json");
    let old_path = data_path().join("wallet.json");

    // old file name
    if !new_path.exists()
        && old_path.exists()
        && let Err(error) = std::fs::rename(&old_path, &new_path)
    {
        eprintln!("Failed to migrate wallet data: {error}");
    }

    new_path
}

pub fn load_portfolio() -> Vec<PortFolio> {
    let path = portfolio_file();
    if !path.exists() {
        return Vec::new();
    }
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_portfolios(portfolio: &[PortFolio]) {
    let path = portfolio_file();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(portfolio) {
        let _ = std::fs::write(&path, json);
    }
}

pub fn portfolio_bar<'a>(portfolio: &PortfolioView<'a>) -> Element<'a, Message> {
    let has_selected = portfolio.count > 0 && portfolio.index > 0;
    let spacing = cosmic::theme::spacing();

    let name_space: Option<Element<'_, Message>> =
        has_selected.then(|| match portfolio.rename_input {
            Some(input) => widget::text_input(crate::fl!("wallet-name"), input)
                .on_input(Message::RenamePortfolio)
                .on_submit(|_| Message::ConfirmRenamePortfolio)
                .width(Length::Shrink)
                .into(),
            None => widget::text::heading(portfolio.name.unwrap_or("Portfolio")).into(),
        });

    let rename_button = has_selected.then(|| {
        let (icon, message) = match portfolio.rename_input {
            Some(_) => ("emblem-ok-symbolic", Message::ConfirmRenamePortfolio),
            None => (
                "accessories-text-editor-symbolic",
                Message::ToggleRenameMode,
            ),
        };

        nav_icon(icon, message, false)
    });

    let previous_button = widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
        .on_press_maybe(
            (!(!has_selected || portfolio.index <= 1)).then_some(Message::PreviusPortfolio),
        );

    let next_button = widget::button::icon(widget::icon::from_name("go-next-symbolic"))
        .on_press_maybe(
            (!(!has_selected || portfolio.index >= portfolio.count))
                .then_some(Message::NextPortfolio),
        );

    widget::row::with_capacity(8)
        .push_maybe(name_space)
        .push_maybe(rename_button)
        .push(widget::Space::new().width(Length::Fill))
        .push_maybe(has_selected.then(|| pin_button(portfolio.pinned, portfolio.index)))
        .push_maybe(has_selected.then(|| {
            nav_icon(
                "edit-delete-symbolic",
                Message::DeleteCurrentPortfolio,
                false,
            )
        }))
        .push(previous_button)
        .push(widget::text::heading(format!(
            "{}/{}",
            portfolio.index, portfolio.count
        )))
        .push(next_button)
        .spacing(spacing.space_xxs)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .into()
}

pub fn portfolio_body<'a>(
    portfolio: &PortfolioView<'a>,
    style: &QuoteStyle<'a>,
) -> Element<'a, Message> {
    let has_selected = portfolio.count > 0 && portfolio.index > 0;

    match (has_selected, portfolio.symbols.is_empty()) {
        (false, _) => friendly_message(
            crate::fl!("port-not-found").to_string(),
            crate::fl!("port-warning").to_string(),
        ),
        (true, true) => friendly_message(
            crate::fl!("port-no-assets").to_string(),
            crate::fl!("port-no-assets-hint").to_string(),
        ),
        (true, false) => render_card_state(
            portfolio.quotes,
            portfolio.error,
            portfolio.loading,
            Some(SkeletonCfg {
                show_logo: style.show_logos,
                pulse: style.pulse,
            }),
            |quotes| {
                quotes
                    .iter()
                    .filter(|q| portfolio.symbols.contains(&q.symbol))
                    .fold(settings::section(), |section, quote| {
                        section.add(port_quote_card(
                            quote,
                            style.show_logos,
                            style.logo_handles.get(&quote.symbol),
                        ))
                    })
                    .into()
            },
        ),
    }
}
