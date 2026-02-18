// SPDX-License-Identifier: GPL-3.0-only
use crate::components::applet::{self};
use crate::components::header::header;
use crate::components::maincard::maincard;
use crate::config::{Config, PopupTab};
use crate::marketwatch::{MarketQuote, fetch_most_active};

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use cosmic::iced::{Length, Limits, Subscription, window::Id};
use cosmic::iced_futures::Subscription as IcedSubscription;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::{Action, widget};

use std::time::Duration;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct AppModel {
    active_tab: PopupTab,
    core: cosmic::Core,
    config_handler: Option<cosmic::cosmic_config::Config>,
    popup: Option<Id>,
    applet_id: widget::Id,
    market_quotes: Vec<MarketQuote>,
    config: Config,
    current_index: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    RefreshMarket,
    TogglePopup,
    PopupClosed(Id),
    UpdateConfig(Config),
    MarketLoaded(Vec<MarketQuote>),
    PreviusWallet,
    NextWallet,
    SelectedOverviewTab(PopupTab),
    OpenConfigBUtton,
    ToggleShowOnlyIcon(bool),
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
        let config_handler = cosmic::cosmic_config::Config::new(Self::APP_ID, Config::VERSION).ok();
        let config = config_handler
            .as_ref()
            .and_then(|h| Config::get_entry(h).ok())
            .unwrap_or_default();

        let count = config.count_stokes_at_once;

        let app = AppModel {
            core,
            config_handler,
            popup: None,
            active_tab: PopupTab::Overview,
            applet_id: widget::Id::unique(),
            market_quotes: Vec::new(),
            config,
            current_index: 0,
        };

        let task = Task::perform(fetch_most_active(count), |result| {
            cosmic::Action::App(Message::MarketLoaded(result.unwrap_or_default()))
        });

        (app, task)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let interval_minutes = self.config.panel_stoke_rotation_interval;
        let refresh_interval = self.config.refresh_interval.as_seconds();

        let rotate = IcedSubscription::run_with_id(
            (std::any::TypeId::of::<Self>(), "rotate", interval_minutes),
            async_stream::stream! {
                let interval = Duration::from_secs(interval_minutes);
                loop {
                    tokio::time::sleep(interval).await;
                    yield Message::Tick;
                }
            },
        );

        let refresh = IcedSubscription::run_with_id(
            (std::any::TypeId::of::<Self>(), "refresh", refresh_interval),
            async_stream::stream! {
                let interval = Duration::from_secs(refresh_interval);
                tokio::time::sleep(interval).await;
                loop {
                    yield Message::RefreshMarket;
                    tokio::time::sleep(interval).await;
                }
            },
        );

        Subscription::batch([rotate, refresh])
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content =
            applet::build_applet_content(&self.config, &self.market_quotes, self.current_index);

        widget::autosize::autosize(
            widget::button::custom(content)
                .class(cosmic::theme::Button::AppletIcon)
                .on_press(Message::TogglePopup),
            self.applet_id.clone(),
        )
        .into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let content = widget::column()
            .padding(0)
            .spacing(6)
            .width(Length::Fill)
            .push(header())
            .push(maincard(self.active_tab, &self.market_quotes, &self.config));

        self.core.applet.popup_container(content).into()
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    self.active_tab = PopupTab::Overview;
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

            Message::RefreshMarket => {
                let count = self.config.count_stokes_at_once;
                return Task::perform(fetch_most_active(count), |result| {
                    Action::App(Message::MarketLoaded(result.unwrap_or_default()))
                });
            }

            Message::UpdateConfig(config) => {
                self.config = config;
            }

            Message::PreviusWallet => {
                println!("Change to previous user collection of stocks");
            }

            Message::NextWallet => {
                println!("Change to next user collection of stocks");
            }

            Message::OpenConfigBUtton => {
                self.active_tab = match self.active_tab {
                    PopupTab::Settings => PopupTab::Overview,
                    _ => PopupTab::Settings,
                };
            }

            Message::ToggleShowOnlyIcon(new_value) => {
                self.config.show_only_icon = new_value;
                self.applet_id = widget::Id::unique(); // reset do cache do autosize
                self::AppModel::save_config(&self);
            }

            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                    self.active_tab = PopupTab::Overview;
                }
            }

            Message::SelectedOverviewTab(tab) => {
                self.active_tab = tab;
            }
        }

        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}

impl AppModel {
    // by cosmic-ext-applet-tempest
    fn save_config(&self) {
        if let Some(ref handler) = self.config_handler {
            if let Err(e) = self.config.write_entry(handler) {
                tracing::error!("Failed to save config: {}", e);
            }
        }
    }
}
