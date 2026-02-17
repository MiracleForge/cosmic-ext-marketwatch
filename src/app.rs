use crate::components::header::header;
use crate::components::maincard::maincard;
// SPDX-License-Identifier: GPL-3.0-only
use crate::config::{Config, PopupTab};
use crate::marketwatch::{MarketQuote, fetch_most_active};

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use cosmic::iced::{Alignment, Length, Limits, Subscription, window::Id};
use cosmic::iced_futures::Subscription as IcedSubscription;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::theme::Text;
use cosmic::{Action, widget};

use std::time::Duration;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct AppModel {
    active_tab: PopupTab,
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
    RefreshMarket,
    TogglePopup,
    PopupClosed(Id),
    UpdateConfig(Config),
    ToggleExampleRow(bool),
    MarketLoaded(Vec<MarketQuote>),
    PreviusWallet,
    NextWallet,
    SelectedOverviewTab(PopupTab),
    OpenConfigBUtton,
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
            active_tab: PopupTab::Overview,
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
        let refresh_interval = self.config.refresh_interval.as_seconds();
        // Periodic stoke rotation refresh
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
        let content = if let Some(current_stoke) = self.market_quotes.get(self.current_index) {
            let color = current_stoke.variation_color();
            widget::row()
                .align_y(Alignment::Center)
                .spacing(12)
                .width(cosmic::iced::Length::Fixed(250.0))
                .push(
                    widget::icon::from_name("org.gnome.PowerStats-symbolic")
                        .size(16)
                        .symbolic(true),
                )
                .push(widget::horizontal_space().width(cosmic::iced::Length::Fill))
                .push(widget::text::heading(current_stoke.symbol.clone()))
                .push(widget::horizontal_space().width(cosmic::iced::Length::Fill))
                .push(widget::text(current_stoke.formatted_price()).class(Text::Color(color)))
                .push(widget::horizontal_space().width(cosmic::iced::Length::Fill))
                .push(widget::text(current_stoke.formatted_variation()).class(Text::Color(color)))
        } else {
            widget::row()
                .align_y(Alignment::Center)
                .spacing(16)
                .width(cosmic::iced::Length::Fixed(250.0))
                .push(
                    widget::icon::from_name("process-working-symbolic")
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
        let mut content = widget::column().padding(0).spacing(6).width(Length::Fill);

        content = content.push(header());
        content = content.push(maincard(&self.market_quotes));

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
                println!("Changue to previeus user collection of stokes");
            }

            Message::NextWallet => {
                println!("Change to next user cllection of stokes");
            }

            Message::OpenConfigBUtton => {
                println!("Opening config button");
            }

            Message::ToggleExampleRow(toggled) => {
                self.example_row = toggled;
            }

            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
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
