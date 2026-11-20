// SPDX-License-Identifier: GPL-3.0-only
use crate::components::applet::{self};

use crate::components::alertcard::notify_alert;
use crate::components::header::{header, tools_header};
use crate::components::infocard::infocard;
use crate::components::newscard::news_card;
use crate::components::overviewcard::overviewcard;
use crate::components::pinnedcard::pinnedcard;
use crate::components::portfoliocard::{
    PortFolio, PortfolioView, QuoteStyle, SearchView, load_portfolio, portfoliocard,
    save_portfolios,
};
use crate::components::settingscard::settingscard;
use crate::components::toolscard::{NewAlert, SelectedAsset, ToolsView, toolscard};
use crate::config::{Config, PopupTab, RefreshInterval};
use crate::constants::{MAX_ASSETS_PER_PORTFOLIO, MAX_PORTFOLIOS, NEWS_SYMBOLS_LIMIT};
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

pub const UI_DEBUG_MODE: bool = false;

#[allow(clippy::struct_excessive_bools)]
pub struct AppModel {
    applet_id: widget::Id,
    popup: Option<Id>,
    config_handler: Option<cosmic::cosmic_config::Config>,
    config: Config,
    core: cosmic::Core,
    active_tab: PopupTab,
    screens_tab: ScreensTab,
    is_horizontal: bool,

    skeleton_phase: f32,

    market_quotes: Vec<MarketQuote>,
    quotes_loading: bool,
    pub logo_handles: HashMap<String, widget::image::Handle>,

    portfolio_quotes: Vec<MarketQuote>,
    portfolio_loading: bool,
    portfolio_error: Option<String>,

    market_news: Vec<YahooNews>,
    news_per_symbol_input: String,
    news_loading: bool,
    news_error: Option<String>,

    current_index: usize,
    error_message: Option<String>,

    user_portfolios: Vec<PortFolio>,
    /// 0 = no portf selected; 1..=N = port selected
    current_portfolio_index: usize,
    portfolio_current_index: usize,

    rename_port_mode: bool,
    rename_port_input: String,

    stock_search_input: String,
    stock_search_results: Vec<String>,
    stock_search_loading: bool,

    last_fetch_time: HashMap<MarketCacheKey, Instant>,
    cached_quotes: HashMap<MarketCacheKey, Vec<MarketQuote>>,
    cached_news: HashMap<MarketCacheKey, Vec<YahooNews>>,

    next_alert_id: u64,
    alert_enabled: bool,
    tools_selected_asset: Option<SelectedAsset>,
    alert_selected_condition: AlertCondition,
    alert_input_value: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    SkeletonTick,
    SetTab(ScreensTab),

    RefreshMarket,
    RefreshPortfolio,
    TogglePopup,
    MarketLoaded(MarketCacheKey, Result<Vec<MarketQuote>, String>),
    LogosLoaded(HashMap<String, Option<String>>),
    LogoHandlesLoaded(HashMap<String, Vec<u8>>),
    NewsLoaded(MarketCacheKey, Result<Vec<YahooNews>, String>),
    PreviusPortfolio,
    NextPortfolio,
    NavigateTo(Option<PopupTab>),
    NavigateToTools(SelectedAsset),
    OpenExternalLink(String),
    ToggleShowOnlyIcon(bool),
    ToggleShowLogos(bool),
    ToggleShowNews(bool),
    SetRefreshInterval(RefreshInterval),
    SetNumberOfNewsBySymbols(String),
    SetStokeRotationInterval(String),
    // wallet navegation
    SwitchPortfolio(usize),
    // wallet management
    AddPortfolio,
    DeleteCurrentPortfolio,
    RenamePortfolio(String),
    ConfirmRenamePortfolio,
    ToggleRenameMode,
    TogglePinPortfolio,
    ToggleAlertsEnabled(bool),
    // Stocks on wallet
    AddStockToPortfolio(String),
    RemoveStockFromPortfolio(String),
    // Alerts
    AddAlert {
        portfolio_index: usize,
        alert: NewAlert,
    },

    AlertSetEnabled(bool),
    RemoveAlert {
        portfolio_index: usize,
        alert_id: u64,
    },
    ToggleAlert {
        portfolio_index: usize,
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

        let user_portfolios = load_portfolio();

        let next_alert_id = user_portfolios
            .iter()
            .flat_map(|w| &w.alerts)
            .map(|a| a.id)
            .max()
            .map_or(0, |max| max + 1);

        let saved_index = config.last_portfolio_index;
        let safe_index = if saved_index <= user_portfolios.len() {
            saved_index
        } else {
            0
        };

        let pinned_index = config
            .pinned_portfolio_index
            .filter(|&index| index > 0 && index <= user_portfolios.len());

        let initial_portfolio_index = pinned_index.unwrap_or(safe_index);

        let initial_tab = if pinned_index.is_some() {
            PopupTab::Pinned
        } else {
            PopupTab::Overview
        };

        let mut app = AppModel {
            core,
            config_handler,
            skeleton_phase: 0.0_f32,
            news_error: None,
            quotes_loading: false,
            news_loading: false,
            popup: None,
            active_tab: initial_tab,
            screens_tab: ScreensTab::MostActive,
            applet_id: widget::Id::unique(),
            market_quotes: Vec::new(),
            portfolio_quotes: Vec::new(),
            portfolio_loading: false,
            portfolio_error: None,
            logo_handles: HashMap::new(),
            market_news: Vec::new(),
            news_per_symbol_input: config.count_news_by_simbol.to_string(),
            config,
            is_horizontal,
            current_index: 0,
            error_message: None,
            user_portfolios,
            current_portfolio_index: initial_portfolio_index,
            portfolio_current_index: 0,
            rename_port_mode: false,
            rename_port_input: String::new(),
            stock_search_input: String::new(),
            stock_search_results: Vec::new(),
            stock_search_loading: false,
            last_fetch_time: HashMap::new(),
            cached_quotes: HashMap::new(),
            cached_news: HashMap::new(),
            next_alert_id,
            alert_enabled: true,
            tools_selected_asset: None,
            alert_selected_condition: AlertCondition::PriceAbove(0.0),
            alert_input_value: String::new(),
        };

        // US market first then portfolio in parallel
        let task = if UI_DEBUG_MODE {
            Task::none()
        } else {
            app.news_loading = app.config.show_news;
            Task::batch([app.fetch_market(), app.fetch_current_portfolio()])
        };

        (app, task)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let rotate = cosmic::iced::time::every(Duration::from_secs(
            self.config.panel_stoke_rotation_interval,
        ))
        .map(|_| Message::Tick);

        let refresh_market = cosmic::iced::time::every(Duration::from_secs(
            self.config.refresh_interval.as_seconds(),
        ))
        .map(|_| Message::RefreshMarket);

        let refresh_portfolio = cosmic::iced::time::every(Duration::from_secs(
            self.config.refresh_interval.as_seconds(),
        ))
        .map(|_| Message::RefreshPortfolio);

        let mut subs = vec![rotate, refresh_market, refresh_portfolio];

        let market_loading =
            self.quotes_loading && self.market_quotes.is_empty() && self.error_message.is_none();

        let portfolio_loading = self.portfolio_loading
            && self.portfolio_quotes.is_empty()
            && self.portfolio_error.is_none();

        if market_loading || portfolio_loading {
            subs.push(
                cosmic::iced::time::every(Duration::from_millis(50)).map(|_| Message::SkeletonTick),
            );
        }

        Subscription::batch(subs)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let pinned_index = self
            .config
            .pinned_portfolio_index
            .filter(|&index| index > 0 && index <= self.user_portfolios.len());

        let (quotes, current_index) = if pinned_index.is_some() {
            (&self.portfolio_quotes, self.portfolio_current_index)
        } else {
            (&self.market_quotes, self.current_index)
        };

        let content = applet::build_applet_content(
            &self.config,
            quotes,
            current_index,
            &self.logo_handles,
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
            .last_updated_key()
            .and_then(|key| self.last_fetch_time.get(&key))
            .map(|instant| {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .saturating_sub(instant.elapsed().as_secs());

                format_publish_time(timestamp)
            });

        let theme = self.core.system_theme();

        let current_portfolio = self
            .current_portfolio_index
            .checked_sub(1)
            .and_then(|i| self.user_portfolios.get(i));

        let header = if self.active_tab == PopupTab::Tools {
            tools_header(last_updated, self.tools_selected_asset.clone())
        } else {
            header(
                last_updated,
                self.active_tab,
                self.user_portfolios.len(),
                current_portfolio.map(|w| w.name.clone()),
            )
        };

        // Maximum supported applet scale is 200%.
        let max_height = self
            .core
            .applet
            .suggested_bounds
            .map_or(500.0, |bounds| bounds.height);

        let body_height = (max_height - 80.0).min(520.0).max(120.0);
        let pulse = 0.5 + 0.5 * self.skeleton_phase.sin();

        let style = QuoteStyle {
            show_logos: self.config.show_logos,
            logo_handles: &self.logo_handles,
            pulse,
        };

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

            PopupTab::Portfolio => {
                let portfolio = self.build_portfolio_view();

                let search = SearchView {
                    value: &self.stock_search_input,
                    results: &self.stock_search_results,
                    loading: self.stock_search_loading,
                    limit_reached: current_portfolio
                        .is_some_and(|p| p.symbols.len() >= MAX_ASSETS_PER_PORTFOLIO),
                };

                Some(
                    widget::container(portfoliocard(theme, &portfolio, &search, &style))
                        .height(Length::Fixed(body_height)),
                )
            }

            PopupTab::Tools => {
                let alerts: &[PriceAlert] = current_portfolio.map_or(&[], |p| p.alerts.as_slice());

                let tools = ToolsView {
                    selected_asset: self.tools_selected_asset.as_ref(),
                    portfolio_index: self.current_portfolio_index,
                    alerts,
                    selected_condition: &self.alert_selected_condition,
                    input_value: &self.alert_input_value,
                    new_alert_enabled: self.alert_enabled,
                };

                Some(widget::container(toolscard(theme, &tools)).height(Length::Fixed(body_height)))
            }

            PopupTab::Pinned => {
                let portfolio = self.build_portfolio_view();

                Some(
                    widget::container(pinnedcard(theme, &portfolio, &style))
                        .height(Length::Fixed(body_height)),
                )
            }

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
                    let pinned_index = self
                        .config
                        .pinned_portfolio_index
                        .filter(|&index| index > 0 && index <= self.user_portfolios.len());

                    if let Some(index) = pinned_index {
                        self.current_portfolio_index = index;
                        self.active_tab = PopupTab::Pinned;
                    } else {
                        self.active_tab = PopupTab::Overview;
                    }

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
                let pinned = self
                    .config
                    .pinned_portfolio_index
                    .is_some_and(|index| index > 0 && index <= self.user_portfolios.len());

                if pinned {
                    if !self.portfolio_quotes.is_empty() {
                        self.portfolio_current_index =
                            (self.portfolio_current_index + 1) % self.portfolio_quotes.len();
                    }
                } else if !self.market_quotes.is_empty() {
                    self.current_index = (self.current_index + 1) % self.market_quotes.len();
                }
            }

            Message::SkeletonTick => {
                self.skeleton_phase = (self.skeleton_phase + 0.15) % std::f32::consts::TAU;
            }

            Message::SetTab(tab) => {
                self.screens_tab = tab;

                let key = self.market_key();
                let refresh_secs = self.config.refresh_interval.as_seconds();

                let cache_valid = self
                    .last_fetch_time
                    .get(&key)
                    .is_some_and(|t| t.elapsed().as_secs() < refresh_secs);

                if cache_valid {
                    self.restore_cached_market(&key);
                    self.quotes_loading = false;
                    return Task::none();
                }

                self.market_quotes.clear();
                return self.fetch_market();
            }

            Message::MarketLoaded(key, result) => {
                let is_current = self.is_key_current(&key);
                let is_portfolio = matches!(key, MarketCacheKey::Portfolio(_));

                match result {
                    Ok(data) => {
                        // Cache is always written to the ORIGIN key
                        self.last_fetch_time.insert(key.clone(), Instant::now());
                        self.cached_quotes.insert(key.clone(), data.clone());

                        // Response from another tab/wallet: cache only, don't touch the UI
                        if !is_current {
                            return Task::none();
                        }

                        let mut tasks = Vec::new();

                        if is_portfolio {
                            self.portfolio_loading = false;
                            self.portfolio_error = None;
                            self.portfolio_quotes = data;

                            // Alertas pertencem aos portfolios
                            self.check_and_trigger_alerts();
                        } else {
                            self.quotes_loading = false;
                            self.error_message = None;
                            self.market_quotes = data;

                            if self.config.show_news {
                                self.news_loading = true;

                                let news_symbols: Vec<String> = self
                                    .market_quotes
                                    .iter()
                                    .take(NEWS_SYMBOLS_LIMIT)
                                    .map(|q| q.symbol.clone())
                                    .collect();
                                let count = self.config.count_news_by_simbol;
                                let news_key = key.clone();

                                tasks.push(Task::perform(
                                    fetch_news_for_symbols(news_symbols, count),
                                    move |result| {
                                        Action::App(Message::NewsLoaded(
                                            news_key,
                                            result.map_err(|e| e.to_string()),
                                        ))
                                    },
                                ));
                            } else {
                                self.news_loading = false;
                            }

                            tasks.push(Task::done(Action::App(Message::Tick)));
                        }

                        if self.config.show_logos {
                            let missing = self.missing_logo_symbols();

                            if !missing.is_empty() {
                                tasks.push(Task::perform(fetch_logos(missing), |logos| {
                                    Action::App(Message::LogosLoaded(logos))
                                }));
                            }
                        }

                        return Task::batch(tasks);
                    }

                    Err(err) => {
                        // Invalidate the ORIGIN key
                        self.invalidate_cache_key(&key);

                        if !is_current {
                            return Task::none();
                        }

                        if is_portfolio {
                            self.portfolio_loading = false;
                            self.portfolio_error = Some(err);
                            self.portfolio_quotes.clear();
                            return Task::none();
                        }

                        self.finish_market_loading();
                        self.error_message = Some(err);
                        self.market_quotes.clear();

                        return Task::done(Action::App(Message::Tick));
                    }
                }
            }

            Message::LogosLoaded(logos) => {
                for quote in self
                    .market_quotes
                    .iter_mut()
                    .chain(self.portfolio_quotes.iter_mut())
                {
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

            Message::NewsLoaded(key, result) => {
                let is_current = self.is_key_current(&key);

                match result {
                    Ok(news) => {
                        self.cached_news.insert(key, news.clone());
                        if is_current {
                            self.market_news = news;
                            self.news_error = None;
                            self.news_loading = false;
                        }
                    }
                    Err(err) => {
                        tracing::warn!("news fetch failed: {err}");
                        if is_current {
                            self.news_error = Some(err);
                            self.news_loading = false;
                        }
                    }
                }
            }

            Message::OpenExternalLink(url) => {
                let _ = std::process::Command::new("xdg-open").arg(url).spawn();
            }

            Message::RefreshMarket => {
                self.reset_market_view();

                let key = self.market_key();
                self.invalidate_cache_key(&key);

                return self.fetch_market();
            }

            Message::RefreshPortfolio => {
                if self.current_portfolio_index == 0 {
                    return Task::none();
                }

                let key = self.portfolio_key();
                self.invalidate_cache_key(&key);

                self.portfolio_quotes.clear();
                self.portfolio_error = None;

                return self.fetch_current_portfolio();
            }

            Message::SetRefreshInterval(interval) => {
                self.config.refresh_interval = interval;
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
                let tab = tab.unwrap_or(PopupTab::Overview);
                self.active_tab = tab;

                if tab == PopupTab::Portfolio
                    && self.current_portfolio_index == 0
                    && !self.user_portfolios.is_empty()
                {
                    return self.update(Message::SwitchPortfolio(1));
                }
            }

            Message::NavigateToTools(asset) => {
                self.active_tab = PopupTab::Tools;
                self.tools_selected_asset = Some(asset);
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
                    let missing = self.missing_logo_symbols();

                    if !missing.is_empty() {
                        return Task::perform(fetch_logos(missing), |logos| {
                            Action::App(Message::LogosLoaded(logos))
                        });
                    }
                }
            }

            Message::ToggleShowNews(new_value) => {
                self.config.show_news = new_value;
                let key = self.market_key();
                self.save_config();

                if new_value && !self.market_quotes.is_empty() {
                    let symbols: Vec<String> = self
                        .market_quotes
                        .iter()
                        .take(NEWS_SYMBOLS_LIMIT)
                        .map(|q| q.symbol.clone())
                        .collect();

                    self.news_loading = true;

                    return Task::perform(
                        fetch_news_for_symbols(symbols, self.config.count_news_by_simbol),
                        move |result| {
                            Action::App(Message::NewsLoaded(key, result.map_err(|e| e.to_string())))
                        },
                    );
                }

                if !new_value {
                    self.market_news.clear();
                    self.news_loading = false;
                }
            }

            Message::AddPortfolio => {
                if self.user_portfolios.len() >= MAX_PORTFOLIOS {
                    return Task::none();
                }

                let index = self.user_portfolios.len() + 1;
                self.user_portfolios
                    .push(PortFolio::new(format!("Portfolio {index}")));
                self.current_portfolio_index = self.user_portfolios.len();

                // Only clear port
                self.portfolio_quotes.clear();
                self.portfolio_error = None;
                self.portfolio_loading = false;

                self.config.last_portfolio_index = self.current_portfolio_index;
                self.save_config();
                save_portfolios(&self.user_portfolios);
            }

            Message::SwitchPortfolio(index) => {
                let safe_index = if index <= self.user_portfolios.len() {
                    index
                } else {
                    0
                };

                self.current_portfolio_index = safe_index;
                self.stock_search_input.clear();
                self.stock_search_results.clear();
                self.rename_port_mode = false;
                self.portfolio_error = None;

                if self.active_tab == PopupTab::Tools {
                    self.active_tab = PopupTab::Overview;
                    self.reset_alert_form();
                }

                self.config.last_portfolio_index = safe_index;
                self.save_config();

                if safe_index == 0 {
                    self.portfolio_quotes.clear();
                    self.portfolio_loading = false;
                    return Task::none();
                }

                let key = self.portfolio_key();
                let refresh_secs = self.config.refresh_interval.as_seconds();

                let cache_valid = self
                    .last_fetch_time
                    .get(&key)
                    .is_some_and(|t| t.elapsed().as_secs() < refresh_secs);

                if cache_valid {
                    self.restore_cached_market(&key);
                    self.portfolio_loading = false;
                    return Task::none();
                }

                self.portfolio_quotes.clear();
                return self.fetch_current_portfolio();
            }

            Message::PreviusPortfolio => {
                if self.active_tab == PopupTab::Tools || self.current_portfolio_index <= 1 {
                    return Task::none();
                }

                return self.update(Message::SwitchPortfolio(self.current_portfolio_index - 1));
            }

            Message::NextPortfolio => {
                if self.active_tab == PopupTab::Tools
                    || self.current_portfolio_index >= self.user_portfolios.len()
                {
                    return Task::none();
                }

                return self.update(Message::SwitchPortfolio(self.current_portfolio_index + 1));
            }

            Message::ToggleRenameMode => {
                if self.current_portfolio_index > 0 {
                    self.rename_port_mode = !self.rename_port_mode;

                    if self.rename_port_mode {
                        self.rename_port_input = self.user_portfolios
                            [self.current_portfolio_index - 1]
                            .name
                            .clone();
                    }
                }
            }

            Message::RenamePortfolio(val) => {
                self.rename_port_input = val;
            }

            Message::ConfirmRenamePortfolio => {
                if self.current_portfolio_index > 0 && !self.rename_port_input.trim().is_empty() {
                    self.user_portfolios[self.current_portfolio_index - 1].name =
                        self.rename_port_input.trim().to_string();

                    save_portfolios(&self.user_portfolios);
                }

                self.rename_port_mode = false;
            }

            Message::TogglePinPortfolio => {
                let current_index = self.current_portfolio_index;

                self.config.pinned_portfolio_index = if current_index == 0
                    || self.config.pinned_portfolio_index == Some(current_index)
                {
                    None
                } else {
                    Some(current_index)
                };

                self.save_config();

                self.active_tab = if self.config.pinned_portfolio_index.is_some() {
                    PopupTab::Pinned
                } else {
                    PopupTab::Overview
                };
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

            Message::AddStockToPortfolio(symbol_label) => {
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

                if self.current_portfolio_index > 0 {
                    let portfolio = &mut self.user_portfolios[self.current_portfolio_index - 1];

                    if portfolio.symbols.len() >= MAX_ASSETS_PER_PORTFOLIO {
                        return Task::none();
                    }

                    if portfolio.symbols.contains(&symbol) {
                        return Task::none();
                    }

                    portfolio.symbols.push(symbol);
                    save_portfolios(&self.user_portfolios);

                    let key = self.portfolio_key();
                    self.invalidate_cache_key(&key);

                    return self.fetch_current_portfolio();
                }

                return Task::none();
            }

            Message::RemoveStockFromPortfolio(symbol) => {
                if self.current_portfolio_index > 0 {
                    let portfolio = &mut self.user_portfolios[self.current_portfolio_index - 1];

                    portfolio.symbols.retain(|s| s != &symbol);
                    portfolio.alerts.retain(|a| a.symbol != symbol);

                    save_portfolios(&self.user_portfolios);

                    let selected_symbol = self
                        .tools_selected_asset
                        .as_ref()
                        .map(|asset| asset.symbol.as_str());

                    if selected_symbol == Some(symbol.as_str()) {
                        self.tools_selected_asset = None;
                        self.alert_input_value.clear();
                        self.active_tab = PopupTab::Portfolio;
                    }

                    let key = self.portfolio_key();
                    self.invalidate_cache_key(&key);
                    self.portfolio_error = None;

                    return self.fetch_current_portfolio();
                }
            }

            Message::DeleteCurrentPortfolio => {
                if self.current_portfolio_index > 0 {
                    // Indices shift after removal, so drop ALL wallet caches
                    self.cached_quotes
                        .retain(|k, _| !matches!(k, MarketCacheKey::Portfolio(_)));
                    self.last_fetch_time
                        .retain(|k, _| !matches!(k, MarketCacheKey::Portfolio(_)));
                    self.cached_news
                        .retain(|k, _| !matches!(k, MarketCacheKey::Portfolio(_)));

                    self.user_portfolios
                        .remove(self.current_portfolio_index - 1);

                    self.current_portfolio_index =
                        self.current_portfolio_index.min(self.user_portfolios.len());

                    self.portfolio_quotes.clear();
                    self.portfolio_error = None;
                    self.portfolio_loading = false;
                    self.rename_port_mode = false;
                    save_portfolios(&self.user_portfolios);

                    self.config.last_portfolio_index = self.current_portfolio_index;
                    let deleted_index = self.current_portfolio_index;

                    self.config.pinned_portfolio_index = match self.config.pinned_portfolio_index {
                        Some(index) if index == deleted_index => None,
                        Some(index) if index > deleted_index => Some(index - 1),
                        other => other,
                    };

                    self.save_config();

                    self.active_tab = PopupTab::Overview;
                    self.reset_alert_form();

                    return self.fetch_current_portfolio();
                }
            }

            Message::ToggleAlertsEnabled(val) => {
                self.config.alerts_enabled = val;
                self.save_config();
            }

            Message::AddAlert {
                portfolio_index,
                alert,
            } => {
                if portfolio_index > 0
                    && let Some(portfolio) = self.user_portfolios.get_mut(portfolio_index - 1)
                {
                    let id = self.next_alert_id;
                    self.next_alert_id += 1;

                    portfolio.alerts.push(PriceAlert::from_new(id, alert));
                    save_portfolios(&self.user_portfolios);
                }
            }

            Message::AlertSetEnabled(val) => {
                self.alert_enabled = val;
            }

            Message::RemoveAlert {
                portfolio_index,
                alert_id,
            } => {
                if portfolio_index > 0
                    && let Some(portfolio) = self.user_portfolios.get_mut(portfolio_index - 1)
                {
                    portfolio.alerts.retain(|a| a.id != alert_id);
                    save_portfolios(&self.user_portfolios);
                }
            }

            Message::ToggleAlert {
                portfolio_index,
                alert_id,
            } => {
                if portfolio_index > 0
                    && let Some(portfolio) = self.user_portfolios.get_mut(portfolio_index - 1)
                    && let Some(alert) = portfolio.alerts.iter_mut().find(|a| a.id == alert_id)
                {
                    alert.enabled = !alert.enabled;
                    save_portfolios(&self.user_portfolios);
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

    fn market_key(&self) -> MarketCacheKey {
        MarketCacheKey::Screen(self.screens_tab)
    }

    fn portfolio_key(&self) -> MarketCacheKey {
        MarketCacheKey::Portfolio(self.current_portfolio_index)
    }

    fn last_updated_key(&self) -> Option<MarketCacheKey> {
        if self.active_tab == PopupTab::Portfolio || self.active_tab == PopupTab::Tools {
            (self.current_portfolio_index > 0).then(|| self.portfolio_key())
        } else {
            Some(self.market_key())
        }
    }

    fn is_key_current(&self, key: &MarketCacheKey) -> bool {
        match key {
            MarketCacheKey::Screen(tab) => *tab == self.screens_tab,
            MarketCacheKey::Portfolio(i) => *i == self.current_portfolio_index,
        }
    }

    fn fetch_market(&mut self) -> Task<cosmic::Action<Message>> {
        self.quotes_loading = true;

        let key = self.market_key();
        let count = self.config.count_stokes_at_once;

        Task::perform(fetch_by_screeners(count, self.screens_tab), move |result| {
            Action::App(Message::MarketLoaded(
                key,
                result.map_err(|e| e.to_string()),
            ))
        })
    }

    fn fetch_current_portfolio(&mut self) -> Task<cosmic::Action<Message>> {
        let symbols = self
            .current_portfolio_index
            .checked_sub(1)
            .and_then(|i| self.user_portfolios.get(i))
            .map(|p| p.symbols.clone())
            .unwrap_or_default();

        if symbols.is_empty() {
            self.portfolio_quotes.clear();
            self.portfolio_loading = false;
            return Task::none();
        }

        self.portfolio_loading = true;

        let key = self.portfolio_key();

        Task::perform(fetch_by_symbols(symbols), move |result| {
            Action::App(Message::MarketLoaded(
                key,
                result.map_err(|e| e.to_string()),
            ))
        })
    }

    fn missing_logo_symbols(&self) -> Vec<String> {
        let mut missing: Vec<String> = self
            .market_quotes
            .iter()
            .chain(&self.portfolio_quotes)
            .filter(|q| !self.logo_handles.contains_key(&q.symbol))
            .map(|q| q.symbol.clone())
            .collect();

        missing.sort();
        missing.dedup();
        missing
    }

    fn check_and_trigger_alerts(&mut self) {
        let quotes: HashMap<&str, &MarketQuote> = self
            .portfolio_quotes
            .iter()
            .map(|q| (q.symbol.as_str(), q))
            .collect();
        let notify = self.config.alerts_enabled;
        let mut changed = false;

        for portfolio in &mut self.user_portfolios {
            portfolio.alerts.retain(|alert| {
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
            save_portfolios(&self.user_portfolios);
        }
    }

    fn invalidate_cache_key(&mut self, key: &MarketCacheKey) {
        self.last_fetch_time.remove(key);
        self.cached_quotes.remove(key);
        self.cached_news.remove(key);
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
        let Some(cached) = self.cached_quotes.get(key) else {
            return;
        };

        match key {
            MarketCacheKey::Portfolio(_) => {
                self.portfolio_quotes = cached.clone();
            }
            MarketCacheKey::Screen(_) => {
                self.market_quotes = cached.clone();

                if let Some(news) = self.cached_news.get(key) {
                    self.market_news = news.clone();
                }
            }
        }
    }
    fn reset_alert_form(&mut self) {
        self.tools_selected_asset = None;
        self.alert_input_value.clear();
        self.alert_selected_condition = AlertCondition::PriceAbove(0.0);
        self.alert_enabled = true;
    }

    fn build_portfolio_view(&self) -> PortfolioView<'_> {
        let current_portfolio = self
            .current_portfolio_index
            .checked_sub(1)
            .and_then(|i| self.user_portfolios.get(i));

        let has_portfolio = self.current_portfolio_index > 0;

        PortfolioView {
            name: current_portfolio.map(|w| w.name.as_str()),
            symbols: current_portfolio.map_or(&[], |w| w.symbols.as_slice()),
            quotes: if has_portfolio {
                &self.portfolio_quotes
            } else {
                &[]
            },
            index: self.current_portfolio_index,
            count: self.user_portfolios.len(),
            rename_input: self
                .rename_port_mode
                .then_some(self.rename_port_input.as_str()),
            error: self.portfolio_error.as_ref(),
            loading: self.portfolio_loading,
            pinned: self.config.pinned_portfolio_index.unwrap_or(0),
        }
    }
}
