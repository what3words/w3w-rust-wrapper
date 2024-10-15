use crate::{
    error::Error,
    models::autosuggest::{Autosuggest, AutosuggestOptions, Suggestion},
    models::error::ErrorResponse,
    models::gridsection::{GridSectionGeoJson, GridSectionJson},
    models::language::AvailableLanguages,
    models::location::{Address, Coordinates},
};
use http::HeaderMap;
use regex::Regex;
use serde::de::DeserializeOwned;
use std::{any::TypeId, collections::HashMap, env};

const DEFAULT_W3W_API_BASE_URL: &str = "https://api.what3words.com/v3";
const HEADER_WHAT3WORDS_API_KEY: &str = "X-Api-Key";
const W3W_WRAPPER: &str = "X-W3W-Wrapper";

pub struct What3words {
    api_key: &'static str,
    host: &'static str,
    headers: HeaderMap,
}

impl What3words {
    pub fn new(api_key: &'static str) -> Self {
        Self {
            api_key,
            headers: HeaderMap::new(),
            host: DEFAULT_W3W_API_BASE_URL,
        }
    }

    pub fn header(mut self, key: &'static str, value: &'static str) -> Self {
        self.headers.insert(key, value.parse().unwrap());
        self
    }

    pub fn hostname(mut self, host: &'static str) -> Self {
        self.host = host;
        self
    }

    pub async fn convert_to_3wa(&self, coordinates: &Coordinates) -> Result<Address, Error> {
        let mut params = HashMap::new();
        let location = format!("{},{}", coordinates.lat, coordinates.lng);
        params.insert("coordinates", location);
        let url = format!("{}/convert-to-3wa", self.host);
        let result = self.request::<Address>(url, Some(params)).await;
        result.map(|address| address)
    }

    pub async fn convert_to_coordinates(&self, what3words: &'static str) -> Result<Address, Error> {
        let mut params = HashMap::new();
        params.insert("words", what3words.to_string());
        let url = format!("{}/convert-to-coordinates", self.host);
        let result = self.request::<Address>(url, Some(params)).await;
        result.map(|address| address)
    }

    pub async fn available_languages(&self) -> Result<AvailableLanguages, Error> {
        let url = format!("{}/available-languages", self.host);
        let result = self.request::<AvailableLanguages>(url, None).await;
        result.map(|languages| languages)
    }

    pub async fn grid_section<T: 'static>(&self, bounding_box: &'static str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let mut params = HashMap::new();
        params.insert("bounding-box", bounding_box.to_string());
        let url = format!("{}/grid-section", self.host);
        if TypeId::of::<T>() == TypeId::of::<GridSectionGeoJson>() {
            params.insert("format", "geojson".to_string());
        } else if TypeId::of::<T>() == TypeId::of::<GridSectionJson>() {
            params.insert("format", "json".to_string());
        }
        self.request::<T>(url, Some(params)).await
    }

    pub async fn autosuggest(
        &self,
        input: &'static str,
        options: Option<&AutosuggestOptions>,
    ) -> Result<Autosuggest, Error> {
        let mut params = options
            .map(|option| option.to_hash_map())
            .unwrap_or(HashMap::<&str, String>::new());
        params.insert("input", input.to_string());
        let url = format!("{}/autosuggest", self.host);
        let result = self.request::<Autosuggest>(url, Some(params)).await;
        result.map(|autosuggest| autosuggest)
    }

    pub async fn autosuggest_with_coordinates(
        &self,
        input: &'static str,
        options: Option<&AutosuggestOptions>,
    ) -> Result<Autosuggest, Error> {
        let mut params = options
            .map(|option| option.to_hash_map())
            .unwrap_or(HashMap::<&str, String>::new());
        params.insert("input", input.to_string());
        let url = format!("{}/autosuggest-with-coordinates", self.host);
        let result = self.request::<Autosuggest>(url, Some(params)).await;
        result.map(|autosuggest| autosuggest)
    }

    pub async fn autosuggest_selection(
        &self,
        input: &'static str,
        suggestion: &Suggestion,
        options: Option<&AutosuggestOptions>,
    ) -> Result<(), Error> {
        let params = match options {
            Some(opts) => {
                let mut map = opts.to_hash_map();
                map.insert("rank", suggestion.rank.to_string());
                map.insert("selection", suggestion.words.clone());
                map.insert("raw-input", input.to_string());
                if let Some(input_type) = opts.input_type.as_ref() {
                    if input_type == "text" {
                        map.insert("source-api", "text".to_string());
                    }
                }
                map
            }
            None => HashMap::<&str, String>::new(),
        };
        let url = format!("{}/autosuggest-selection", self.host);
        let result = self.request::<()>(url, Some(params)).await;
        result.map(|autosuggest| autosuggest)
    }

    pub fn is_possible_3wa(&self, input: &'static str) -> bool {
        let pattern = Regex::new(r#"^/*(?:[^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]{1,}|'<,.>?/"";:£§º©®\s]+[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+|[^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+([\u0020\u00A0][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+){1,3}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+([\u0020\u00A0][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+){1,3}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+([\u0020\u00A0][^0-9`~!@#$%^&*()+\-_=[{\\]}\|'<,.>?/"";:£§º©®\s]+){1,3})"#).unwrap();
        pattern.is_match(input)
    }
    pub fn find_possible_3wa(&self, input: &'static str) -> Vec<String> {
        let pattern = Regex::new(r#"[^0-9`~!@#$%^&*()+\-_=[{\\]}\\|'<,.>?/"";:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\\|'<,.>?/"";:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=[{\\]}\\|'<,.>?/"";:£§º©®\s]{1,}"#).unwrap();
        pattern
            .find_iter(input)
            .map(|mat| mat.as_str().to_string())
            .collect()
    }

    pub fn is_valid_3wa(&self, input: &'static str) -> bool {
        if self.is_possible_3wa(input) {
            if let Ok(suggestion) = futures::executor::block_on(
                self.autosuggest(input, Some(&AutosuggestOptions::default().n_result("1"))),
            ) {
                return suggestion
                    .suggestions
                    .first()
                    .map_or(false, |s| s.words == input);
            }
        }
        false
    }

    async fn request<T>(
        &self,
        url: String,
        params: Option<HashMap<&str, String>>,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let user_agent = format!(
            "what3words-rust/{} ({})",
            env!("CARGO_PKG_VERSION"),
            env::consts::OS
        );

        let response = reqwest::Client::new()
            .get(&url)
            .query(&params)
            .headers(self.headers.clone())
            .header(W3W_WRAPPER, &user_agent)
            .header(HEADER_WHAT3WORDS_API_KEY, self.api_key)
            .send()
            .await
            .map_err(Error::from)?;

        if !response.status().is_success() {
            let error_response = response
                .json::<ErrorResponse>()
                .await
                .map_err(Error::from)?;
            return Err(Error::ApiError(error_response));
        }
        match response.content_length() {
            Some(0) => Ok(serde_json::from_str("null").unwrap()),
            _ => response.json::<T>().await.map_err(Error::from),
        }
    }
}
