use cosmic::{
    Element, Theme,
    iced::{Alignment, Length},
    widget::{self, Space},
};
// SPDX-License-Identifier: GPL-3.0-only
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{
    app::Message,
    components::{
        buttons::icon_button,
        portfoliocard::{PortfolioView, QuoteStyle},
        ui::{friendly_message, pin_button, port_quote_card},
    },
    marketwatch::PriceAlert,
    utils::{SkeletonCfg, render_card_state},
};

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

pub fn portfolio_bar<'a>(theme: &Theme, portfolio: &PortfolioView<'a>) -> Element<'a, Message> {
    let has_selected = portfolio.count > 0 && portfolio.index > 0;

    let rename_button = has_selected.then(|| {
        let (icon, msg) = match portfolio.rename_input {
            Some(_) => ("emblem-ok-symbolic", Message::ConfirmRenamePortfolio),
            None => (
                "accessories-text-editor-symbolic",
                Message::ToggleRenameMode,
            ),
        };
        icon_button(None, icon, msg)
    });

    let name_space: Option<Element<'_, Message>> =
        has_selected.then(|| match portfolio.rename_input {
            Some(input) => widget::text_input(crate::fl!("wallet-name"), input)
                .on_input(Message::RenamePortfolio)
                .on_submit(|_| Message::ConfirmRenamePortfolio)
                .width(Length::Shrink)
                .into(),
            None => widget::text::heading(portfolio.name.unwrap_or("Portfolio")).into(),
        });

    widget::row![
        name_space,
        rename_button,
        Space::new().width(Length::Fill),
        has_selected.then(|| pin_button(portfolio.pinned, portfolio.index, theme)),
        has_selected.then(|| icon_button(
            None,
            "edit-delete-symbolic",
            Message::DeleteCurrentPortfolio
        )),
        icon_button(
            Some(!has_selected || portfolio.index <= 1),
            "go-previous-symbolic",
            Message::PreviusPortfolio,
        ),
        widget::text::heading(format!("{}/{}", portfolio.index, portfolio.count)),
        icon_button(
            Some(!has_selected || portfolio.index >= portfolio.count),
            "go-next-symbolic",
            Message::NextPortfolio,
        ),
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .into()
}

pub fn portfolio_body<'a>(
    theme: &Theme,
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
                widget::column::with_children(
                    quotes
                        .iter()
                        .filter(|q| portfolio.symbols.contains(&q.symbol))
                        .map(|q| {
                            port_quote_card(
                                q,
                                style.show_logos,
                                style.logo_handles.get(&q.symbol),
                                theme,
                            )
                        }),
                )
                .spacing(1)
                .width(Length::Fill)
                .into()
            },
        ),
    }
}
