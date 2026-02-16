// SPDX-License-Identifier: GPL-3.0-only
//
// Inspired by cosmic-ext-applet-tempest
// https://codeberg.org/VintageTechie/cosmic-ext-applet-tempest

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
// DATA STRUCTURES
//

#[derive(Debug, Clone)]
pub struct MarketQuote {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
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
    symbol: String,

    #[serde(rename = "shortName")]
    short_name: Option<String>,

    #[serde(rename = "regularMarketPrice")]
    regular_market_price: Option<f64>,

    #[serde(rename = "regularMarketChange")]
    regular_market_change: Option<f64>,

    #[serde(rename = "regularMarketChangePercent")]
    regular_market_change_percent: Option<f64>,
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
        })
        .collect();

    Ok(quotes)
}
