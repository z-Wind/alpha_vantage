use async_trait::async_trait;

use crate::error::{Error, Result};

#[async_trait]
/// Trait which can be implemented for all common library client for getting
/// output from server
/// surf and reqwest are two client which are supported with feature flag. If
/// you prefer alternate http client you can add support by implementing
/// `HttpClient` trait for client.
/// Some example of other client which can be used are isahc client
pub trait HttpClient {
    /// AlphaVantage provider output function which provides one field path
    /// where get GET request needs to be performed
    async fn get_alpha_vantage_provider_output(&self, path: String) -> Result<String>;

    /// RapidAPI provider function which provides two field path and api_key.
    /// Path needs to be set along with header x-rapidapi-host as
    /// alpha-vantage.p.rapidapi.com and header x-rapidapi-key same as
    /// api_key field
    async fn get_rapid_api_provider_output(&self, path: String, api_key: String) -> Result<String>;
}

#[cfg(feature = "reqwest-client")]
#[async_trait]
impl HttpClient for reqwest::Client {
    async fn get_alpha_vantage_provider_output(&self, path: String) -> Result<String> {
        self.get(&path)
            .send()
            .await
            .map_err(|_| Error::GetRequestFailed)?
            .text()
            .await
            .map_err(|_| Error::GetRequestFailed)
    }

    async fn get_rapid_api_provider_output(&self, path: String, api_key: String) -> Result<String> {
        self.get(&path)
            .header(
                String::from("x-rapidapi-host"),
                String::from("alpha-vantage.p.rapidapi.com"),
            )
            .header(String::from("x-rapidapi-key"), api_key)
            .send()
            .await
            .map_err(|_| Error::GetRequestFailed)?
            .text()
            .await
            .map_err(|_| Error::GetRequestFailed)
    }
}

#[cfg(feature = "surf-client")]
#[async_trait]
impl HttpClient for surf::Client {
    async fn get_alpha_vantage_provider_output(&self, path: String) -> Result<String> {
        self.get(path)
            .recv_string()
            .await
            .map_err(|_| Error::GetRequestFailed)
    }

    async fn get_rapid_api_provider_output(&self, path: String, api_key: String) -> Result<String> {
        self.get(&path)
            .header("x-rapidapi-host", "alpha-vantage.p.rapidapi.com")
            .header("x-rapidapi-key", api_key)
            .recv_string()
            .await
            .map_err(|_| Error::GetRequestFailed)
    }
}
