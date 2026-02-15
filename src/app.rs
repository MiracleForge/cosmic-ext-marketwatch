// SPDX-License-Identifier: GPL-3.0-only

use crate::config::Config;
use crate::fl;
use crate::marketwatch::{MarketQuote, fetch_most_active};

use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{Limits, Subscription, window::Id};
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::widget;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    market_quotes: Vec<MarketQuote>,
    config: Config,
    example_row: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
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
        let app = AppModel {
            core,
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => config,
                })
                .unwrap_or_default(),
            ..Default::default()
        };

        (app, Task::none())
    }

    fn view(&self) -> Element<'_, Self::Message> {
        self.core
            .applet
            .icon_button("display-symbolic")
            .on_press(Message::TogglePopup)
            .into()
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

    fn subscription(&self) -> Subscription<Self::Message> {
        self.core()
            .watch_config::<Config>(Self::APP_ID)
            .map(|update| Message::UpdateConfig(update.config))
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

                    Task::batch(vec![
                        Task::perform(fetch_most_active(), |result| {
                            cosmic::Action::App(Message::MarketLoaded(result.unwrap_or_default()))
                        }),
                        get_popup(popup_settings),
                    ])
                };
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
