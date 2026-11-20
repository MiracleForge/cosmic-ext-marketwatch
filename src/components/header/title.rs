use crate::config::PopupTab;
use cosmic::{Element, widget};
use std::collections::HashMap;

pub fn title_dynamic(
    active_tab: PopupTab,
    pinned_port_tittle: Option<String>,
) -> Option<Element<'static, crate::app::Message>> {
    let title = match active_tab {
        PopupTab::Overview => crate::fl!("overview-title").to_string(),
        PopupTab::Portfolio => crate::fl!("port-base").to_string(),
        PopupTab::News => "News".to_string(),
        PopupTab::Settings => crate::fl!("settings-title").to_string(),
        PopupTab::Pinned => pinned_port_tittle?,
        PopupTab::Tools | PopupTab::Info => return None,
    };

    Some(widget::text::title4(title).into())
}

pub fn overview_subtitle(
    active_tab: PopupTab,
    last_updated: Option<String>,
) -> Element<'static, crate::app::Message> {
    if !matches!(
        active_tab,
        PopupTab::Overview | PopupTab::Portfolio | PopupTab::Tools | PopupTab::Pinned
    ) {
        return widget::Space::new().into();
    }

    let text = last_updated
        .map(|t| crate::fl!("wallet-updated-at", HashMap::from([("time", t)])))
        .unwrap_or_default();

    widget::container(widget::text::caption_heading(text).size(10))
        .style(|theme| widget::container::Style {
            text_color: Some(cosmic::iced::Color::from(theme.cosmic().accent_color())),
            ..Default::default()
        })
        .into()
}
