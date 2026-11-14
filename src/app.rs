// SPDX-License-Identifier: GPL-3.0-only
// TODO: REMOVE MY OWN SLOP
//TODO: CACHE HELPERS
use crate::components::applet::{self};

use crate::components::alertcard::notify_alert;
use crate::components::header::header;
use crate::components::infocard::infocard;
use crate::components::newscard::news_card;
use crate::components::overviewcard::overviewcard;
use crate::components::settingscard::settingscard;
use crate::components::walletcard::{Wallet, load_wallets, save_wallets};
use crate::config::{Config, PopupTab, RefreshInterval};
use crate::marketwatch::{
    AlertCondition, MarketCacheKey, MarketQuote, PriceAlert, ScreensTab, YahooNews,
    fetch_by_screeners, fetch_by_symbols, fetch_logo_handles, fetch_logos, fetch_news_for_symbols,
    format_publish_time, search_symbols,
};
use cosmic::applet::cosmic_panel_config::PanelAnchor;
use cosmic::cosmic_config::CosmicConfigEntry;
use cosmic::iced::Subscription;
use cosmic::iced::{Length, Limits, window::Id};
use cosmic::prelude::*;
use cosmic::{Action, widget};
use iced_winit::commands::popup::{destroy_popup, get_popup};

use std::collections::HashMap;
use std::time::{Duration, Instant};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const MAX_WALLETS: usize = 10;
pub const MAX_ASSETS_PER_WALLET: usize = 10;
pub const UI_DEBUG_MODE: bool = false;

#[allow(clippy::struct_excessive_bools)]
pub struct AppModel {
    active_tab: PopupTab,
    skeleton_phase: f32,
    news_error: Option<String>,
    quotes_loading: bool,
    news_loading: bool,
    screens_tab: ScreensTab,
    core: cosmic::Core,
    config_handler: Option<cosmic::cosmic_config::Config>,
    popup: Option<Id>,
    applet_id: widget::Id,
    market_quotes: Vec<MarketQuote>,
    pub logo_handles: HashMap<String, widget::image::Handle>,
    market_news: Vec<YahooNews>,
    news_per_symbol_input: String,
    config: Config,
    is_horizontal: bool,
    current_index: usize,
    error_message: Option<String>,
    wallets: Vec<Wallet>,
    current_wallet_index: usize,

    rename_mode: bool,
    rename_input: String,

    stock_search_input: String,
    stock_search_results: Vec<String>,
    stock_search_loading: bool,

    last_fetch_time: HashMap<MarketCacheKey, Instant>,
    cached_quotes: HashMap<MarketCacheKey, Vec<MarketQuote>>,
    cached_news: HashMap<MarketCacheKey, Vec<YahooNews>>,

    next_alert_id: u64,
    alert_selected_symbol: Option<String>,
    alert_selected_condition: AlertCondition,
    alert_input_value: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    SkeletonTick,
    SetTab(ScreensTab),

    RefreshMarket,
    TogglePopup,
    MarketLoaded(Result<Vec<MarketQuote>, String>),
    LogosLoaded(HashMap<String, Option<String>>),
    LogoHandlesLoaded(HashMap<String, Vec<u8>>),
    NewsLoaded(Result<Vec<YahooNews>, String>),
    PreviusWallet,
    NextWallet,
    NavigateTo(Option<PopupTab>),
    OpenExternalLink(String),
    ToggleShowOnlyIcon(bool),
    ToggleShowLogos(bool),
    ToggleShowNews(bool),
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

    ToggleAlertsEnabled(bool),
    // Stocks on wallet
    AddStockToWallet(String),
    RemoveStockFromWallet(String),

    // Alerts
    AddAlert {
        wallet_index: usize,
        symbol: String,
        condition: AlertCondition,
    },
    RemoveAlert {
        wallet_index: usize,
        alert_id: u64,
    },
    ToggleAlert {
        wallet_index: usize,
        alert_id: u64,
    },

    AlertSelectCondition(AlertCondition),
    AlertSetValue(String),

    // Autocomplete
    StockSearchInput(String),
    StockSearchResults(Result<Vec<String>, String>),
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.github.MiracleForge.cosmic-ext-marketwatch";

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

        let next_alert_id = wallets
            .iter()
            .flat_map(|w| &w.alerts)
            .map(|a| a.id)
            .max()
            .map_or(0, |max| max + 1);

        let count = config.count_stokes_at_once;
        let saved_index = config.last_wallet_index;

        let safe_index = if saved_index <= wallets.len() {
            saved_index
        } else {
            0
        };

        let task = if UI_DEBUG_MODE {
            Task::none()
        } else if safe_index > 0 {
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
            let screens_init = ScreensTab::MostActive;
            Task::perform(fetch_by_screeners(count, screens_init), |result| {
                Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
            })
        };

        let app = AppModel {
            core,
            config_handler,
            skeleton_phase: 0.0_f32,
            news_error: None,
            quotes_loading: true,
            news_loading: config.show_news,
            popup: None,
            active_tab: PopupTab::Overview,
            screens_tab: ScreensTab::MostActive,
            applet_id: widget::Id::unique(),
            market_quotes: Vec::new(),
            logo_handles: HashMap::new(),
            market_news: Vec::new(),
            news_per_symbol_input: config.count_news_by_simbol.to_string(),
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
            next_alert_id,
            alert_selected_symbol: None,
            alert_selected_condition: AlertCondition::PriceAbove(0.0),
            alert_input_value: String::new(),
        };

        (app, task)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let rotate = cosmic::iced::time::every(Duration::from_secs(
            self.config.panel_stoke_rotation_interval,
        ))
        .map(|_| Message::Tick);

        let refresh = cosmic::iced::time::every(Duration::from_secs(
            self.config.refresh_interval.as_seconds(),
        ))
        .map(|_| Message::RefreshMarket);

        let mut subs = vec![rotate, refresh];

        let loading = self.market_quotes.is_empty() && self.error_message.is_none();
        if loading {
            subs.push(
                cosmic::iced::time::every(Duration::from_millis(50)).map(|_| Message::SkeletonTick),
            );
        }

        Subscription::batch(subs)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = applet::build_applet_content(
            &self.config,
            &self.market_quotes,
            self.current_index,
            self.is_horizontal,
            self.error_message.as_ref(),
            &self.core,
        );

        widget::autosize::autosize(
            widget::button::custom(content)
                .class(cosmic::theme::Button::AppletIcon)
                .on_press(Message::TogglePopup),
            widget::Id::unique(),
        )
        .into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let last_updated = self
            .last_fetch_time
            .get(&self.current_cache_key())
            .map(|instant| {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .saturating_sub(instant.elapsed().as_secs());

                format_publish_time(timestamp)
            });

        let theme = self.core.system_theme();
        let header = header(last_updated, self.active_tab);

        // Maximum supported applet scale is 200%.
        let max_height = self
            .core
            .applet
            .suggested_bounds
            .map_or(500.0, |bounds| bounds.height);

        let body_height = (max_height - 80.0).min(520.0).max(120.0);
        let pulse = 0.5 + 0.5 * self.skeleton_phase.sin();

        let body = match self.active_tab {
            PopupTab::Overview => Some(
                widget::container(overviewcard(
                    theme,
                    &self.market_quotes,
                    self.error_message.as_ref(),
                    self.quotes_loading,
                    self.config.show_logos,
                    self.screens_tab,
                    &self.logo_handles,
                    pulse,
                ))
                .height(Length::Fixed(body_height)),
            ),
            PopupTab::Info => {
                Some(widget::container(infocard(theme)).height(Length::Fixed(body_height)))
            }
            PopupTab::Settings => Some(
                widget::container(settingscard(&self.config, theme))
                    .height(Length::Fixed(body_height)),
            ),
            PopupTab::News => Some(
                widget::container(news_card(
                    &self.market_news,
                    self.config.show_news,
                    self.error_message.as_ref(),
                    self.news_loading,
                    theme,
                ))
                .height(Length::Fixed(body_height)),
            ),
            _ => None,
        };

        let content = widget::column![widget::container(header).height(Length::Shrink),]
            .padding(0)
            .width(Length::Fill)
            .height(Length::Shrink)
            .push_maybe(body);

        self.core
            .applet
            .popup_container(content)
            .limits(
                Limits::NONE
                    .min_width(350.0)
                    .max_width(350.0)
                    .min_height(200.0)
                    .max_height(f32::INFINITY),
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

                    let max_height = self
                        .core
                        .applet
                        .suggested_bounds
                        .map_or(800.0, |b| (b.height - 20.0).max(300.0));

                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(480.0)
                        .min_width(480.0)
                        .min_height(200.0)
                        .max_height(max_height);

                    get_popup(popup_settings)
                };
            }

            Message::Tick => {
                if !self.market_quotes.is_empty() {
                    self.current_index = (self.current_index + 1) % self.market_quotes.len();
                }
            }

            Message::SkeletonTick => {
                self.skeleton_phase = (self.skeleton_phase + 0.15) % std::f32::consts::TAU;
            }

            Message::SetTab(tab) => {
                self.screens_tab = tab;

                let key = self.current_cache_key();
                let refresh_secs = self.config.refresh_interval.as_seconds();

                let cache_valid = self
                    .last_fetch_time
                    .get(&key)
                    .is_some_and(|t| t.elapsed().as_secs() < refresh_secs);

                if cache_valid {
                    self.restore_cached_market(&key);
                    return Task::none();
                }

                let count = self.config.count_stokes_at_once;

                return Task::perform(fetch_by_screeners(count, self.screens_tab), |result| {
                    Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                });
            }

            Message::MarketLoaded(result) => match result {
                Ok(data) => {
                    self.quotes_loading = false;
                    self.news_loading = self.config.show_news;
                    self.market_quotes = data;
                    self.error_message = None;

                    let key = self.current_cache_key();
                    self.last_fetch_time.insert(key.clone(), Instant::now());
                    self.cached_quotes
                        .insert(key.clone(), self.market_quotes.clone());

                    self.check_and_trigger_alerts();

                    let mut tasks = Vec::new();

                    if self.config.show_logos {
                        let logos_symbols: Vec<String> = self
                            .market_quotes
                            .iter()
                            .filter(|q| !self.logo_handles.contains_key(&q.symbol))
                            .map(|q| q.symbol.clone())
                            .collect();

                        if !logos_symbols.is_empty() {
                            tasks.push(Task::perform(fetch_logos(logos_symbols), |logos| {
                                Action::App(Message::LogosLoaded(logos))
                            }));
                        }
                    }

                    if self.config.show_news {
                        let news_symbols: Vec<String> = self
                            .market_quotes
                            .iter()
                            .take(3)
                            .map(|q| q.symbol.clone())
                            .collect();

                        tasks.push(Task::perform(
                            fetch_news_for_symbols(news_symbols, self.config.count_news_by_simbol),
                            |result| {
                                Action::App(Message::NewsLoaded(result.map_err(|e| e.to_string())))
                            },
                        ));
                    }

                    tasks.push(Task::done(Action::App(Message::Tick)));

                    return Task::batch(tasks);
                }

                Err(err) => {
                    self.finish_market_loading();
                    self.news_error = Some(err.clone());
                    self.error_message = Some(err);
                    self.market_quotes.clear();

                    self.invalidate_current_market_cache();

                    return Task::done(Action::App(Message::Tick));
                }
            },

            Message::LogosLoaded(logos) => {
                for quote in &mut self.market_quotes {
                    if let Some(logo) = logos.get(&quote.symbol) {
                        quote.logo_url = logo.clone();
                    }
                }

                return Task::perform(fetch_logo_handles(logos), |handles| {
                    Action::App(Message::LogoHandlesLoaded(handles))
                });
            }

            Message::LogoHandlesLoaded(handles) => {
                for (symbol, bytes) in handles {
                    let handle = widget::image::Handle::from_bytes(bytes);
                    self.logo_handles.insert(symbol, handle);
                }
            }

            Message::NewsLoaded(result) => match result {
                Ok(news) => {
                    self.market_news = news;
                    self.news_error = None;

                    let key = self.current_cache_key();
                    self.cached_news.insert(key, self.market_news.clone());
                }
                Err(err) => {
                    tracing::warn!("news fetch failed: {err}");
                    self.news_error = Some(err.to_string());
                }
            },

            Message::OpenExternalLink(url) => {
                let _ = std::process::Command::new("xdg-open").arg(url).spawn();
            }

            Message::RefreshMarket => {
                self.reset_market_view();
                self.invalidate_current_market_cache();

                if self.current_wallet_index > 0 {
                    if let Some(wallet) = self.wallets.get(self.current_wallet_index - 1)
                        && !wallet.symbols.is_empty()
                    {
                        let symbols = wallet.symbols.clone();

                        return Task::perform(fetch_by_symbols(symbols), |result| {
                            Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                        });
                    }

                    self.quotes_loading = false;
                    return Task::none();
                }

                let count = self.config.count_stokes_at_once;
                self.quotes_loading = true;

                return Task::perform(fetch_by_screeners(count, self.screens_tab), |result| {
                    Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                });
            }

            Message::SetRefreshInterval(interval) => {
                self.config.refresh_interval = interval;
                self.clear_all_market_cache();
                self.save_config();
            }

            Message::SetNumberOfNewsBySymbols(val) => {
                self.news_per_symbol_input.clone_from(&val);

                if let Ok(n) = val.parse::<u64>() {
                    let clamped = n.clamp(1, 5);
                    self.config.count_news_by_simbol = clamped;
                    self.save_config();

                    return self.update(Message::RefreshMarket);
                }
            }

            Message::SetStokeRotationInterval(val) => {
                if let Ok(n) = val.parse::<u64>() {
                    self.config.panel_stoke_rotation_interval = n;
                    self.save_config();
                }
            }

            Message::NavigateTo(tab) => {
                self.active_tab = tab.unwrap_or(PopupTab::Overview);
            }

            Message::ToggleShowOnlyIcon(new_value) => {
                self.config.show_only_icon = new_value;
                self.applet_id = widget::Id::unique();
                self.save_config();
            }

            Message::ToggleShowLogos(new_value) => {
                self.config.show_logos = new_value;
                self.save_config();

                if new_value {
                    let missing: Vec<String> = self
                        .market_quotes
                        .iter()
                        .filter(|q| !self.logo_handles.contains_key(&q.symbol))
                        .map(|q| q.symbol.clone())
                        .collect();

                    if !missing.is_empty() {
                        return Task::perform(fetch_logos(missing), |logos| {
                            Action::App(Message::LogosLoaded(logos))
                        });
                    }
                }
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

                self.market_news.clear();
            }

            Message::AddWallet => {
                if self.wallets.len() >= MAX_WALLETS {
                    return Task::none();
                }

                let index = self.wallets.len() + 1;
                self.wallets.push(Wallet::new(format!("Wallet {index}")));
                self.current_wallet_index = self.wallets.len();

                self.clear_market_data();
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
                self.market_news.clear();

                if self.active_tab == PopupTab::Alerts {
                    self.active_tab = PopupTab::Overview;
                    self.reset_alert_form();
                }

                self.config.last_wallet_index = safe_index;
                self.save_config();

                let refresh_secs = self.config.refresh_interval.as_seconds();
                let key = self.current_cache_key();

                let cache_valid = self
                    .last_fetch_time
                    .get(&key)
                    .is_some_and(|t| t.elapsed().as_secs() < refresh_secs);

                if cache_valid {
                    self.restore_cached_market(&key);
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

                return Task::perform(fetch_by_screeners(count, self.screens_tab), |result| {
                    Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                });
            }

            Message::PreviusWallet => {
                if self.active_tab == PopupTab::Alerts {
                    return Task::none();
                }

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
                if self.active_tab == PopupTab::Alerts {
                    return Task::none();
                }

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
                // TODO: DEBOUNCE how to make in rust ?
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

                self.stock_search_loading = false;
                self.stock_search_input = val;
                self.stock_search_results.clear();
            }

            Message::StockSearchResults(result) => {
                self.stock_search_loading = false;

                match result {
                    Ok(results) => {
                        self.stock_search_results = results;
                    }
                    Err(err) => {
                        tracing::warn!("Stock search failed: {err}");
                        self.stock_search_results.clear();
                    }
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

                    if wallet.symbols.contains(&symbol) {
                        return Task::none();
                    }

                    wallet.symbols.push(symbol);
                    save_wallets(&self.wallets);

                    self.invalidate_current_market_cache();

                    let symbols = self.wallets[self.current_wallet_index - 1].symbols.clone();

                    return Task::perform(fetch_by_symbols(symbols), |result| {
                        Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                    });
                }
            }

            Message::RemoveStockFromWallet(symbol) => {
                if self.current_wallet_index > 0 {
                    let wallet = &mut self.wallets[self.current_wallet_index - 1];

                    wallet.symbols.retain(|s| s != &symbol);
                    wallet.alerts.retain(|a| a.symbol != symbol);

                    save_wallets(&self.wallets);

                    if self.alert_selected_symbol.as_deref() == Some(&symbol) {
                        self.alert_selected_symbol = None;
                        self.alert_input_value.clear();
                        self.active_tab = PopupTab::Overview;
                    }

                    self.invalidate_current_market_cache();

                    let symbols = self.wallets[self.current_wallet_index - 1].symbols.clone();

                    if symbols.is_empty() {
                        self.clear_market_data();
                        return Task::none();
                    }

                    return Task::perform(fetch_by_symbols(symbols), |result| {
                        Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                    });
                }
            }

            Message::DeleteCurrentWallet => {
                if self.current_wallet_index > 0 {
                    self.invalidate_current_market_cache();

                    self.wallets.remove(self.current_wallet_index - 1);
                    self.current_wallet_index = 0;
                    save_wallets(&self.wallets);

                    self.active_tab = PopupTab::Overview;
                    self.reset_alert_form();

                    let count = self.config.count_stokes_at_once;

                    return Task::perform(fetch_by_screeners(count, self.screens_tab), |result| {
                        Action::App(Message::MarketLoaded(result.map_err(|e| e.to_string())))
                    });
                }
            }

            Message::ToggleAlertsEnabled(val) => {
                self.config.alerts_enabled = val;
                self.save_config();
            }

            Message::AddAlert {
                wallet_index,
                symbol,
                condition,
            } => {
                if wallet_index > 0
                    && let Some(wallet) = self.wallets.get_mut(wallet_index - 1)
                {
                    let id = self.next_alert_id;
                    self.next_alert_id += 1;

                    wallet.alerts.push(PriceAlert {
                        id,
                        symbol,
                        condition,
                        triggered: false,
                        enabled: true,
                    });

                    save_wallets(&self.wallets);
                }
            }

            Message::RemoveAlert {
                wallet_index,
                alert_id,
            } => {
                if wallet_index > 0
                    && let Some(wallet) = self.wallets.get_mut(wallet_index - 1)
                {
                    wallet.alerts.retain(|a| a.id != alert_id);
                    save_wallets(&self.wallets);
                }
            }

            Message::ToggleAlert {
                wallet_index,
                alert_id,
            } => {
                if wallet_index > 0
                    && let Some(wallet) = self.wallets.get_mut(wallet_index - 1)
                    && let Some(alert) = wallet.alerts.iter_mut().find(|a| a.id == alert_id)
                {
                    alert.enabled = !alert.enabled;
                    save_wallets(&self.wallets);
                }
            }

            Message::AlertSelectCondition(condition) => {
                self.alert_selected_condition = condition;
            }

            Message::AlertSetValue(val) => {
                self.alert_input_value = val;
            }
        }

        Task::none()
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

    fn current_cache_key(&self) -> MarketCacheKey {
        if self.current_wallet_index > 0 {
            MarketCacheKey::Wallet(self.current_wallet_index)
        } else {
            MarketCacheKey::Screen(self.screens_tab)
        }
    }

    fn check_and_trigger_alerts(&mut self) {
        let quotes: HashMap<&str, &MarketQuote> = self
            .market_quotes
            .iter()
            .map(|q| (q.symbol.as_str(), q))
            .collect();
        let notify = self.config.alerts_enabled;
        let mut changed = false;

        for wallet in &mut self.wallets {
            wallet.alerts.retain(|alert| {
                if !alert.enabled {
                    return true;
                }
                let Some(quote) = quotes.get(alert.symbol.as_str()) else {
                    return true;
                };
                if !alert.condition.is_triggered(quote) {
                    return true;
                }
                if notify {
                    notify_alert(alert, quote);
                }
                changed = true;
                false
            });
        }

        if changed {
            save_wallets(&self.wallets);
        }
    }

    fn invalidate_current_market_cache(&mut self) {
        let key = self.current_cache_key();

        self.last_fetch_time.remove(&key);
        self.cached_quotes.remove(&key);
        self.cached_news.remove(&key);
    }

    fn clear_all_market_cache(&mut self) {
        self.last_fetch_time.clear();
        self.cached_quotes.clear();
        self.cached_news.clear();
    }

    fn clear_market_data(&mut self) {
        self.market_quotes.clear();
        self.market_news.clear();
    }

    fn reset_market_view(&mut self) {
        self.current_index = 0;
        self.clear_market_data();
        self.error_message = None;
        self.news_error = None;
    }

    fn finish_market_loading(&mut self) {
        self.quotes_loading = false;
        self.news_loading = false;
    }

    fn restore_cached_market(&mut self, key: &MarketCacheKey) {
        if let Some(cached) = self.cached_quotes.get(key) {
            self.market_quotes = cached.clone();
        }

        if let Some(cached) = self.cached_news.get(key) {
            self.market_news = cached.clone();
        }
    }

    fn reset_alert_form(&mut self) {
        self.alert_selected_symbol = None;
        self.alert_input_value.clear();
        self.alert_selected_condition = AlertCondition::PriceAbove(0.0);
    }
}
