// SPDX-License-Identifier: GPL-3.0-only
use crate::config::{Config, PopupTab};
use crate::marketwatch::{MarketQuote, fetch_most_active};

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use cosmic::iced::{Alignment, Length, Limits, Subscription, window::Id};
use cosmic::iced_futures::Subscription as IcedSubscription;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::theme::{Button, Text};
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
        let mut content = widget::column().padding(0).spacing(0).width(Length::Fill);

        // ========== TOP ROW (Header com botões) ==========
        let top_row = widget::row()
            .spacing(8)
            .padding([8, 12])
            .align_y(cosmic::iced::Alignment::Center)
            .push(
                widget::button::icon(widget::icon::from_name("go-previous-symbolic"))
                    .on_press(Message::PreviusWallet)
                    .padding(6),
            )
            .push(
                widget::button::icon(widget::icon::from_name("go-next-symbolic"))
                    .on_press(Message::NextWallet)
                    .padding(6),
            )
            .push(widget::horizontal_space())
            .push(
                widget::button::icon(widget::icon::from_name("view-refresh-symbolic"))
                    .on_press(Message::RefreshMarket)
                    .padding(6),
            )
            .push(
                widget::button::icon(widget::icon::from_name("emblem-system-symbolic"))
                    .on_press(Message::OpenConfigBUtton)
                    .padding(6),
            );

        content = content.push(top_row);

        // ========== TABS (Overview, Trending, News) ==========
        let tabs = widget::column()
            .width(Length::Fill)
            .spacing(0)
            .push(
                widget::container(
                    widget::row()
                        .spacing(0)
                        .align_y(cosmic::iced::Alignment::Center)
                        .push(
                            widget::button::custom(widget::text("Overview").size(14))
                                .padding([12, 24])
                                .class(if self.active_tab == PopupTab::Overview {
                                    Button::Standard
                                } else {
                                    Button::Text
                                })
                                .on_press(Message::SelectedOverviewTab(PopupTab::Overview)),
                        )
                        .push(
                            widget::button::custom(widget::text("Trending").size(14))
                                .padding([12, 24])
                                .class(if self.active_tab == PopupTab::Trending {
                                    Button::Standard
                                } else {
                                    Button::Text
                                })
                                .on_press(Message::SelectedOverviewTab(PopupTab::Trending)),
                        )
                        .push(
                            widget::button::custom(widget::text("News").size(14))
                                .padding([12, 24])
                                .class(if self.active_tab == PopupTab::News {
                                    Button::Standard
                                } else {
                                    Button::Text
                                })
                                .on_press(Message::SelectedOverviewTab(PopupTab::News)),
                        ),
                )
                .style(|_theme: &cosmic::Theme| widget::container::Style {
                    background: Some(cosmic::iced::Color::from_rgba8(0, 0, 0, 0.15).into()),
                    ..Default::default()
                }),
            )
            .push(
                widget::container(widget::horizontal_space())
                    .width(Length::Fill)
                    .height(1)
                    .style(|theme: &cosmic::Theme| widget::container::Style {
                        background: Some(
                            cosmic::iced::Color::from(theme.cosmic().accent_color()).into(),
                        ),
                        ..Default::default()
                    }),
            )
            .push(
                widget::row()
                    .height(3)
                    .push(
                        widget::container(widget::horizontal_space())
                            .width(Length::FillPortion(1))
                            .height(3)
                            .style(move |theme: &cosmic::Theme| widget::container::Style {
                                background: if self.active_tab == PopupTab::Overview {
                                    Some(
                                        cosmic::iced::Color::from(theme.cosmic().accent_color())
                                            .into(),
                                    )
                                } else {
                                    None
                                },
                                ..Default::default()
                            }),
                    )
                    .push(
                        widget::container(widget::horizontal_space())
                            .width(Length::FillPortion(1))
                            .height(3)
                            .style(move |theme: &cosmic::Theme| widget::container::Style {
                                background: if self.active_tab == PopupTab::Trending {
                                    Some(
                                        cosmic::iced::Color::from(theme.cosmic().accent_color())
                                            .into(),
                                    )
                                } else {
                                    None
                                },
                                ..Default::default()
                            }),
                    )
                    .push(
                        widget::container(widget::horizontal_space())
                            .width(Length::FillPortion(1))
                            .height(3),
                    )
                    .push(
                        widget::container(widget::horizontal_space())
                            .width(Length::FillPortion(1))
                            .height(3)
                            .style(move |theme: &cosmic::Theme| widget::container::Style {
                                background: if self.active_tab == PopupTab::News {
                                    Some(
                                        cosmic::iced::Color::from(theme.cosmic().accent_color())
                                            .into(),
                                    )
                                } else {
                                    None
                                },
                                ..Default::default()
                            }),
                    ),
            );

        content = content.push(tabs);

        // ========== CONTEÚDO PRINCIPAL (scrollable) ==========
        let main_content = if self.market_quotes.is_empty() {
            widget::column()
                .padding(20)
                .push(widget::text("Loading market data...").size(14))
        } else {
            let mut col = widget::column().padding([16, 12]).spacing(16);

            if let Some(quote) = self.market_quotes.first() {
                let color = quote.variation_color();

                let main_card = widget::container(
                    widget::column()
                        .spacing(12)
                        .push(
                            widget::row()
                                .align_y(Alignment::Center)
                                .push(
                                    widget::icon::from_name("stock-symbolic")
                                        .size(24)
                                        .symbolic(true),
                                )
                                .push(widget::text(&quote.symbol).size(20))
                                .push(widget::horizontal_space())
                                .push(widget::text(quote.formatted_price()).size(20))
                                .push(widget::text(quote.formatted_variation()).size(16))
                                .push(
                                    widget::icon::from_name("go-next-symbolic")
                                        .size(16)
                                        .symbolic(true),
                                ),
                        )
                        .push(
                            widget::row()
                                .spacing(24)
                                .push(
                                    widget::column()
                                        .spacing(4)
                                        .push(widget::text("High").size(11))
                                        .push(
                                            widget::text(format!("${:.2}", quote.price * 1.04))
                                                .size(13),
                                        )
                                        .push(widget::text("+7.86%").size(10)),
                                )
                                .push(
                                    widget::column()
                                        .spacing(4)
                                        .push(widget::text("Low").size(11))
                                        .push(
                                            widget::text(format!("${:.2}", quote.price)).size(13),
                                        ),
                                )
                                .push(widget::horizontal_space())
                                .push(
                                    widget::column()
                                        .spacing(4)
                                        .align_x(cosmic::iced::alignment::Horizontal::Right)
                                        .push(widget::text("Market Cap").size(11))
                                        .push(widget::text("448.3B").size(13)),
                                ),
                        )
                        .push(
                            widget::container(widget::text("📈 Chart goes here").size(12))
                                .height(Length::Fixed(120.0))
                                .width(Length::Fill)
                                .style(|_theme: &cosmic::Theme| widget::container::Style {
                                    background: Some(
                                        cosmic::iced::Color::from_rgba8(255, 120, 0, 0.1).into(),
                                    ),
                                    ..Default::default()
                                }),
                        ),
                )
                .padding(16)
                .width(Length::Fill)
                .style(|theme: &cosmic::Theme| widget::container::Style {
                    background: Some(cosmic::iced::Color::from_rgba8(255, 120, 0, 0.08).into()),
                    border: cosmic::iced::Border {
                        color: cosmic::iced::Color::from(theme.cosmic().accent_color()),
                        width: 1.5,
                        radius: [8.0; 4].into(),
                    },
                    ..Default::default()
                });

                col = col.push(main_card);
            }

            // Top Gainers section
            col = col.push(
                widget::row()
                    .push(widget::text("Top Gainers").size(16))
                    .push(widget::horizontal_space())
                    .push(
                        widget::button::text("See All")
                            .class(Button::Link)
                            .on_press(Message::TogglePopup),
                    ),
            );

            // Top Gainers list
            for quote in self.market_quotes.iter().skip(1).take(3) {
                let color = quote.variation_color();

                let gainer_row = widget::container(
                    widget::row()
                        .align_y(Alignment::Center)
                        .spacing(12)
                        .push(
                            widget::icon::from_name("stock-symbolic")
                                .size(16)
                                .symbolic(true),
                        )
                        .push(widget::text(&quote.symbol).size(14))
                        .push(widget::text(quote.formatted_price()).size(14))
                        .push(widget::text(format!("+{:.2}%", quote.change_percent.abs())).size(12))
                        .push(widget::horizontal_space())
                        .push(
                            widget::text(format!("+{:.2}%", quote.change_percent.abs())).size(14),
                        ),
                )
                .padding([8, 12])
                .width(Length::Fill);

                col = col.push(gainer_row);
            }

            col = col.push(widget::text("Loading market data... 🔄").size(12));

            // Trending News section
            col = col.push(
                widget::row()
                    .push(widget::text("Trending News").size(16))
                    .push(widget::horizontal_space())
                    .push(
                        widget::button::text("See All")
                            .class(Button::Link)
                            .on_press(Message::TogglePopup),
                    ),
            );

            // News items
            col = col.push(
            widget::container(
                widget::column()
                    .spacing(4)
                    .push(
                        widget::row()
                            .spacing(8)
                            .push(
                                widget::icon::from_name("stock-symbolic")
                                    .size(14)
                                    .symbolic(true),
                            )
                            .push(
                                widget::text(
                                    "Rivian (RIVN) Reports Strong Q1 Earnings, Surpasses Expectations",
                                )
                                .size(13),
                            ),
                    )
                    .push(widget::text("3m ago").size(10)),
            )
            .padding([8, 12])
            .width(Length::Fill),
        );

            col = col.push(
            widget::container(
                widget::column()
                    .spacing(4)
                    .push(
                        widget::row()
                            .spacing(8)
                            .push(
                                widget::icon::from_name("stock-symbolic")
                                    .size(14)
                                    .symbolic(true),
                            )
                            .push(
                                widget::text(
                                    "Market Recap: Stocks Rally Amid Fed's Latest Interest Rate",
                                )
                                .size(13),
                            ),
                    )
                    .push(widget::text("1h ago").size(10)),
            )
            .padding([8, 12])
            .width(Length::Fill),
        );

            col = col.push(widget::text("Loading market data... 🔄").size(12));

            col
        };

        let scrollable = widget::scrollable(main_content).height(Length::Fill);

        content = content.push(scrollable);

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
