//! Module for crypto real time data
//!
//! APIs under this section provide a wide range of data feed for digital and
//! crypto currencies such as Bitcoin.
//!
//! You can read about [Cryptocurrency][crypto_currency] API and what it returns
//! on alphavantage documentation
//!
//! [crypto_currency]: https://www.alphavantage.co/documentation/#digital-currency

use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

use serde::Deserialize;

use crate::api::ApiClient;
use crate::deserialize::from_str;
use crate::error::{detect_common_helper_error, Error, Result};

/// Store Meta Data Information
#[derive(Deserialize, Clone, Default)]
struct MetaData {
    #[serde(rename = "1. Information")]
    information: String,
    #[serde(rename = "2. Digital Currency Code")]
    digital_code: String,
    #[serde(rename = "3. Digital Currency Name")]
    digital_name: String,
    #[serde(rename = "4. Market Code")]
    market_code: String,
    #[serde(rename = "5. Market Name")]
    market_name: String,
    #[serde(rename = "6. Last Refreshed")]
    last_refreshed: String,
    #[serde(rename = "7. Time Zone")]
    time_zone: String,
}

/// Struct which stores Crypto data
#[derive(Default, Debug, Clone)]
pub struct Entry {
    time: String,
    market_open: f64,
    usd_open: f64,
    market_high: f64,
    usd_high: f64,
    market_low: f64,
    usd_low: f64,
    market_close: f64,
    usd_close: f64,
    volume: f64,
    market_cap: f64,
}

impl Entry {
    /// Return time
    #[must_use]
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Return market open value
    #[must_use]
    pub fn market_open(&self) -> f64 {
        self.market_open
    }

    /// Return usd open value
    #[must_use]
    pub fn usd_open(&self) -> f64 {
        self.usd_open
    }

    /// Return market high value
    #[must_use]
    pub fn market_high(&self) -> f64 {
        self.market_high
    }

    /// Return usd high value
    #[must_use]
    pub fn usd_high(&self) -> f64 {
        self.usd_high
    }

    /// Return market low value
    #[must_use]
    pub fn market_low(&self) -> f64 {
        self.market_low
    }

    /// Return usd low value
    #[must_use]
    pub fn usd_low(&self) -> f64 {
        self.usd_low
    }

    /// Return market close value
    #[must_use]
    pub fn market_close(&self) -> f64 {
        self.market_close
    }

    /// Return usd close value
    #[must_use]
    pub fn usd_close(&self) -> f64 {
        self.usd_close
    }

    /// Return volume
    #[must_use]
    pub fn volume(&self) -> f64 {
        self.volume
    }

    /// Return market cap
    #[must_use]
    pub fn market_cap(&self) -> f64 {
        self.market_cap
    }
}

/// Struct which holds Crypto currency information
#[derive(Default)]
pub struct Crypto {
    meta_data: MetaData,
    entry: Vec<Entry>,
}

impl Crypto {
    /// Return meta data information
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let crypto = api
    ///         .crypto(alpha_vantage::crypto::CryptoFunction::Daily, "BTC", "CNY")
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let information = crypto.information();
    ///     assert_eq!(information, "Daily Prices and Volumes for Digital Currency");
    /// }
    /// ```
    #[must_use]
    pub fn information(&self) -> &str {
        self.return_meta_string("information")
    }

    /// Return digital currency code
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let crypto = api
    ///         .crypto(alpha_vantage::crypto::CryptoFunction::Daily, "BTC", "CNY")
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let digital_code = crypto.digital_code();
    ///     assert_eq!(digital_code, "BTC");
    /// }
    /// ```
    #[must_use]
    pub fn digital_code(&self) -> &str {
        self.return_meta_string("digital code")
    }

    /// Return digital currency name
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let crypto = api
    ///         .crypto(alpha_vantage::crypto::CryptoFunction::Daily, "BTC", "CNY")
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let digital_name = crypto.digital_name();
    ///     assert_eq!(digital_name, "Bitcoin");
    /// }
    /// ```
    #[must_use]
    pub fn digital_name(&self) -> &str {
        self.return_meta_string("digital name")
    }

    /// Return market code
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let crypto = api
    ///         .crypto(alpha_vantage::crypto::CryptoFunction::Daily, "BTC", "CNY")
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let market_code = crypto.market_code();
    ///     assert_eq!(market_code, "CNY");
    /// }
    /// ```
    #[must_use]
    pub fn market_code(&self) -> &str {
        self.return_meta_string("market code")
    }

    /// Return market name
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo", reqwest::Client::new());
    ///     let crypto = api
    ///         .crypto(alpha_vantage::crypto::CryptoFunction::Daily, "BTC", "CNY")
    ///         .json()
    ///         .await
    ///         .unwrap();
    ///     let market_name = crypto.market_name();
    ///     assert_eq!(market_name, "Chinese Yuan");
    /// }
    /// ```
    #[must_use]
    pub fn market_name(&self) -> &str {
        self.return_meta_string("market name")
    }

    /// Return last refreshed time
    #[must_use]
    pub fn last_refreshed(&self) -> &str {
        self.return_meta_string("last refreshed")
    }

    /// Return time zone of all data time
    #[must_use]
    pub fn time_zone(&self) -> &str {
        self.return_meta_string("time zone")
    }

    /// Return a entry
    #[must_use]
    pub fn entry(&self) -> &Vec<Entry> {
        &self.entry
    }

    /// Return meta string
    fn return_meta_string(&self, which_val: &str) -> &str {
        match which_val {
            "information" => &self.meta_data.information,
            "digital code" => &self.meta_data.digital_code,
            "digital name" => &self.meta_data.digital_name,
            "market code" => &self.meta_data.market_code,
            "market name" => &self.meta_data.market_name,
            "time zone" => &self.meta_data.time_zone,
            "last refreshed" => &self.meta_data.last_refreshed,
            _ => "",
        }
    }
}

/// Struct to help out for creation of struct Entry
#[derive(Deserialize, Clone)]
struct EntryHelper {
    #[serde(rename = "1b. open (USD)", deserialize_with = "from_str")]
    open_usd: f64,
    #[serde(rename = "2b. high (USD)", deserialize_with = "from_str")]
    high_usd: f64,
    #[serde(rename = "3b. low (USD)", deserialize_with = "from_str")]
    low_usd: f64,
    #[serde(rename = "4b. close (USD)", deserialize_with = "from_str")]
    close_usd: f64,
    #[serde(rename = "5. volume", deserialize_with = "from_str")]
    volume: f64,
    #[serde(rename = "6. market cap (USD)", deserialize_with = "from_str")]
    market_cap: f64,
    #[serde(flatten)]
    market_data: HashMap<String, String>,
}

/// Struct to help out for creation of struct Crypto
#[derive(Deserialize)]
pub(crate) struct CryptoHelper {
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Note")]
    note: Option<String>,
    #[serde(rename = "Meta Data")]
    meta_data: Option<MetaData>,
    #[serde(flatten)]
    entry: Option<HashMap<String, HashMap<String, EntryHelper>>>,
}

impl CryptoHelper {
    /// Function which convert `CryptoHelper` to `Crypto`
    pub(crate) fn convert(self) -> Result<Crypto> {
        detect_common_helper_error(self.information, self.error_message, self.note)?;

        if self.meta_data.is_none() || self.entry.is_none() {
            return Err(Error::EmptyResponse);
        }

        let mut vec_entry = Vec::new();
        // Can use unwrap here is none condition is checked already
        for value in self.entry.unwrap().values() {
            for key in value.keys() {
                let entry_helper = value
                    .get(key)
                    .expect("failed to get value from Crypto hashmap");

                let mut entry = Entry {
                    time: key.to_string(),
                    usd_open: entry_helper.open_usd,
                    usd_high: entry_helper.high_usd,
                    usd_low: entry_helper.low_usd,
                    usd_close: entry_helper.close_usd,
                    market_cap: entry_helper.market_cap,
                    volume: entry_helper.volume,
                    ..Entry::default()
                };

                for key in entry_helper.market_data.keys() {
                    let value = &entry_helper.market_data[key];
                    let f64_value = f64::from_str(value).unwrap();
                    if key.contains("1a") {
                        entry.market_open = f64_value;
                    } else if key.contains("2a") {
                        entry.market_high = f64_value;
                    } else if key.contains("3a") {
                        entry.market_low = f64_value;
                    } else if key.contains("4a") {
                        entry.market_close = f64_value;
                    }
                }
                vec_entry.push(entry);
            }
        }

        Ok(Crypto {
            entry: vec_entry,
            meta_data: self.meta_data.unwrap(),
        })
    }
}

/// trait which helps for performing some common operation on Vec<Entry>
pub trait VecEntry {
    /// Find a entry with a given time as a input return none if no entry found
    fn find(&self, time: &str) -> Option<&Entry>;
    /// Return a entry which is of latest time period
    fn latest(&self) -> Entry;
    /// Return a top n latest Entry
    /// # Errors
    /// If n is greater than no of entry
    fn latest_n(&self, n: usize) -> Result<Vec<&Entry>>;
}

impl VecEntry for Vec<Entry> {
    #[must_use]
    fn find(&self, time: &str) -> Option<&Entry> {
        self.iter().find(|&entry| entry.time == time)
    }

    #[must_use]
    fn latest(&self) -> Entry {
        let mut latest = &Entry::default();
        for entry in self {
            if latest.time < entry.time {
                latest = entry;
            }
        }
        latest.clone()
    }

    fn latest_n(&self, n: usize) -> Result<Vec<&Entry>> {
        let mut time_list = self.iter().map(|entry| &entry.time).collect::<Vec<_>>();
        time_list.sort_by_key(|w| cmp::Reverse(*w));

        if n > time_list.len() {
            return Err(Error::DesiredNumberOfEntryNotPresent(time_list.len()));
        }

        let mut full_list = Vec::<&Entry>::new();

        for time in &time_list[0..n] {
            full_list.push(self.find(time).unwrap());
        }

        Ok(full_list)
    }
}

/// Builder to help create `Crypto`
pub struct CryptoBuilder<'a> {
    api_client: &'a ApiClient<'a>,
    function: CryptoFunction,
    symbol: &'a str,
    market: &'a str,
}

impl<'a> CryptoBuilder<'a> {
    /// Create new `CryptoBuilder` with help of `APIClient`
    #[must_use]
    pub fn new(
        api_client: &'a ApiClient,
        function: CryptoFunction,
        symbol: &'a str,
        market: &'a str,
    ) -> Self {
        Self {
            api_client,
            function,
            symbol,
            market,
        }
    }

    fn create_url(&self) -> String {
        let function_name = match self.function {
            CryptoFunction::Daily => "DIGITAL_CURRENCY_DAILY",
            CryptoFunction::Weekly => "DIGITAL_CURRENCY_WEEKLY",
            CryptoFunction::Monthly => "DIGITAL_CURRENCY_MONTHLY",
        };

        format!(
            "query?function={}&symbol={}&market={}",
            &function_name, &self.symbol, &self.market
        )
    }

    /// Returns JSON data struct
    ///
    /// # Errors
    /// Raise error if data obtained cannot be properly converted to struct or
    /// API returns any 4 possible known errors
    pub async fn json(&self) -> Result<Crypto> {
        let url = self.create_url();
        let crypto_helper: CryptoHelper = self.api_client.get_json(&url).await?;
        crypto_helper.convert()
    }
}

/// Enum for declaring function for crypto series by defining which type of
/// crypto series to be returned
#[derive(Clone)]
pub enum CryptoFunction {
    /// returns the daily historical time series for a digital currency (e.g.,
    /// BTC) traded on a specific market (e.g., CNY/Chinese Yuan), refreshed
    /// daily at midnight (UTC). Prices and volumes are quoted in both the
    /// market-specific currency and USD.
    Daily,
    /// returns the weekly historical time series for a digital currency (e.g.,
    /// BTC) traded on a specific market (e.g., CNY/Chinese Yuan), refreshed
    /// daily at midnight (UTC). Prices and volumes are quoted in both the
    /// market-specific currency and USD.
    Weekly,
    /// returns the monthly historical time series for a digital currency (e.g.,
    /// BTC) traded on a specific market (e.g., CNY/Chinese Yuan), refreshed
    /// daily at midnight (UTC). Prices and volumes are quoted in both the
    /// market-specific currency and USD.
    Monthly,
}
