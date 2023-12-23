use crate::dexscreener::Pair;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PairConfig {
    pub chain_id: String,
    pub base_token_symbol: String,
    pub quote_token_symbol: String,
    pub address: String,
    pub pair_address: String,
}

impl PairConfig {
    pub fn from(pair: Pair) -> PairConfig {
        PairConfig {
            chain_id: pair.chain_id,
            base_token_symbol: pair.base_token.symbol,
            quote_token_symbol: pair.quote_token.symbol,
            address: pair.base_token.address,
            pair_address: pair.pair_address,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pairs: Vec<PairConfig>,
}

impl Config {
    pub fn clear(&mut self) {
        self.pairs.clear();
        self.save()
    }
    pub fn exist(&mut self, pair: PairConfig) -> bool {
        self.pairs
            .iter()
            .find(|value| {
                value.quote_token_symbol == pair.quote_token_symbol
                    && value.pair_address == pair.pair_address
                    && value.address == pair.address
                    && value.base_token_symbol == pair.base_token_symbol
            })
            .is_some()
    }
    pub fn search_token(&mut self, search: &str) -> Option<PairConfig> {
        self.pairs
            .iter()
            .find(|value| {
                value.address == search
                    || value.pair_address == search
                    || value.base_token_symbol == search
                    || value.quote_token_symbol == search
            })
            .cloned()
    }
    pub fn append_token(&mut self, token: PairConfig) {
        if !self.exist(token.clone()) {
            self.pairs.push(token);
            self.save();
        }
    }

    fn save(&self) {
        let config_path = Config::get_config_path();
        let toml = toml::to_string(&self).expect("Cannot serialize config");
        let mut file = File::create(&config_path).expect("Cannot create config file");
        file.write_all(toml.as_bytes())
            .expect("Cannot write config file");
    }

    fn get_config_path() -> PathBuf {
        let mut config_path = dirs::home_dir().expect("Cannot find home dir");
        config_path.push(".config/token-tide/config.toml");
        config_path
    }

    fn create_default_config() -> Config {
        let config_path = Config::get_config_path();
        const DEFAULT_CONFIG: Config = Config { pairs: vec![] };
        let toml = toml::to_string(&DEFAULT_CONFIG).expect("Cannot serialize default config");
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).expect("Cannot create config dir");
        }
        let mut file = File::create(&config_path).expect("Cannot create config file");
        file.write_all(toml.as_bytes())
            .expect("Cannot write config files");
        DEFAULT_CONFIG
    }

    pub fn load() -> Config {
        let config_path = Config::get_config_path();

        if !config_path.exists() {
            return Config::create_default_config();
        }

        let config_content =
            fs::read_to_string(config_path.clone()).expect("Cannot read config file");
        let config = toml::from_str(&config_content);
        match config {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Error parsing config file: {}", err);
                if let Err(delete_err) = fs::remove_file(&config_path.clone()) {
                    eprintln!("Error deleting config file: {}", delete_err);
                }
                return Config::create_default_config();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_clear_token() {
        let mut config = Config::load();
        config.clear();
        let token = PairConfig {
            chain_id: "solana".to_string(),
            base_token_symbol: "TEST".to_string(),
            quote_token_symbol: "USDT".to_string(),
            address: "test_address".to_string(),
            pair_address: "test_pair_address".to_string(),
        };

        config.append_token(token);
        assert_eq!(config.pairs.len(), 1);
        assert_eq!(config.pairs[0].base_token_symbol, "TEST");
        assert_eq!(config.pairs[0].quote_token_symbol, "USDT");
        assert_eq!(config.pairs[0].address, "test_address");
        assert_eq!(config.pairs[0].pair_address, "test_pair_address");

        config.clear();
        assert_eq!(config.pairs.len(), 0);
    }

    #[test]
    fn test_append_tokens() {
        let mut config = Config::load();
        config.clear();
        let token = PairConfig {
            chain_id: "solana".to_string(),
            base_token_symbol: "TEST".to_string(),
            quote_token_symbol: "USDT".to_string(),
            address: "test_address".to_string(),
            pair_address: "test_pair_address".to_string(),
        };

        let token2 = PairConfig {
            chain_id: "solana".to_string(),
            base_token_symbol: "TEST2".to_string(),
            quote_token_symbol: "USDT".to_string(),
            address: "test_address2".to_string(),
            pair_address: "test_pair_address2".to_string(),
        };

        config.append_token(token);
        config.append_token(token2);
        assert_eq!(config.pairs.len(), 2);
        assert_eq!(config.pairs[0].base_token_symbol, "TEST");
        assert_eq!(config.pairs[1].base_token_symbol, "TEST2");

        config.clear();
        assert_eq!(config.pairs.len(), 0);
    }

    #[test]
    fn test_repeat_tokens() {
        let mut config = Config::load();
        config.clear();
        let token = PairConfig {
            chain_id: "solana".to_string(),
            base_token_symbol: "TEST".to_string(),
            quote_token_symbol: "USDT".to_string(),
            address: "test_address".to_string(),
            pair_address: "test_pair_address".to_string(),
        };

        config.append_token(token.clone());
        config.append_token(token.clone());
        config.append_token(token.clone());
        assert_eq!(config.pairs.len(), 1);
        assert_eq!(config.pairs[0].base_token_symbol, "TEST");
        assert_eq!(config.pairs[0].quote_token_symbol, "USDT");
        assert_eq!(config.pairs[0].address, "test_address");
        assert_eq!(config.pairs[0].pair_address, "test_pair_address");

        config.clear();
        assert_eq!(config.pairs.len(), 0);
    }

    #[test]
    fn test_search_tokens() {
        let mut config = Config::load();
        config.clear();
        let token = PairConfig {
            chain_id: "solana".to_string(),
            base_token_symbol: "HONEY".to_string(),
            quote_token_symbol: "USDT".to_string(),
            address: "4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy".to_string(),
            pair_address: "2RVVkjA9cRHzZgpLiS1s5eRudqF8ZD3kguCGoU1vhjPo".to_string(),
        };
        assert_eq!(config.search_token("HO").is_some(), false);
        assert_eq!(config.search_token("HONEY").is_some(), false);
        assert_eq!(config.search_token("USDT").is_some(), false);
        assert_eq!(
            config
                .search_token("4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy")
                .is_some(),
            false
        );
        assert_eq!(
            config
                .search_token("2RVVkjA9cRHzZgpLiS1s5eRudqF8ZD3kguCGoU1vhjPo")
                .is_some(),
            false
        );
        config.append_token(token);
        assert_eq!(config.search_token("HO").is_some(), false);
        assert_eq!(config.search_token("HONEY").is_some(), true);
        assert_eq!(config.search_token("USDT").is_some(), true);
        assert_eq!(
            config
                .search_token("4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy")
                .is_some(),
            true
        );
        assert_eq!(
            config
                .search_token("2RVVkjA9cRHzZgpLiS1s5eRudqF8ZD3kguCGoU1vhjPo")
                .is_some(),
            true
        );
    }
}
