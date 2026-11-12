use std::collections::HashMap;

use cosmic::{Element, widget};

use crate::config::PopupTab;

pub fn title_dynamic(active_tab: PopupTab) -> Option<Element<'static, crate::app::Message>> {
    match active_tab {
        PopupTab::Overview => Some(widget::text::heading(crate::fl!("overview-title")).into()),
        PopupTab::Wallet => Some(widget::text::heading("Wallet").into()),
        PopupTab::News => Some(widget::text::heading("News").into()),
        PopupTab::Settings => Some(widget::text::heading(crate::fl!("settings-title")).into()),
        PopupTab::Alerts | PopupTab::Trending => Some(widget::text::heading("Alerts").into()),
        PopupTab::Info => None,
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
