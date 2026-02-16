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

use cosmic::iced::Color;

pub struct VariationStyle {
    pub color: Color,
    pub icon: &'static str,
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
            Color::from_rgb(0.13, 0.77, 0.37) // verde
        } else {
            Color::from_rgb(0.94, 0.27, 0.27) // vermelho
        }
    }
} //
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
