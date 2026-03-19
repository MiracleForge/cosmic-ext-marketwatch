// SPDX-License-Identifier: GPL-3.0-only
use crate::components::applet::{self};

use crate::components::header::header;
use crate::components::maincard::maincard;
use crate::components::wallet::Wallet;
use crate::components::wallet::wallet::{load_wallets, save_wallets};
use crate::config::{Config, PopupTab, RefreshInterval};
use crate::marketwatch::{
    MarketQuote, YahooNews, fetch_by_symbols, fetch_most_active, fetch_news_for_symbols,
    format_publish_time, search_symbols, user_friendly_error_message,
};
use cosmic::applet::cosmic_panel_config::PanelAnchor;
use cosmic::cosmic_config::CosmicConfigEntry;
use cosmic::iced::{Length, Limits, Subscription, window::Id};
use cosmic::iced_futures::Subscription as IcedSubscription;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::{Action, widget};

use std::collections::HashMap;
use std::time::{Duration, Instant};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const MAX_WALLETS: usize = 10;
pub const MAX_ASSETS_PER_WALLET: usize = 10;

use std::sync::LazyLock;

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
    is_horizontal: bool,
    current_index: usize,
    error_message: Option<String>,
    wallets: Vec<Wallet>,
    current_wallet_index: usize, // 0 = Trending, 1+ = user wallets

    rename_mode: bool,
    rename_input: String,

    stock_search_input: String,
    stock_search_results: Vec<String>,
    stock_search_loading: bool,

    last_fetch_time: HashMap<usize, Instant>,

    cached_quotes: HashMap<usize, Vec<MarketQuote>>,

    cached_news: HashMap<usize, Vec<YahooNews>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    RefreshMarket,
    TogglePopup,
    MarketLoaded(Result<Vec<MarketQuote>, String>),
    NewsLoaded(Result<Vec<YahooNews>, String>),
    PreviusWallet,
    NextWallet,
    OpenConfigBUtton,
    OpenNewsLink(String),
    ToggleShowOnlyIcon(bool),
    ToggleShowNews(bool),
    ToggleNewsExpanded,
    SetRefreshInterval(RefreshInterval),
    SetNumberOfNewsBySymbols(String),
    SetStokeRotationInterval(String),

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
        let is_horizontal = matches!(core.applet.anchor, PanelAnchor::Top | PanelAnchor::Bottom);

        let wallets = load_wallets();

        let count = config.count_stokes_at_once;
        let saved_index = config.last_wallet_index;

        let safe_index = if saved_index <= wallets.len() {
            saved_index
        } else {
            0
        };

        let task = if safe_index > 0 {
            if let Some(wallet) = wallets.get(safe_index - 1) {
                if wallet.symbols.is_empty() {
                    Task::none()
                } else {
                    let symbols = wallet.symbols.clone();
                    Task::perform(fetch_by_symbols(symbols), |result| {
                        Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                    })
                }
            } else {
                Task::none()
            }
        } else {
            Task::perform(fetch_most_active(count), |result| {
                Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
            })
        };

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
            is_horizontal,
            current_index: 0,
            error_message: None,
            wallets,
            current_wallet_index: safe_index,
            rename_mode: false,
            rename_input: String::new(),
            stock_search_input: String::new(),
            stock_search_results: Vec::new(),
            stock_search_loading: false,
            last_fetch_time: HashMap::new(),
            cached_quotes: HashMap::new(),
            cached_news: HashMap::new(),
        };

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
        eprintln!("test {}", self.is_horizontal);
        dbg!(self.core.applet.anchor);
        let content = applet::build_applet_content(
            &self.config,
            &self.market_quotes,
            self.current_index,
            self.is_horizontal,
            self.error_message.as_ref(),
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
        let last_updated = self
            .last_fetch_time
            .get(&self.current_wallet_index)
            .map(|instant| {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .saturating_sub(instant.elapsed().as_secs());
                format_publish_time(timestamp)
            });

        let last_updated_ref = last_updated;

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
                self.wallets.len(),
                last_updated_ref,
            ))
            .push(maincard(
                self.active_tab,
                self.current_wallet_index,
                self.wallets
                    .get(self.current_wallet_index.saturating_sub(1))
                    .map_or(&[], |w| w.symbols.as_slice()),
                &self.market_quotes,
                &self.news_items,
                self.news_expanded,
                &self.config,
                self.error_message.as_ref(),
                &self.stock_search_input,
                &self.stock_search_results,
                self.stock_search_loading,
                self.wallets
                    .get(self.current_wallet_index.saturating_sub(1))
                    .is_some_and(|w| w.symbols.len() >= MAX_ASSETS_PER_WALLET),
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

    #[allow(clippy::too_many_lines)]
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
                    self.market_quotes = data;
                    self.error_message = None;

                    let idx = self.current_wallet_index;
                    self.last_fetch_time.insert(idx, Instant::now());
                    self.cached_quotes.insert(idx, self.market_quotes.clone());

                    if self.config.show_news {
                        let symbols: Vec<String> = self
                            .market_quotes
                            .iter()
                            .take(3)
                            .map(|q| q.symbol.clone())
                            .collect();

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
                    let idx = self.current_wallet_index;
                    self.last_fetch_time.remove(&idx);
                    self.cached_quotes.remove(&idx);
                    self.cached_news.remove(&idx);
                }
            },

            Message::NewsLoaded(result) => match result {
                Ok(news) => {
                    self.news_items = news;
                    let idx = self.current_wallet_index;
                    self.cached_news.insert(idx, self.news_items.clone());
                }
                Err(err) => {
                    tracing::warn!("{}", user_friendly_error_message(&err));
                }
            },

            Message::OpenNewsLink(url) => {
                let _ = std::process::Command::new("xdg-open").arg(url).spawn();
            }

            Message::RefreshMarket => {
                self.current_index = 0;
                self.market_quotes.clear();
                self.news_items.clear();
                self.error_message = None;

                let idx = self.current_wallet_index;
                self.last_fetch_time.remove(&idx);
                self.cached_quotes.remove(&idx);
                self.cached_news.remove(&idx);

                if self.current_wallet_index > 0 {
                    if let Some(wallet) = self.wallets.get(self.current_wallet_index - 1)
                        && !wallet.symbols.is_empty()
                    {
                        let symbols = wallet.symbols.clone();
                        return Task::perform(fetch_by_symbols(symbols), |result| {
                            Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                        });
                    }
                    return Task::none();
                }

                let count = self.config.count_stokes_at_once;
                return Task::perform(fetch_most_active(count), |result| {
                    Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                });
            }

            Message::SetRefreshInterval(interval) => {
                self.config.refresh_interval = interval;
                self.last_fetch_time.clear();
                self.cached_quotes.clear();
                self.cached_news.clear();
                self.save_config();
            }

            Message::SetNumberOfNewsBySymbols(val) => {
                if let Ok(n) = val.parse::<u64>() {
                    self.config.count_news_by_simbol = n;
                    self.save_config();
                }
            }

            Message::SetStokeRotationInterval(val) => {
                if let Ok(n) = val.parse::<u64>() {
                    self.config.panel_stoke_rotation_interval = n;
                    self.save_config();
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
                self.save_config();
            }

            Message::ToggleShowNews(new_value) => {
                self.config.show_news = new_value;
                self.save_config();

                if new_value {
                    let symbols: Vec<String> = self
                        .market_quotes
                        .iter()
                        .take(3)
                        .map(|q| q.symbol.clone())
                        .collect();

                    return Task::perform(
                        fetch_news_for_symbols(symbols, self.config.count_news_by_simbol),
                        |result| {
                            Action::App(Message::NewsLoaded(result.map_err(|e| e.to_string())))
                        },
                    );
                }
                self.news_items.clear();
            }

            Message::ToggleNewsExpanded => {
                self.news_expanded = !self.news_expanded;
            }

            Message::AddWallet => {
                if self.wallets.len() >= MAX_WALLETS {
                    return Task::none();
                }

                let index = self.wallets.len() + 1;
                self.wallets.push(Wallet::new(format!("Wallet {index}")));
                self.current_wallet_index = self.wallets.len();
                self.market_quotes.clear();
                self.news_items.clear();
                self.error_message = None;
                save_wallets(&self.wallets);
            }

            Message::SwitchWallet(index) => {
                let total = self.wallets.len() + 1;
                let safe_index = if index < total { index } else { 0 };

                self.current_wallet_index = safe_index;
                self.stock_search_input.clear();
                self.stock_search_results.clear();
                self.rename_mode = false;
                self.error_message = None;
                self.news_items.clear();

                self.config.last_wallet_index = safe_index;
                self.save_config();

                let refresh_secs = self.config.refresh_interval.as_seconds();
                let cache_valid = self
                    .last_fetch_time
                    .get(&safe_index)
                    .is_some_and(|t| t.elapsed().as_secs() < refresh_secs);

                if cache_valid {
                    if let Some(cached) = self.cached_quotes.get(&safe_index) {
                        self.market_quotes = cached.clone();
                    }
                    if let Some(cached) = self.cached_news.get(&safe_index) {
                        self.news_items = cached.clone();
                    }
                    return Task::none();
                }

                self.market_quotes.clear();

                if safe_index > 0 {
                    if let Some(wallet) = self.wallets.get(safe_index - 1)
                        && !wallet.symbols.is_empty()
                    {
                        let symbols = wallet.symbols.clone();
                        return Task::perform(fetch_by_symbols(symbols), |result| {
                            Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                        });
                    }
                    return Task::none();
                }

                let count = self.config.count_stokes_at_once;
                return Task::perform(fetch_most_active(count), |result| {
                    Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                });
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
                if self.current_wallet_index > 0 {
                    self.rename_mode = !self.rename_mode;
                    if self.rename_mode {
                        self.rename_input =
                            self.wallets[self.current_wallet_index - 1].name.clone();
                    }
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
                //TODO: DEBOUNCE how to make in rust ?
                if val.len() >= 2 {
                    self.stock_search_loading = true;
                    let query = val.clone();
                    self.stock_search_input = val;
                    return Task::perform(search_symbols(query), |result| {
                        Action::App(Message::StockSearchResults(
                            result.map_err(|e| e.to_string()),
                        ))
                    });
                }
                self.stock_search_input = val;
                self.stock_search_results.clear();
            }

            Message::StockSearchResults(result) => {
                self.stock_search_loading = false;
                if let Ok(results) = result {
                    self.stock_search_results = results;
                }
            }

            Message::AddStockToWallet(symbol_label) => {
                let is_valid = self.stock_search_results.iter().any(|r| r == &symbol_label);

                if !is_valid {
                    return Task::none();
                }

                let symbol = symbol_label
                    .split(" — ")
                    .next()
                    .unwrap_or(&symbol_label)
                    .to_string();

                self.stock_search_input.clear();
                self.stock_search_results.clear();

                if self.current_wallet_index > 0 {
                    let wallet = &mut self.wallets[self.current_wallet_index - 1];
                    if wallet.symbols.len() >= MAX_ASSETS_PER_WALLET {
                        return Task::none();
                    }
                    if !wallet.symbols.contains(&symbol) {
                        wallet.symbols.push(symbol);
                        save_wallets(&self.wallets);
                    }

                    // new symbol , invalid cache for this wallet
                    let idx = self.current_wallet_index;
                    self.last_fetch_time.remove(&idx);
                    self.cached_quotes.remove(&idx);
                    self.cached_news.remove(&idx);

                    let symbols = self.wallets[self.current_wallet_index - 1].symbols.clone();
                    return Task::perform(fetch_by_symbols(symbols), |result| {
                        Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                    });
                }
            }

            Message::RemoveStockFromWallet(symbol) => {
                if self.current_wallet_index > 0 {
                    self.wallets[self.current_wallet_index - 1]
                        .symbols
                        .retain(|s| s != &symbol);
                    save_wallets(&self.wallets);

                    let idx = self.current_wallet_index;
                    self.last_fetch_time.remove(&idx);
                    self.cached_quotes.remove(&idx);
                    self.cached_news.remove(&idx);

                    let symbols = self.wallets[self.current_wallet_index - 1].symbols.clone();

                    if symbols.is_empty() {
                        self.market_quotes.clear();
                        self.news_items.clear();
                        return Task::none();
                    }

                    return Task::perform(fetch_by_symbols(symbols), |result| {
                        Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                    });
                }
            }

            Message::DeleteCurrentWallet => {
                if self.current_wallet_index > 0 {
                    let idx = self.current_wallet_index;
                    self.last_fetch_time.remove(&idx);
                    self.cached_quotes.remove(&idx);
                    self.cached_news.remove(&idx);

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
    fn save_config(&self) {
        if let Some(ref handler) = self.config_handler
            && let Err(e) = self.config.write_entry(handler)
        {
            tracing::error!("Failed to save config: {}", e);
        }
    }
}
