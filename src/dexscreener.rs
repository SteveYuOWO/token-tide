use crate::{constants, number};
use prettytable::{row, Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub chain_id: String,
    pub dex_id: String,
    pub url: String,
    pub pair_address: String,
    pub base_token: Token,
    pub quote_token: QuoteToken,
    pub price_native: String,
    pub price_usd: Option<String>,
    pub txns: Transactions,
    pub volume: Volume,
    pub price_change: PriceChange,
    pub liquidity: Option<Liquidity>,
    pub fdv: Option<f64>,
    pub pair_created_at: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Token {
    pub address: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuoteToken {
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transactions {
    pub m5: BuySell,
    pub h1: BuySell,
    pub h6: BuySell,
    pub h24: BuySell,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BuySell {
    pub buys: i32,
    pub sells: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Volume {
    pub m5: f64,
    pub h1: f64,
    pub h6: f64,
    pub h24: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PriceChange {
    pub m5: f64,
    pub h1: f64,
    pub h6: f64,
    pub h24: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Liquidity {
    pub usd: Option<f64>,
    pub base: f64,
    pub quote: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pairs {
    pub pairs: Vec<Pair>,
}

const DEXSCREENER_HOST: &str = "https://api.dexscreener.io";
async fn search_token(token: &str) -> Result<Vec<Pair>, Box<dyn Error>> {
    let url = format!("{}/latest/dex/search/?q={}", DEXSCREENER_HOST, token);
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let text = response.text().await?;
        let pairs_result: Pairs = serde_json::from_str(&text)?;
        Ok(pairs_result.pairs)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to fetch data: {}", response.status()),
        )))
    }
}

async fn query_pair(chain_id: &str, pair_address: &str) -> Result<Vec<Pair>, Box<dyn Error>> {
    let url = format!(
        "{}/latest/dex/pairs/{}/{}",
        DEXSCREENER_HOST, chain_id, pair_address
    );
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let text = response.text().await?;
        let pairs_result: Pairs = serde_json::from_str(&text)?;
        Ok(pairs_result.pairs)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to fetch data: {}", response.status()),
        )))
    }
}
pub async fn query(pair_address: &str, simple: &bool) {
    let pair_result = search_token(pair_address).await;
    let search_pair: Option<Pair> = match pair_result {
        Ok(pairs) => pairs.get(0).cloned(),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    };
    if let Some(pair) = search_pair {
        let query_result = query_pair(pair.chain_id.as_str(), pair.pair_address.as_str()).await;
        match query_result {
            Ok(pairs) => {
                let mut table = Table::new();
                table.add_row(row!["Property", "Value"]);
                if let Some(first_pair) = pairs.get(0) {
                    table.add_row(row![
                        "Pair",
                        format!(
                            "{}{}",
                            first_pair.base_token.symbol, first_pair.quote_token.symbol
                        )
                    ]);
                    table.add_row(row![
                        "Price In USD",
                        &first_pair
                            .price_usd
                            .clone()
                            .unwrap_or(constants::NONE_STR.to_string())
                    ]);
                    table.add_row(row!["Token Address", &first_pair.base_token.address]);

                    if !simple {
                        table.add_row(row!["Chain", &first_pair.chain_id]);
                        table.add_row(row!["DEX", &first_pair.dex_id]);
                        table.add_row(row![
                            "24h Volume",
                            &format!(
                                "{} ({})",
                                number::format_with_unit(first_pair.volume.h24),
                                number::to_locale_string(first_pair.volume.h24)
                            )
                        ]);
                        table.add_row(row![
                            "FDV",
                            &first_pair
                                .fdv
                                .map(|value| format!(
                                    "{} ({})",
                                    number::format_with_unit(value),
                                    number::to_locale_string(value)
                                ))
                                .unwrap_or(constants::NONE_STR.to_string())
                        ]);
                        table.add_row(row![
                            "Liquidity",
                            &first_pair
                                .liquidity
                                .clone()
                                .map(|value| format!(
                                    "${} (${})",
                                    number::format_with_unit(value.usd.unwrap_or(0.0)),
                                    number::to_locale_string(value.usd.unwrap_or(0.0))
                                ))
                                .unwrap_or(constants::NONE_STR.to_string())
                        ]);
                        table.add_row(row!["Pair Address", &first_pair.pair_address]);
                        table.add_row(row!["Link", &first_pair.url]);
                    }
                } else {
                    println!("No pairs found.");
                }
                table.printstd();
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

pub async fn search(token: &str) {
    match search_token(token).await {
        Ok(pairs) => {
            let mut table = Table::new();

            table.add_row(row![
                "Pair",
                "Chain",
                "DEX",
                "Price In USD",
                "Token Address",
                "Pair Address",
            ]);
            for pair in pairs {
                table.add_row(Row::new(vec![
                    Cell::new(&(pair.base_token.symbol + pair.quote_token.symbol.as_str())),
                    Cell::new(&pair.chain_id),
                    Cell::new(&pair.dex_id),
                    Cell::new(&pair.price_usd.unwrap_or(constants::NONE_STR.to_string())),
                    Cell::new(&pair.base_token.address),
                    Cell::new(&pair.pair_address),
                ]));
            }

            table.printstd();
        }
        Err(e) => println!("Error: {}", e),
    }
}
