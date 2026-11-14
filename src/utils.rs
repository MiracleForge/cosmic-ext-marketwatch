use cosmic::{Element, Theme, iced::Length, widget};

use crate::{
    app::Message,
    components::{overviewcard::quotes_skeleton, ui::friendly_message},
    marketwatch::user_friendly_error_message,
};

enum CardState<'a, T> {
    Loading,
    Empty,
    Error(&'a str),
    Ready(&'a [T]),
}

fn derive_state<'a, T>(
    items: &'a [T],
    error: Option<&'a String>,
    loading: bool,
) -> CardState<'a, T> {
    if let Some(err) = error {
        CardState::Error(err)
    } else if !items.is_empty() {
        CardState::Ready(items)
    } else if loading {
        CardState::Loading
    } else {
        CardState::Empty
    }
}

pub struct SkeletonCfg<'t> {
    pub show_logo: bool,
    pub pulse: f32,
    pub theme: &'t Theme,
}

pub fn render_card_state<'a, T, F>(
    items: &'a [T],
    error: Option<&'a String>,
    loading: bool,
    skeleton: Option<SkeletonCfg<'_>>,
    render_ready: F,
) -> Element<'a, Message>
where
    F: FnOnce(&'a [T]) -> Element<'a, Message>,
{
    match derive_state(items, error, loading) {
        CardState::Loading => match skeleton {
            Some(cfg) => quotes_skeleton(cfg.show_logo, cfg.pulse, cfg.theme),
            None => widget::container(widget::text::body(crate::fl!("quotes-loading")))
                .width(Length::Fill)
                .padding([16, 16])
                .center_x(Length::Fill)
                .into(),
        },

        CardState::Error(err) => {
            let (title, description) = user_friendly_error_message(err);
            friendly_message(title.to_string(), description.to_string())
        }

        CardState::Empty => friendly_message(
            "Something happened".to_string(),
            "Something happened description".to_string(),
        ),

        CardState::Ready(items) => render_ready(items),
    }
}
