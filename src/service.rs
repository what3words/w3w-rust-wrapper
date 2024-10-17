use crate::models::{
    autosuggest::{Autosuggest, AutosuggestResult, AutosuggestSelection},
    error::ErrorResult,
    gridsection::{FormattedGridSection, GridSection, GridSectionGeoJson},
    language::AvailableLanguages,
    location::{Address, AddressGeoJson, ConvertTo3wa, ConvertToCoordinates, FormattedAddress},
};
use http::{HeaderMap, HeaderName, HeaderValue};
use regex::Regex;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::{any::TypeId, collections::HashMap, env, fmt};

pub trait ToHashMap {
    fn to_hash_map<'a>(&self) -> HashMap<&'a str, String>;
}

#[derive(Debug)]
pub enum Error {
    Network(String),
    Http(String),
    Api(ErrorResult),
    Decode(String),
    Unknown(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Network(msg) => write!(f, "Network error: {}", msg),
            Error::Http(msg) => write!(f, "HTTP error: {}", msg),
            Error::Api(res) => {
                write!(f, "W3W error: {} {}", res.error.code, res.error.message)
            }
            Error::Decode(msg) => write!(f, "Decode error: {}", msg),
            Error::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if error.is_request() {
            Error::Http(error.to_string())
        } else if error.is_connect() {
            Error::Network(error.to_string())
        } else if error.is_decode() {
            Error::Decode(error.to_string())
        } else {
            Error::Unknown(error.to_string())
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

const DEFAULT_W3W_API_BASE_URL: &str = "https://api.what3words.com/v3";
const HEADER_WHAT3WORDS_API_KEY: &str = "X-Api-Key";
const W3W_WRAPPER: &str = "X-W3W-Wrapper";

pub struct What3words {
    api_key: String,
    host: String,
    headers: HeaderMap,
}

impl What3words {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            headers: HeaderMap::new(),
            host: DEFAULT_W3W_API_BASE_URL.into(),
        }
    }

    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        if let (Ok(header_name), Ok(header_value)) =
            (HeaderName::try_from(key), HeaderValue::try_from(value))
        {
            self.headers.insert(header_name, header_value);
        }
        self
    }

    pub fn hostname(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub async fn convert_to_3wa<T>(&self, conversion_options: ConvertTo3wa) -> Result<T>
    where
        T: 'static + DeserializeOwned + FormattedAddress,
    {
        let url = format!("{}/convert-to-3wa", self.host);
        let mut params = conversion_options.to_hash_map();
        if TypeId::of::<T>() == TypeId::of::<AddressGeoJson>() {
            params.insert("format", "geojson".to_string());
        } else if TypeId::of::<T>() == TypeId::of::<Address>() {
            params.insert("format", "json".to_string());
        }
        self.request::<T>(url, Some(params)).await
    }

    pub async fn convert_to_coordinates<T>(
        &self,
        conversion_options: ConvertToCoordinates,
    ) -> Result<T>
    where
        T: 'static + DeserializeOwned + FormattedAddress,
    {
        let url = format!("{}/convert-to-coordinates", self.host);
        let mut params = conversion_options.to_hash_map();
        if TypeId::of::<T>() == TypeId::of::<AddressGeoJson>() {
            params.insert("format", "geojson".to_string());
        } else if TypeId::of::<T>() == TypeId::of::<Address>() {
            params.insert("format", "json".to_string());
        }
        self.request::<T>(url, Some(params)).await
    }

    pub async fn available_languages(&self) -> Result<AvailableLanguages> {
        let url = format!("{}/available-languages", self.host);
        self.request::<AvailableLanguages>(url, None).await
    }

    pub async fn grid_section<T>(&self, bounding_box: impl Into<String>) -> Result<T>
    where
        T: 'static + DeserializeOwned + FormattedGridSection,
    {
        let mut params = HashMap::new();
        params.insert("bounding-box", bounding_box.into());
        let url = format!("{}/grid-section", self.host);
        if TypeId::of::<T>() == TypeId::of::<GridSectionGeoJson>() {
            params.insert("format", "geojson".to_string());
        } else if TypeId::of::<T>() == TypeId::of::<GridSection>() {
            params.insert("format", "json".to_string());
        }
        self.request::<T>(url, Some(params)).await
    }

    pub async fn autosuggest(&self, autosuggest: &Autosuggest) -> Result<AutosuggestResult> {
        let params = autosuggest.clone().to_hash_map();
        let url = format!("{}/autosuggest", self.host);
        self.request::<AutosuggestResult>(url, Some(params)).await
    }

    pub async fn autosuggest_with_coordinates(
        &self,
        autosuggest: &Autosuggest,
    ) -> Result<AutosuggestResult> {
        let params = autosuggest.clone().to_hash_map();
        let url = format!("{}/autosuggest-with-coordinates", self.host);
        self.request::<AutosuggestResult>(url, Some(params)).await
    }

    pub async fn autosuggest_selection(&self, selection: AutosuggestSelection) -> Result<()> {
        let params = selection.to_hash_map();
        let url = format!("{}/autosuggest-selection", self.host);
        self.request::<()>(url, Some(params)).await
    }

    pub fn is_possible_3wa(&self, input: impl Into<String>) -> bool {
        let pattern = Regex::new(
            r#"^/*(?:[^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]{1,}|'<,.>?/"";:£§º©®\s]+[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+|[^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+([\u0020\u00A0][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+){1,3}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+([\u0020\u00A0][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+){1,3}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+([\u0020\u00A0][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+){1,3})"#,
        );
        match pattern {
            Ok(pattern) => pattern.is_match(&input.into()),
            Err(_) => false,
        }
    }

    pub fn find_possible_3wa(&self, input: impl Into<String>) -> Vec<String> {
        let pattern = Regex::new(
            r#"[^0-9`~!@#$%^&*()+\-_=[{\\]}\\|'<,.>?/"";:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\\|'<,.>?/"";:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\\|'<,.>?/"";:£§º©®\s]{1,}"#,
        );
        match pattern {
            Ok(pattern) => pattern
                .find_iter(&input.into())
                .map(|mat| mat.as_str().to_string())
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    pub fn is_valid_3wa(&self, input: impl Into<String>) -> bool {
        let input_str = input.into();
        if self.is_possible_3wa(&input_str) {
            if let Ok(suggestion) = futures::executor::block_on(
                self.autosuggest(&Autosuggest::new(&input_str).n_result("1")),
            ) {
                return suggestion
                    .suggestions
                    .first()
                    .map_or(false, |s| s.words == input_str);
            }
        }
        false
    }

    async fn request<T>(&self, url: String, params: Option<HashMap<&str, String>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_agent = format!(
            "what3words-rust/{} ({})",
            env!("CARGO_PKG_VERSION"),
            env::consts::OS
        );

        let response = Client::new()
            .get(&url)
            .query(&params)
            .headers(self.headers.clone())
            .header(W3W_WRAPPER, &user_agent)
            .header(HEADER_WHAT3WORDS_API_KEY, &self.api_key)
            .send()
            .await
            .map_err(Error::from)?;

        if !response.status().is_success() {
            let error_response = response.json::<ErrorResult>().await.map_err(Error::from)?;
            return Err(Error::Api(error_response));
        }
        match response.content_length() {
            // Captures successful responses with no content
            Some(0) => Ok(serde_json::from_str("null").unwrap()),
            _ => response.json::<T>().await.map_err(Error::from),
        }
    }
}
