// SPDX-License-Identifier: GPL-3.0-only
use crate::components::applet::{self};
use crate::components::header::header;
use crate::components::maincard::maincard;
use crate::components::wallet::wallet::{load_wallets, save_wallets};
use crate::components::wallet::{Wallet, wallet};
use crate::config::{Config, PopupTab, RefreshInterval};
use crate::marketwatch::{
    MarketQuote, YahooNews, fetch_by_symbols, fetch_most_active, fetch_news_for_symbols,
    search_symbols, user_friendly_error_message,
};

use cosmic::cosmic_config::CosmicConfigEntry;
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
    news_items: Vec<YahooNews>,
    news_expanded: bool,
    config: Config,
    current_index: usize,
    error_message: Option<String>,
    wallets: Vec<Wallet>,
    current_wallet_index: usize, // 0 = Trending, 1+ = user wallets

    rename_mode: bool,
    rename_input: String,

    stock_search_input: String,
    stock_search_results: Vec<String>,
    stock_search_loading: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    RefreshMarket,
    TogglePopup,
    PopupClosed(Id),
    UpdateConfig(Config),
    MarketLoaded(Result<Vec<MarketQuote>, String>),
    NewsLoaded(Result<Vec<YahooNews>, String>),
    PreviusWallet,
    NextWallet,
    SelectedOverviewTab(PopupTab),
    OpenConfigBUtton,
    OpenNewsLink(String),
    ToggleShowOnlyIcon(bool),
    ToggleShowNews(bool),
    ToggleNewsExpanded,
    SetRefreshInterval(RefreshInterval),
    SetNumberOfNewsBySymbols(String),

    // wallet navegation
    SwitchWallet(usize),

    // wallet management
    AddWallet,
    DeleteCurrentWallet,
    RenameWallet(String),
    ConfirmRenameWallet,
    ToggleRenameMode,

    // Stocks on wallet
    AddStockToWallet(String),
    RemoveStockFromWallet(String),

    // Autocomplete
    StockSearchInput(String),
    StockSearchResults(Result<Vec<String>, String>),
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
            news_items: Vec::new(),
            news_expanded: false,
            config,
            current_index: 0,
            error_message: None,
            wallets: load_wallets(),
            current_wallet_index: 0,
            rename_mode: false,
            rename_input: String::new(),
            stock_search_input: String::new(),
            stock_search_results: Vec::new(),
            stock_search_loading: false,
        };

        let task = Task::perform(fetch_most_active(count), |result| {
            cosmic::Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
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
        let content = applet::build_applet_content(
            &self.config,
            &self.market_quotes,
            self.current_index,
            &self.error_message,
        );

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
            .push(header(
                self.current_wallet_index,
                self.wallets
                    .get(self.current_wallet_index.saturating_sub(1))
                    .map(|w| w.name.as_str()),
                self.rename_mode,
                &self.rename_input,
            ))
            .push(maincard(
                self.active_tab,
                self.current_wallet_index,
                self.wallets
                    .get(self.current_wallet_index.saturating_sub(1))
                    .map(|w| w.symbols.as_slice())
                    .unwrap_or(&[]),
                &self.market_quotes,
                &self.news_items,
                self.news_expanded,
                &self.config,
                &self.error_message,
                &self.stock_search_input,
                &self.stock_search_results,
            ));

        self.core
            .applet
            .popup_container(content)
            .limits(
                Limits::NONE
                    .min_width(480.0)
                    .max_width(480.0)
                    .min_height(200.0)
                    .max_height(1080.0),
            )
            .into()
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

            Message::MarketLoaded(result) => match result {
                Ok(data) => {
                    let symbols: Vec<String> =
                        data.iter().take(3).map(|q| q.symbol.clone()).collect();
                    self.market_quotes = data;
                    self.error_message = None;

                    if self.config.show_news {
                        return Task::perform(
                            fetch_news_for_symbols(symbols, self.config.count_news_by_simbol),
                            |result| {
                                Action::App(Message::NewsLoaded(result.map_err(|e| e.to_string())))
                            },
                        );
                    }
                }
                Err(err) => {
                    self.error_message = Some(err);
                    self.market_quotes.clear();
                }
            },

            Message::NewsLoaded(result) => match result {
                Ok(news) => {
                    self.news_items = news;
                }
                Err(err) => {
                    tracing::warn!("{}", user_friendly_error_message(&err));
                }
            },

            Message::OpenNewsLink(url) => {
                let _ = std::process::Command::new("xdg-open").arg(url).spawn();
            }

            Message::RefreshMarket => {
                let count = self.config.count_stokes_at_once;
                return Task::perform(fetch_most_active(count), |result| {
                    Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                });
            }

            Message::UpdateConfig(config) => {
                self.config = config;
            }

            Message::SetRefreshInterval(interval) => {
                self.config.refresh_interval = interval;
                self::AppModel::save_config(&self);
            }

            Message::SetNumberOfNewsBySymbols(val) => {
                if let Ok(n) = val.parse::<u64>() {
                    self.config.count_news_by_simbol = n;
                    self::AppModel::save_config(&self);
                }
            }

            Message::OpenConfigBUtton => {
                self.active_tab = match self.active_tab {
                    PopupTab::Settings => PopupTab::Overview,
                    _ => PopupTab::Settings,
                };
            }

            Message::ToggleShowOnlyIcon(new_value) => {
                self.config.show_only_icon = new_value;
                self.applet_id = widget::Id::unique();
                self::AppModel::save_config(&self);
            }

            Message::ToggleShowNews(new_value) => {
                self.config.show_news = new_value;
                if new_value {
                    let symbols: Vec<String> = self
                        .market_quotes
                        .iter()
                        .take(3)
                        .map(|q| q.symbol.clone())
                        .collect();
                    self::AppModel::save_config(&self);
                    return Task::perform(
                        fetch_news_for_symbols(symbols, self.config.count_news_by_simbol),
                        |result| {
                            Action::App(Message::NewsLoaded(result.map_err(|e| e.to_string())))
                        },
                    );
                } else {
                    self.news_items.clear();
                    self::AppModel::save_config(&self);
                }
            }

            Message::ToggleNewsExpanded => {
                self.news_expanded = !self.news_expanded;
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

            Message::AddWallet => {
                let index = self.wallets.len() + 1;
                self.wallets
                    .push(Wallet::new(format!("Carteira {}", index)));
                self.current_wallet_index = self.wallets.len();
                self.market_quotes.clear();
                self.error_message = None; // <-- adicione isso
                save_wallets(&self.wallets);
            }

            Message::SwitchWallet(index) => {
                self.current_wallet_index = index;
                self.stock_search_input.clear();
                self.stock_search_results.clear();
                self.rename_mode = false;
                self.error_message = None;

                // 🔥 IMPORTANTE: limpa dados antigos
                self.market_quotes.clear();
                self.news_items.clear();

                if index > 0 {
                    if let Some(wallet) = self.wallets.get(index - 1) {
                        if !wallet.symbols.is_empty() {
                            let symbols = wallet.symbols.clone();

                            return Task::perform(fetch_by_symbols(symbols), |result| {
                                Action::App(Message::MarketLoaded(
                                    result.map_err(|e| e.to_string()),
                                ))
                            });
                        }
                    }

                    // carteira vazia
                    return Task::none();
                } else {
                    let count = self.config.count_stokes_at_once;

                    return Task::perform(fetch_most_active(count), |result| {
                        Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                    });
                }
            }
            Message::PreviusWallet => {
                let total = self.wallets.len() + 1;
                if total <= 1 {
                    return Task::none();
                }
                let new_index = if self.current_wallet_index == 0 {
                    total - 1
                } else {
                    self.current_wallet_index - 1
                };
                return self.update(Message::SwitchWallet(new_index));
            }

            Message::NextWallet => {
                let total = self.wallets.len() + 1;
                if total <= 1 {
                    return Task::none();
                }
                let new_index = (self.current_wallet_index + 1) % total;
                return self.update(Message::SwitchWallet(new_index));
            }

            Message::ToggleRenameMode => {
                self.rename_mode = !self.rename_mode;
                if self.rename_mode && self.current_wallet_index > 0 {
                    self.rename_input = self.wallets[self.current_wallet_index - 1].name.clone();
                }
            }

            Message::RenameWallet(val) => {
                self.rename_input = val;
            }

            Message::ConfirmRenameWallet => {
                if self.current_wallet_index > 0 && !self.rename_input.trim().is_empty() {
                    self.wallets[self.current_wallet_index - 1].name =
                        self.rename_input.trim().to_string();
                    save_wallets(&self.wallets);
                }
                self.rename_mode = false;
            }

            Message::StockSearchInput(val) => {
                self.stock_search_input = val.clone();
                if val.len() >= 2 {
                    self.stock_search_loading = true;
                    return Task::perform(search_symbols(val), |result| {
                        Action::App(Message::StockSearchResults(
                            result.map_err(|e| e.to_string()),
                        ))
                    });
                } else {
                    self.stock_search_results.clear();
                }
            }

            Message::StockSearchResults(result) => {
                self.stock_search_loading = false;
                if let Ok(results) = result {
                    self.stock_search_results = results;
                }
            }

            Message::AddStockToWallet(symbol_label) => {
                let symbol = symbol_label
                    .split(" — ")
                    .next()
                    .unwrap_or(&symbol_label)
                    .to_string();
                if self.current_wallet_index > 0 {
                    let wallet = &mut self.wallets[self.current_wallet_index - 1];
                    if !wallet.symbols.contains(&symbol) {
                        wallet.symbols.push(symbol);
                        save_wallets(&self.wallets);
                    }
                    // refaz o fetch com os símbolos atualizados
                    let symbols = self.wallets[self.current_wallet_index - 1].symbols.clone();
                    self.stock_search_input.clear();
                    self.stock_search_results.clear();
                    return Task::perform(fetch_by_symbols(symbols), |result| {
                        Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                    });
                }
                self.stock_search_input.clear();
                self.stock_search_results.clear();
            }

            Message::RemoveStockFromWallet(symbol) => {
                if self.current_wallet_index > 0 {
                    self.wallets[self.current_wallet_index - 1]
                        .symbols
                        .retain(|s| s != &symbol);
                    save_wallets(&self.wallets);
                }
            }

            Message::DeleteCurrentWallet => {
                if self.current_wallet_index > 0 {
                    self.wallets.remove(self.current_wallet_index - 1);
                    self.current_wallet_index = 0;
                    save_wallets(&self.wallets);

                    let count = self.config.count_stokes_at_once;
                    return Task::perform(fetch_most_active(count), |result| {
                        Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                    });
                }
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
