// SPDX-License-Identifier: GPL-3.0-only
//
// Inspired by cosmic-ext-applet-tempest
// https://codeberg.org/VintageTechie/cosmic-ext-applet-tempest

use cosmic::iced::Color;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::OnceLock;

const USER_AGENT: &str =
    "(cosmic-ext-marketwatch, https://github.com/MiracleForge/cosmic-marketwatch)";

const GET_STOKS_BY_SYMBOL: &str =
    "https://query1.finance.yahoo.com/v8/finance/chart/AAPL?range=5d&interval=1m";
//
// HTTP CLIENT
//

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to build HTTP client")
    })
}

//
// DATA STRUCTURES
//

#[derive(Debug, Clone)]
pub struct MarketQuote {
    pub change: f64,
    pub change_percent: f64,
    pub currency: String,
    pub name: String,
    pub price: f64,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
struct ScreenerResponse {
    finance: Finance,
}

#[derive(Debug, Deserialize)]
struct Finance {
    result: Option<Vec<ScreenerResult>>,
}

#[derive(Debug, Deserialize)]
struct ScreenerResult {
    quotes: Vec<YahooQuote>,
}

#[derive(Debug, Deserialize)]
struct YahooQuote {
    currency: Option<String>,

    #[serde(rename = "regularMarketChange")]
    regular_market_change: Option<f64>,

    #[serde(rename = "regularMarketChangePercent")]
    regular_market_change_percent: Option<f64>,

    #[serde(rename = "regularMarketPrice")]
    regular_market_price: Option<f64>,

    #[serde(rename = "shortName")]
    short_name: Option<String>,

    symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YahooNews {
    pub title: String,
    pub link: String,
    pub publisher: Option<String>,
    #[serde(rename = "providerPublishTime")]
    pub publish_time: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    news: Option<Vec<YahooNews>>,
}

pub struct VariationStyle {
    pub color: Color,
    pub icon: &'static str,
}

#[derive(Debug, Deserialize)]
struct QuoteSearchResponse {
    quotes: Option<Vec<QuoteSearchItem>>,
}

#[derive(Debug, Deserialize)]
struct QuoteSearchItem {
    symbol: String,
    #[serde(rename = "shortname")]
    short_name: Option<String>,
}

impl MarketQuote {
    pub fn formatted_price(&self) -> String {
        match self.currency.as_str() {
            "USD" => format!("${:.2}", self.price),
            "BRL" => {
                let value = format!("{:.2}", self.price).replace(".", ",");
                format!("R$ {}", value)
            }
            "EUR" => {
                let value = format!("{:.2}", self.price).replace(".", ",");
                format!("{} €", value)
            }
            "JPY" => format!("¥{:.0}", self.price),
            code => format!("{} {:.2}", code, self.price),
        }
    }

    pub fn formatted_variation(&self) -> String {
        format!(
            "{} {:.2}%",
            self.variation_icon(),
            self.change_percent.abs()
        )
    }

    pub fn variation_icon(&self) -> &'static str {
        if self.change >= 0.0 { "▲" } else { "▼" }
    }

    pub fn is_positive(&self) -> bool {
        self.change >= 0.0
    }

    pub fn variation_color(&self) -> Color {
        if self.is_positive() {
            Color::from_rgb(0.13, 0.77, 0.37)
        } else {
            Color::from_rgb(0.94, 0.27, 0.27)
        }
    }
}
//
// FETCH FUNCTIONS
//

pub async fn fetch_most_active(count: u64) -> Result<Vec<MarketQuote>, reqwest::Error> {
    println!("Starting fet most active");
    tracing::info!("Starting feth Most active {}", count);
    let url = format!(
        "https://query1.finance.yahoo.com/v1/finance/screener/predefined/saved?count={}&scrIds=most_actives",
        count
    );

    let response = http_client().get(url).send().await?;
    let data: ScreenerResponse = response.json().await?;
    println!("parset data: {:#?}", data);

    let quotes: Vec<MarketQuote> = data
        .finance
        .result
        .unwrap_or_default()
        .into_iter()
        .flat_map(|r| r.quotes)
        .map(|q| MarketQuote {
            symbol: q.symbol,
            name: q.short_name.unwrap_or_else(|| "Unknown".into()),
            price: q.regular_market_price.unwrap_or(0.0),
            change: q.regular_market_change.unwrap_or(0.0),
            change_percent: q.regular_market_change_percent.unwrap_or(0.0),
            currency: q.currency.unwrap_or_else(|| "USD".to_string()),
        })
        .collect();

    Ok(quotes)
}

pub async fn fetch_news_for_symbols(
    symbols: Vec<String>,
    news_per_symbol: u64,
) -> Result<Vec<YahooNews>, reqwest::Error> {
    let mut all_news = Vec::new();

    for symbol in symbols.iter() {
        let url = format!(
            "https://query1.finance.yahoo.com/v1/finance/search?q={}&newsCount={}&quotesCount=0",
            symbol, news_per_symbol
        );

        let response = http_client().get(&url).send().await?;
        let data: SearchResponse = response.json().await?;

        if let Some(news) = data.news {
            all_news.extend(news);
        }
    }

    Ok(all_news)
}

pub fn user_friendly_error_message(err: &str) -> &'static str {
    if err.contains("dns error") || err.contains("failed to lookup") {
        "You're offline. Please check your internet connection."
    } else if err.contains("timeout") {
        "The request took too long. Try again in a moment."
    } else if err.contains("connection refused") {
        "Unable to reach the server right now."
    } else {
        "Something went wrong while fetching market data."
    }
}

pub async fn search_symbols(query: String) -> Result<Vec<String>, reqwest::Error> {
    let url = format!(
        "https://query1.finance.yahoo.com/v1/finance/search?q={}&quotesCount=6&newsCount=0",
        query
    );

    let response = http_client().get(&url).send().await?;
    let data: QuoteSearchResponse = response.json().await?;

    let symbols = data
        .quotes
        .unwrap_or_default()
        .into_iter()
        .map(|q| {
            let label = q
                .short_name
                .map(|n| format!("{} — {}", q.symbol, n))
                .unwrap_or(q.symbol.clone());
            label
        })
        .collect();

    Ok(symbols)
}

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    #[serde(rename = "quoteResponse")]
    quote_response: QuoteResponseInner,
}

#[derive(Debug, Deserialize)]
struct QuoteResponseInner {
    result: Option<Vec<YahooQuote>>,
}

#[derive(Debug, Deserialize)]
struct ChartResponse {
    chart: Chart,
}

#[derive(Debug, Deserialize)]
struct Chart {
    result: Option<Vec<ChartResult>>,
}

#[derive(Debug, Deserialize)]
struct ChartResult {
    meta: ChartMeta,
}

#[derive(Debug, Deserialize)]
struct ChartMeta {
    symbol: String,
    currency: Option<String>,
    regularMarketPrice: Option<f64>,
    chartPreviousClose: Option<f64>,
}
pub async fn fetch_by_symbols(symbols: Vec<String>) -> Result<Vec<MarketQuote>, reqwest::Error> {
    let mut quotes = Vec::new();

    for symbol in symbols {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?range=1d&interval=1d",
            symbol
        );

        println!("URL: {}", url);

        let response = http_client().get(&url).send().await?;
        let data: ChartResponse = response.json().await?;

        if let Some(results) = data.chart.result {
            if let Some(result) = results.first() {
                let meta = &result.meta;

                let price = meta.regularMarketPrice.unwrap_or(0.0);
                let previous = meta.chartPreviousClose.unwrap_or(price);
                let change = price - previous;
                let change_percent = if previous != 0.0 {
                    (change / previous) * 100.0
                } else {
                    0.0
                };

                quotes.push(MarketQuote {
                    symbol: meta.symbol.clone(),
                    name: meta.symbol.clone(),
                    price,
                    change,
                    change_percent,
                    currency: meta.currency.clone().unwrap_or("USD".to_string()),
                });
            }
        }
    }

    Ok(quotes)
}
