// SPDX-License-Identifier: GPL-3.0-only
//
// Inspired by cosmic-ext-applet-tempest
// https://codeberg.org/VintageTechie/cosmic-ext-applet-tempest

use cosmic::iced::Color;
use serde::Deserialize;
use std::sync::OnceLock;

const USER_AGENT: &str =
    "(cosmic-ext-marketwatch, https://github.com/MiracleForge/cosmic-marketwatch)";

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
// ================= DATA STRUCTURES =================
//

#[derive(Debug, Clone)]
pub struct MarketQuote {
    pub change: f64,
    pub change_percent: f64,
    pub currency: String,
    #[allow(dead_code)]
    pub name: String,
    pub price: f64,
    pub symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YahooNews {
    pub title: String,
    pub link: String,
    pub publisher: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "providerPublishTime")]
    pub publish_time: Option<u64>,
}

//
// ================= IMPLEMENTATION =================
//

impl MarketQuote {
    pub fn formatted_price(&self) -> String {
        self.format_currency(self.price)
    }

    fn format_currency(&self, value: f64) -> String {
        let abs_value = value.abs();
        let is_negative = value.is_sign_negative();

        let (symbol, decimals, decimal_sep, thousand_sep, symbol_before) =
            match self.currency.as_str() {
                "USD" => ("$", 2, ".", ",", true),
                "BRL" => ("R$", 2, ",", ".", true),
                "EUR" => ("€", 2, ",", ".", false),
                "GBP" => ("£", 2, ".", ",", true),
                "JPY" => ("¥", 0, ".", ",", true),
                "CHF" => ("CHF", 2, ".", "'", false),
                "CAD" => ("C$", 2, ".", ",", true),
                "AUD" => ("A$", 2, ".", ",", true),
                "CNY" => ("¥", 2, ".", ",", true),
                "INR" => ("₹", 2, ".", ",", true),
                _ => (self.currency.as_str(), 2, ".", ",", true),
            };

        let formatted = format_number(abs_value, decimals, decimal_sep, thousand_sep);

        let result = if symbol_before {
            format!("{symbol}{formatted}")
        } else {
            format!("{formatted} {symbol}")
        };

        if is_negative {
            format!("-{result}")
        } else {
            result
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
// ================= DESERIALIZATION =================
//

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

#[derive(Debug, Deserialize)]
struct SearchResponse {
    news: Option<Vec<YahooNews>>,
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

    #[serde(rename = "regularMarketPrice")]
    regular_market_price: Option<f64>,

    #[serde(rename = "chartPreviousClose")]
    chart_previous_close: Option<f64>,
}

//
// ================= FETCH FUNCTIONS =================
//

pub async fn fetch_most_active(count: u64) -> Result<Vec<MarketQuote>, reqwest::Error> {
    let url = format!(
        "https://query1.finance.yahoo.com/v1/finance/screener/predefined/saved?count={count}&scrIds=most_actives"
    );

    let response = http_client().get(&url).send().await?;
    let data: ScreenerResponse = response.json().await?;

    let quotes = data
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

    for symbol in symbols {
        let url = format!(
            "https://query1.finance.yahoo.com/v1/finance/search?q={symbol}&newsCount={news_per_symbol}&quotesCount=0"
        );

        let response = http_client().get(&url).send().await?;
        let data: SearchResponse = response.json().await?;

        if let Some(news) = data.news {
            all_news.extend(news);
        }
    }

    Ok(all_news)
}

pub async fn search_symbols(query: String) -> Result<Vec<String>, reqwest::Error> {
    let url = format!(
        "https://query1.finance.yahoo.com/v1/finance/search?q={query}&quotesCount=6&newsCount=0"
    );

    let response = http_client().get(&url).send().await?;
    let data: QuoteSearchResponse = response.json().await?;

    let symbols = data
        .quotes
        .unwrap_or_default()
        .into_iter()
        .map(|q| {
            q.short_name
                .map(|n| format!("{} — {}", q.symbol, n))
                .unwrap_or(q.symbol)
        })
        .collect();

    Ok(symbols)
}

pub async fn fetch_by_symbols(symbols: Vec<String>) -> Result<Vec<MarketQuote>, reqwest::Error> {
    let mut quotes = Vec::new();

    for symbol in symbols {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{symbol}?range=1d&interval=1d"
        );

        let response = http_client().get(&url).send().await?;
        let data: ChartResponse = response.json().await?;

        if let Some(results) = data.chart.result
            && let Some(result) = results.first()
        {
            let meta = &result.meta;

            let price = meta.regular_market_price.unwrap_or(0.0);
            let previous = meta.chart_previous_close.unwrap_or(price);
            let change = price - previous;

            let change_percent = if previous == 0.0 {
                0.0
            } else {
                (change / previous) * 100.0
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

    Ok(quotes)
}

//
// Utils functions
//
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

fn format_number(value: f64, decimals: usize, decimal_sep: &str, thousand_sep: &str) -> String {
    let formatted = format!("{value:.decimals$}");

    let mut parts = formatted.split('.');
    let integer_part = parts.next().unwrap_or("");
    let decimal_part = parts.next();

    let integer_with_sep = add_thousand_separator(integer_part, thousand_sep);

    match decimal_part {
        Some(dec) if decimals > 0 => format!("{integer_with_sep}{decimal_sep}{dec}"),
        _ => integer_with_sep,
    }
}

fn add_thousand_separator(number: &str, sep: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = number.chars().rev().collect();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push_str(sep);
        }
        result.push(*ch);
    }

    result.chars().rev().collect()
}

pub fn format_publish_time(timestamp: u64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if timestamp > now {
        return "just now".to_string();
    }

    let diff = now - timestamp;

    if diff < 60 {
        "just now".to_string()
    } else if diff < 3600 {
        let mins = diff / 60;
        if mins == 1 {
            "1 min ago".to_string()
        } else {
            format!("{mins} min ago")
        }
    } else if diff < 86400 {
        let hours = diff / 3600;
        if hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{hours} hours ago")
        }
    } else if diff < 172_800 {
        "yesterday".to_string()
    } else {
        let days = diff / 86400;
        if days < 7 {
            format!("{days} days ago")
        } else if days < 14 {
            "1 week ago".to_string()
        } else {
            format!("{} weeks ago", days / 7)
        }
    }
}
