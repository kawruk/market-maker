pub struct Config;

impl Config {
    pub fn binance_api_url() -> &'static str {
        "https://api.binance.com"
    }

    pub fn oanda_api_url() -> &'static str {
        "https://api-fxpractice.oanda.com"
    }

    pub fn alpha_vantage_url() -> &'static str {
        "https://www.alphavantage.co"
    }
}
