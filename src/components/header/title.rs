use std::collections::HashMap;

use cosmic::{Element, widget};

use crate::config::PopupTab;

pub fn title_dynamic(active_tab: PopupTab) -> Element<'static, crate::app::Message> {
    match active_tab {
        PopupTab::Overview => widget::text::heading(crate::fl!("overview-title")).into(),
        PopupTab::Wallet => widget::text::heading("Wallet").into(),
        PopupTab::News => widget::text::heading("News").into(),
        PopupTab::Settings => widget::text::heading(crate::fl!("settings-title")).into(),
        PopupTab::Alerts => widget::text::heading("Alerts").into(),
        PopupTab::Trending => widget::text::heading("Alerts").into(),
        PopupTab::Info => widget::text::heading("MarketWatch").into(),
    }
}

pub fn overview_subtitle(
    active_tab: PopupTab,
    last_updated: Option<String>,
) -> Element<'static, crate::app::Message> {
    if active_tab != PopupTab::Overview {
        return widget::Space::new().into();
    }

    let text = last_updated
        .map(|t| crate::fl!("wallet-updated-at", HashMap::from([("time", t)])))
        .unwrap_or_default();

    widget::container(
        widget::text::caption_heading(text)
            .size(12)
            .class(cosmic::theme::Text::Default),
    )
    .into()
}
