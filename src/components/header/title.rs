use std::collections::HashMap;

use cosmic::{Element, widget};

use crate::{config::PopupTab, constants::TEXT_LABEL};

pub fn title_dynamic(active_tab: PopupTab) -> Option<Element<'static, crate::app::Message>> {
    let title = match active_tab {
        PopupTab::Overview => crate::fl!("overview-title").to_string(),
        PopupTab::Wallet => "Wallet".to_string(),
        PopupTab::News => "News".to_string(),
        PopupTab::Settings => crate::fl!("settings-title").to_string(),
        PopupTab::Alerts | PopupTab::Trending => "Alerts".to_string(),
        PopupTab::Info => return None,
    };

    Some(widget::text::title4(title).into())
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
            .size(10)
            .class(cosmic::theme::Text::Default),
    )
    .into()
}
