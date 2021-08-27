use serde::de::DeserializeOwned;

use crate::{
    client::HttpClient,
    crypto::CryptoBuilder,
    custom::CustomBuilder,
    earning::EarningBuilder,
    error::{Error, Result},
    exchange::ExchangeBuilder,
    forex::ForexBuilder,
    quote::QuoteBuilder,
    search::SearchBuilder,
    sector::SectorBuilder,
    stock_time::TimeSeriesBuilder,
    technical_indicator::IndicatorBuilder,
    utils::{
        CryptoFunction, ForexFunction, OutputSize, StockFunction, TechnicalIndicatorInterval,
        TimeSeriesInterval,
    },
};

const BASE_URL: &str = "https://www.alphavantage.co/";

/// Struct for initializing client which contains different method for API call
pub struct ApiClient<'a> {
    api: &'a str,
    client: Box<dyn HttpClient>,
}

impl<'a> ApiClient<'a> {
    /// Method for initializing [ApiClient][ApiClient] struct using  user
    /// provided client
    ///
    /// ```
    /// use alpha_vantage::api::ApiClient;
    /// let api = ApiClient::set_api("some_key", reqwest::Client::new());
    /// ```
    #[must_use]
    pub fn set_api<T>(api: &'a str, client: T) -> Self
    where
        T: HttpClient + 'static,
    {
        Self {
            api,
            client: Box::new(client),
        }
    }

    /// Method to get api key
    ///
    /// ```
    /// use alpha_vantage::api::ApiClient;
    /// let api = alpha_vantage::api::ApiClient::set_api("some_key", reqwest::Client::new());
    /// assert_eq!(api.get_api_key(), "some_key");
    /// ```
    #[must_use]
    pub fn get_api_key(&self) -> &str {
        self.api
    }

    // Get json from api endpoint and create struct
    pub(crate) async fn get_json<T>(&self, path: String) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let full_path = format!("{}{}", BASE_URL, path);
        let string_output = self.client.get_output(full_path).await?;
        serde_json::from_str(&string_output).map_err(|_| Error::DecodeJsonToStruct)
    }

    /// Crypto method for calling cryptography function with help of
    /// `CryptoBuilder`
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let crypto = api
    ///         .crypto(alpha_vantage::utils::CryptoFunction::Daily, "BTC", "CNY")
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(crypto.digital_code(), "BTC");
    ///     assert_eq!(crypto.digital_name(), "Bitcoin");
    ///     assert_eq!(crypto.market_code(), "CNY");
    ///     assert_eq!(crypto.market_name(), "Chinese Yuan");
    /// }
    /// ```
    #[must_use]
    pub fn crypto(
        &'a self,
        function: CryptoFunction,
        symbol: &'a str,
        market: &'a str,
    ) -> CryptoBuilder<'a> {
        CryptoBuilder::new(self, function, symbol, market)
    }

    /// Method for calling custom function not implemented currently in library
    /// using `CustomBuilder`
    #[must_use]
    pub fn custom(&'a self, function: &'a str) -> CustomBuilder<'a> {
        CustomBuilder::new(self, function)
    }

    /// Method for returning `EarningBuilder` for earning API
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let earning = api.earning("IBM").json().await.unwrap();
    ///     let symbol = earning.symbol();
    ///     assert_eq!(symbol, "IBM");
    /// }
    /// ```
    #[must_use]
    pub fn earning(&'a self, symbol: &'a str) -> EarningBuilder<'a> {
        EarningBuilder::new(self, symbol)
    }

    /// Method for creating `ExchangeBuilder` for exchanging currency value from
    /// one currency to another currency.
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let exchange = api.exchange("BTC", "CNY").json().await.unwrap();
    ///     assert_eq!(exchange.name_from(), "Bitcoin");
    ///     assert_eq!(exchange.code_from(), "BTC");
    ///     assert_eq!(exchange.name_to(), "Chinese Yuan");
    ///     assert_eq!(exchange.code_to(), "CNY");
    /// }
    /// ```
    #[must_use]
    pub fn exchange(&'a self, from_currency: &'a str, to_currency: &'a str) -> ExchangeBuilder<'a> {
        ExchangeBuilder::new(self, from_currency, to_currency)
    }

    /// Method for creating `ForexBuilder` for `Forex` API
    ///
    /// # Example
    /// ```
    /// use alpha_vantage::utils::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let forex = api
    ///         .forex(
    ///             ForexFunction::Weekly,
    ///             "EUR",
    ///             "USD",
    ///             TimeSeriesInterval::None,
    ///             OutputSize::None,
    ///         )
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(forex.symbol_from(), "EUR");
    ///     assert_eq!(forex.symbol_to(), "USD");
    ///     assert!(forex.interval().is_none());
    /// }
    /// ```
    #[must_use]
    pub fn forex(
        &'a self,
        function: ForexFunction,
        from_symbol: &'a str,
        to_symbol: &'a str,
        interval: TimeSeriesInterval,
        output_size: OutputSize,
    ) -> ForexBuilder<'a> {
        ForexBuilder::new(
            self,
            function,
            from_symbol,
            to_symbol,
            interval,
            output_size,
        )
    }

    /// Method for creating `QuoteBuilder` from `APIClient`
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let quote = api.quote("MSFT").json().await.unwrap();
    ///     let symbol = quote.symbol();
    ///     assert_eq!(symbol, "MSFT");
    /// }
    /// ```
    #[must_use]
    pub fn quote(&'a self, symbol: &'a str) -> QuoteBuilder<'a> {
        QuoteBuilder::new(self, symbol)
    }

    /// Method for creating search builder
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let search = api.search("BA").json().await.unwrap();
    ///     let first_search_result = &search.result()[0];
    ///     assert_eq!(first_search_result.symbol(), "BA");
    ///     assert_eq!(first_search_result.name(), "Boeing Company");
    ///     assert_eq!(first_search_result.stock_type(), "Equity");
    ///     assert_eq!(first_search_result.region(), "United States");
    ///     assert_eq!(first_search_result.currency(), "USD");
    ///     assert_eq!(first_search_result.match_score(), 1.0);
    /// }
    /// ```
    #[must_use]
    pub fn search(&'a self, keywords: &'a str) -> SearchBuilder<'a> {
        SearchBuilder::new(self, keywords)
    }

    /// Method for creating `SectorBuilder`
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let sector = api.sector().json().await.unwrap();
    ///     assert_eq!(
    ///         sector.information(),
    ///         "US Sector Performance (realtime & historical)"
    ///     );
    /// }
    /// ```
    #[must_use]
    pub fn sector(&'a self) -> SectorBuilder<'a> {
        SectorBuilder::new(self)
    }

    /// Method for creating Stock time Builder from `APIClient`
    ///
    /// # Example
    /// ```
    /// use alpha_vantage::utils::*;
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let stock = api
    ///         .stock_time(
    ///             StockFunction::Weekly,
    ///             "MSFT",
    ///             TimeSeriesInterval::None,
    ///             OutputSize::None,
    ///         )
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(stock.symbol(), "MSFT");
    ///     assert!(stock.interval().is_none());
    /// }
    /// ```
    #[must_use]
    pub fn stock_time(
        &'a self,
        function: StockFunction,
        symbol: &'a str,
        interval: TimeSeriesInterval,
        output_size: OutputSize,
    ) -> TimeSeriesBuilder<'a> {
        TimeSeriesBuilder::new(self, function, symbol, interval, output_size)
    }

    /// Method for technical indicator builder
    ///
    /// # Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let technical = api
    ///         .technical_indicator(
    ///             "SMA",
    ///             "IBM",
    ///             alpha_vantage::utils::TechnicalIndicatorInterval::Weekly,
    ///         )
    ///         .time_period(10)
    ///         .series_type("open")
    ///         .json()
    ///         .await;
    ///     assert!(technical.is_ok());
    /// }
    /// ```
    #[must_use]
    pub fn technical_indicator(
        &'a self,
        function: &'a str,
        symbol: &'a str,
        interval: TechnicalIndicatorInterval,
    ) -> IndicatorBuilder<'a> {
        IndicatorBuilder::new(self, function, symbol, interval)
    }
}
