// SPDX-License-Identifier: GPL-3.0-only
use crate::config::{self, Config};
use crate::fl;
use crate::marketwatch::{MarketQuote, fetch_most_active};

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use cosmic::iced::Alignment;
use cosmic::iced::{Limits, Subscription, window::Id};
use cosmic::iced_futures::Subscription as IcedSubscription;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::{Action, widget};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// System-wide time format preference from COSMIC time applet.
#[derive(Debug, Clone, Default, PartialEq, Eq, CosmicConfigEntry, Deserialize, Serialize)]
pub struct TimeAppletConfig {
    #[serde(default)]
    pub military_time: bool,
}
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    applet_id: widget::Id,
    market_quotes: Vec<MarketQuote>,
    config: Config,
    current_index: usize,
    example_row: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    TogglePopup,
    PopupClosed(Id),
    UpdateConfig(Config),
    ToggleExampleRow(bool),
    MarketLoaded(Vec<MarketQuote>),
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.github.MiracleForge.cosmic-marketwatch";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let config = cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
            .map(|context| match Config::get_entry(&context) {
                Ok(config) => config,
                Err((_errors, config)) => config,
            })
            .unwrap_or_default();

        let count = config.count_stokes_at_once;

        let app = AppModel {
            core,
            popup: None,
            applet_id: widget::Id::unique(),
            market_quotes: Vec::new(),
            config,
            example_row: false,
            current_index: 0,
        };

        let task = Task::perform(fetch_most_active(count), |result| {
            cosmic::Action::App(Message::MarketLoaded(result.unwrap_or_default()))
        });

        (app, task)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let interval_minutes = self.config.panel_stoke_rotation_interval;

        // Periodic stoke rotation refresh
        let rotate = IcedSubscription::run_with_id(
            (std::any::TypeId::of::<Self>(), interval_minutes),
            async_stream::stream! {
                let interval = Duration::from_secs(6);
                loop {
                    tokio::time::sleep(interval).await;
                    yield Message::Tick;
                }
            },
        );

        Subscription::batch([rotate])
    }

    fn view(&self) -> Element<'_, Self::Message> {
        use cosmic::iced::Alignment;

        let content = if let Some(current_stoke) = self.market_quotes.get(self.current_index) {
            widget::row()
                .align_y(Alignment::Center)
                .spacing(12)
                .push(
                    widget::icon::from_name("display-symbolic")
                        .size(16)
                        .symbolic(true),
                )
                .push(widget::text(current_stoke.symbol.clone()))
                .push(widget::text(format!(
                    "({:.2}%)",
                    current_stoke.change_percent
                )))
        } else {
            widget::row()
                .align_y(Alignment::Center)
                .spacing(6)
                .push(
                    widget::icon::from_name("display-symbolic")
                        .size(16)
                        .symbolic(true),
                )
                .push(widget::text("Loading..."))
        };

        let button = widget::button::custom(content)
            .class(cosmic::theme::Button::AppletIcon)
            .on_press(Message::TogglePopup);

        widget::autosize::autosize(button, self.applet_id.clone()).into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let mut content = widget::list_column().padding(10).spacing(6);

        if self.market_quotes.is_empty() {
            content = content.add(widget::text("Loading market data..."));
        } else {
            for quote in &self.market_quotes {
                let row = format!(
                    "{}  ${:.2}  ({:.2}%)",
                    quote.symbol, quote.price, quote.change_percent
                );

                content = content.add(widget::text(row));
            }
        }

        self.core.applet.popup_container(content).into()
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup = Some(new_id);

                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );

                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);

                    get_popup(popup_settings)
                };
            }

            Message::Tick => {
                if !self.market_quotes.is_empty() {
                    self.current_index = (self.current_index + 1) % self.market_quotes.len();
                }
            }

            Message::MarketLoaded(data) => {
                self.market_quotes = data;
            }

            Message::UpdateConfig(config) => {
                self.config = config;
            }

            Message::ToggleExampleRow(toggled) => {
                self.example_row = toggled;
            }

            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
        }

        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}
