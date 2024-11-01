use crate::models::{
    autosuggest::{Autosuggest, AutosuggestResult, AutosuggestSelection},
    error::ErrorResult,
    gridsection::{BoundingBox, FormattedGridSection},
    language::AvailableLanguages,
    location::{ConvertTo3wa, ConvertToCoordinates, FormattedAddress},
};
use http::{HeaderMap, HeaderName, HeaderValue};
use regex::Regex;
use reqwest::{blocking::Client, Client as ClientAsync};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, env, fmt};

pub(crate) trait ToHashMap {
    fn to_hash_map<'a>(&self) -> HashMap<&'a str, String>;
}

#[derive(Debug)]
pub enum Error {
    Network(String),
    Http(String),
    Api(String, String),
    Decode(String),
    Unknown(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Network(msg) => write!(f, "Network error: {}", msg),
            Error::Http(msg) => write!(f, "HTTP error: {}", msg),
            Error::Api(code, message) => {
                write!(f, "W3W error: {} {}", code, message)
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

pub(crate) type Result<T> = std::result::Result<T, Error>;

const DEFAULT_W3W_API_BASE_URL: &str = "https://api.what3words.com/v3";
const HEADER_WHAT3WORDS_API_KEY: &str = "X-Api-Key";
const W3W_WRAPPER: &str = "X-W3W-Wrapper";

pub struct What3words {
    api_key: String,
    host: String,
    headers: HeaderMap,
    user_agent: String,
}

impl What3words {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            headers: HeaderMap::new(),
            host: DEFAULT_W3W_API_BASE_URL.into(),
            user_agent: format!(
                "what3words-rust/{} ({})",
                env!("CARGO_PKG_VERSION"),
                env::consts::OS
            ),
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

    pub fn convert_to_3wa<T: FormattedAddress + DeserializeOwned>(
        &self,
        options: ConvertTo3wa,
    ) -> Result<T> {
        let url = format!("{}/convert-to-3wa", self.host);
        let mut params = options.to_hash_map();
        params.insert("format", T::format().to_string());
        self.request(url, Some(params))
    }

    pub async fn convert_to_3wa_async<T: FormattedAddress + DeserializeOwned>(
        &self,
        options: ConvertTo3wa,
    ) -> Result<T> {
        let url = format!("{}/convert-to-3wa", self.host);
        let mut params = options.to_hash_map();
        params.insert("format", T::format().to_string());
        self.request_async(url, Some(params)).await
    }

    pub fn convert_to_coordinates<T: FormattedAddress + DeserializeOwned>(
        &self,
        options: ConvertToCoordinates,
    ) -> Result<T> {
        let url = format!("{}/convert-to-coordinates", self.host);
        let mut params = options.to_hash_map();
        params.insert("format", T::format().to_string());
        self.request(url, Some(params))
    }

    pub async fn convert_to_coordinates_async<T: FormattedAddress + DeserializeOwned>(
        &self,
        options: ConvertToCoordinates,
    ) -> Result<T> {
        let url = format!("{}/convert-to-coordinates", self.host);
        let mut params = options.to_hash_map();
        params.insert("format", T::format().to_string());
        self.request_async(url, Some(params)).await
    }

    pub fn available_languages(&self) -> Result<AvailableLanguages> {
        let url = format!("{}/available-languages", self.host);
        self.request(url, None)
    }

    pub async fn available_languages_async(&self) -> Result<AvailableLanguages> {
        let url = format!("{}/available-languages", self.host);
        self.request_async(url, None).await
    }

    pub fn grid_section<T: DeserializeOwned + FormattedGridSection>(
        &self,
        bounding_box: BoundingBox,
    ) -> Result<T> {
        let mut params = HashMap::new();
        params.insert("bounding-box", bounding_box.to_string());
        let url = format!("{}/grid-section", self.host);
        params.insert("format", T::format().to_string());
        self.request(url, Some(params))
    }

    pub async fn grid_section_async<T: DeserializeOwned + FormattedGridSection>(
        &self,
        bounding_box: BoundingBox,
    ) -> Result<T> {
        let mut params = HashMap::new();
        params.insert("bounding-box", bounding_box.to_string());
        let url = format!("{}/grid-section", self.host);
        params.insert("format", T::format().to_string());
        self.request_async(url, Some(params)).await
    }

    pub fn autosuggest(&self, autosuggest: &Autosuggest) -> Result<AutosuggestResult> {
        let params = autosuggest.clone().to_hash_map();
        let url = format!("{}/autosuggest", self.host);
        self.request(url, Some(params))
    }

    pub async fn autosuggest_async(&self, autosuggest: &Autosuggest) -> Result<AutosuggestResult> {
        let params = autosuggest.clone().to_hash_map();
        let url = format!("{}/autosuggest", self.host);
        self.request_async(url, Some(params)).await
    }

    pub fn autosuggest_with_coordinates(
        &self,
        autosuggest: &Autosuggest,
    ) -> Result<AutosuggestResult> {
        let params = autosuggest.clone().to_hash_map();
        let url = format!("{}/autosuggest-with-coordinates", self.host);
        self.request(url, Some(params))
    }

    pub async fn autosuggest_with_coordinates_async(
        &self,
        autosuggest: &Autosuggest,
    ) -> Result<AutosuggestResult> {
        let params = autosuggest.clone().to_hash_map();
        let url = format!("{}/autosuggest-with-coordinates", self.host);
        self.request_async(url, Some(params)).await
    }

    pub fn autosuggest_selection(&self, selection: AutosuggestSelection) -> Result<()> {
        let params = selection.to_hash_map();
        let url = format!("{}/autosuggest-selection", self.host);
        self.request(url, Some(params))
    }

    pub async fn autosuggest_selection_async(&self, selection: AutosuggestSelection) -> Result<()> {
        let params = selection.to_hash_map();
        let url = format!("{}/autosuggest-selection", self.host);
        self.request_async(url, Some(params)).await
    }

    pub fn is_valid_3wa(&self, input: impl Into<String>) -> bool {
        let input_str = input.into();
        if self.is_possible_3wa(&input_str) {
            if let Ok(suggestion) = self.autosuggest(&Autosuggest::new(&input_str).n_results("1")) {
                return suggestion
                    .suggestions
                    .first()
                    .map_or(false, |suggestion| suggestion.words == input_str);
            }
        }
        false
    }

    pub async fn is_valid_3wa_async(&self, input: impl Into<String>) -> bool {
        let input_str = input.into();
        if self.is_possible_3wa(&input_str) {
            if let Ok(suggestion) = self
                .autosuggest_async(&Autosuggest::new(&input_str).n_results("1"))
                .await
            {
                return suggestion
                    .suggestions
                    .first()
                    .map_or(false, |suggestion| suggestion.words == input_str);
            }
        }
        false
    }

    pub fn did_you_mean(&self, input: impl Into<String>) -> bool {
        let pattern = Regex::new(
            r#"^/?[^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}[.\uFF61\u3002\uFF65\u30FB\uFE12\u17D4\u0964\u1362\u3002:။^_۔։ ,\\/+'&\\:;|\u3000-]{1,2}[^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}[.\uFF61\u3002\uFF65\u30FB\uFE12\u17D4\u0964\u1362\u3002:။^_۔։ ,\\/+'&\\:;|\u3000-]{1,2}[^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}$"#,
        ).unwrap();
        pattern.is_match(&input.into())
    }

    pub fn is_possible_3wa(&self, input: impl Into<String>) -> bool {
        let pattern = Regex::new(
            r#"^/*(?:[^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}|[^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}([\u0020\u00A0][^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]+){1,3}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}([\u0020\u00A0][^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]+){1,3}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}([\u0020\u00A0][^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]+){1,3})$"#,
        ).unwrap();
        pattern.is_match(&input.into())
    }

    pub fn find_possible_3wa(&self, input: impl Into<String>) -> Vec<String> {
        let pattern = Regex::new(
            r#"[^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}[.｡。･・︒។։။۔።।][^0-9`~!@#$%^&*()+\-_=\[\{\]}\\|'<>.,?/;:£§º©®\s]{1,}"#,
        ).unwrap();
        pattern
            .find_iter(&input.into())
            .map(|matched| matched.as_str().to_string())
            .collect()
    }

    fn request<T: DeserializeOwned>(
        &self,
        url: String,
        params: Option<HashMap<&str, String>>,
    ) -> Result<T> {
        let response = Client::new()
            .get(&url)
            .query(&params)
            .headers(self.headers.clone())
            .header(W3W_WRAPPER, &self.user_agent)
            .header(HEADER_WHAT3WORDS_API_KEY, &self.api_key)
            .send()
            .map_err(Error::from)?;

        if !response.status().is_success() {
            let error_response = response.json::<ErrorResult>().map_err(Error::from)?;
            return Err(Error::Api(
                error_response.error.code,
                error_response.error.message,
            ));
        }
        match response.content_length() {
            // Captures successful responses with no content
            Some(0) => Ok(serde_json::from_str("null").unwrap()),
            _ => response.json::<T>().map_err(Error::from),
        }
    }

    async fn request_async<T: DeserializeOwned>(
        &self,
        url: String,
        params: Option<HashMap<&str, String>>,
    ) -> Result<T> {
        let response = ClientAsync::new()
            .get(&url)
            .query(&params)
            .headers(self.headers.clone())
            .header(W3W_WRAPPER, &self.user_agent)
            .header(HEADER_WHAT3WORDS_API_KEY, &self.api_key)
            .send()
            .await
            .map_err(Error::from)?;

        if !response.status().is_success() {
            let error_response = response.json::<ErrorResult>().await.map_err(Error::from)?;
            return Err(Error::Api(
                error_response.error.code,
                error_response.error.message,
            ));
        }
        match response.content_length() {
            // Captures successful responses with no content
            Some(0) => Ok(serde_json::from_str("null").unwrap()),
            _ => response.json::<T>().await.map_err(Error::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{
            autosuggest::Autosuggest,
            location::{ConvertTo3wa, ConvertToCoordinates},
        },
        Address, AddressGeoJson, GridSection, Suggestion,
    };

    use mockito::{Matcher, Server};
    use serde_json::json;

    #[test]
    fn test_custom_headers() {
        let w3w = What3words::new("TEST_API_KEY").header("Custom-Header", "CustomValue");

        assert_eq!(
            w3w.headers.get("Custom-Header"),
            Some(&HeaderValue::from_static("CustomValue"))
        );
    }

    #[test]
    fn test_custom_hostname() {
        let w3w = What3words::new("TEST_API_KEY").hostname("https://custom.api.url");
        assert_eq!(w3w.host, "https://custom.api.url");
    }

    #[test]
    fn test_error_display() {
        let network_error = Error::Network(String::from("Connection lost"));
        assert_eq!(
            format!("{}", network_error),
            "Network error: Connection lost"
        );

        let http_error = Error::Http(String::from("404 Not Found"));
        assert_eq!(format!("{}", http_error), "HTTP error: 404 Not Found");

        let error_result = ErrorResult {
            error: crate::models::error::Error {
                code: String::from("400"),
                message: String::from("Bad Request"),
            },
        };
        let api_error = Error::Api(error_result.error.code, error_result.error.message);
        assert_eq!(format!("{}", api_error), "W3W error: 400 Bad Request");

        let decode_error = Error::Decode(String::from("Invalid JSON"));
        assert_eq!(format!("{}", decode_error), "Decode error: Invalid JSON");

        let unknown_error = Error::Unknown(String::from("Something went wrong"));
        assert_eq!(
            format!("{}", unknown_error),
            "Unknown error: Something went wrong"
        );
    }

    #[test]
    fn test_convert_to_3wa() {
        let words = "filled.count.soap";
        let mut mock_server = Server::new();
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/convert-to-3wa")
            .match_query(mockito::Matcher::AllOf(vec![
                Matcher::UrlEncoded("coordinates".into(), "51.521251,-0.203586".into()),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "country": "GB",
                    "square": {
                        "southwest": {
                            "lng": -0.203607,
                            "lat": 51.521241
                        },
                        "northeast": {
                            "lng": -0.203575,
                            "lat": 51.521261
                        }
                    },
                    "nearestPlace": "Bayswater, London",
                    "coordinates": {
                        "lng": -0.203586,
                        "lat": 51.521251
                    },
                    "words": words,
                    "language": "en",
                    "map": format!("https://w3w.co/{}", words)
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: Address = w3w
            .convert_to_3wa(ConvertTo3wa::new(51.521251, -0.203586))
            .unwrap();
        mock.assert();
        assert_eq!(result.words, words);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_convert_to_3wa_async() {
        let words = "filled.count.soap";
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/convert-to-3wa")
            .match_query(mockito::Matcher::AllOf(vec![
                Matcher::UrlEncoded("coordinates".into(), "51.521251,-0.203586".into()),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "country": "GB",
                    "square": {
                        "southwest": {
                            "lng": -0.203607,
                            "lat": 51.521241
                        },
                        "northeast": {
                            "lng": -0.203575,
                            "lat": 51.521261
                        }
                    },
                    "nearestPlace": "Bayswater, London",
                    "coordinates": {
                        "lng": -0.203586,
                        "lat": 51.521251
                    },
                    "words": words,
                    "language": "en",
                    "map": format!("https://w3w.co/{}", words)
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: Address = w3w
            .convert_to_3wa_async(ConvertTo3wa::new(51.521251, -0.203586))
            .await
            .unwrap();
        mock.assert_async().await;
        assert_eq!(result.words, "filled.count.soap");
    }

    #[test]
    fn test_convert_to_coordinates() {
        let words = "filled.count.soap";
        let mut mock_server = Server::new();
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/convert-to-coordinates")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("words".into(), words.into()),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "country": "GB",
                    "square": {
                        "southwest": {
                            "lng": -0.203607,
                            "lat": 51.521241
                        },
                        "northeast": {
                            "lng": -0.203575,
                            "lat": 51.521261
                        }
                    },
                    "nearestPlace": "Bayswater, London",
                    "coordinates": {
                        "lng": -0.203586,
                        "lat": 51.521251
                    },
                    "words": words,
                    "language": "en",
                    "map": format!("https://w3w.co/{}", words)
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: Address = w3w
            .convert_to_coordinates(ConvertToCoordinates::new(words))
            .unwrap();
        mock.assert();
        assert_eq!(result.coordinates.lng, -0.203586);
        assert_eq!(result.coordinates.lat, 51.521251);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_convert_to_coordinates_async() {
        let words = "filled.count.soap";
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/convert-to-coordinates")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("words".into(), words.into()),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "country": "GB",
                    "square": {
                        "southwest": {
                            "lng": -0.203607,
                            "lat": 51.521241
                        },
                        "northeast": {
                            "lng": -0.203575,
                            "lat": 51.521261
                        }
                    },
                    "nearestPlace": "Bayswater, London",
                    "coordinates": {
                        "lng": -0.203586,
                        "lat": 51.521251
                    },
                    "words": words,
                    "language": "en",
                    "map": format!("https://w3w.co/{}", words)
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: Address = w3w
            .convert_to_coordinates_async(ConvertToCoordinates::new(words))
            .await
            .unwrap();
        mock.assert_async().await;
        assert_eq!(result.coordinates.lng, -0.203586);
        assert_eq!(result.coordinates.lat, 51.521251);
    }

    #[test]
    fn test_convert_to_coordinates_bad_words() {
        let bad_words = "filled.count";
        let mut mock_server = Server::new();
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/convert-to-coordinates")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("words".into(), bad_words.into()),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_status(400)
            .with_body(
                json!({
                    "error": {
                        "code": "BadWords",
                        "message": "words must be a valid 3 word address, such as filled.count.soap or ///filled.count.soap"
                    }
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: std::result::Result<Address, Error> =
            w3w.convert_to_coordinates::<Address>(ConvertToCoordinates::new(bad_words));
        mock.assert();
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(format!("{}", error), "W3W error: BadWords words must be a valid 3 word address, such as filled.count.soap or ///filled.count.soap");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_convert_to_coordinates_async_bad_words() {
        let bad_words = "filled.count";
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/convert-to-coordinates")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("words".into(), bad_words.into()),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_status(400)
            .with_body(
                json!({
                    "error": {
                        "code": "BadWords",
                        "message": "words must be a valid 3 word address, such as filled.count.soap or ///filled.count.soap"
                    }
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: std::result::Result<Address, Error> = w3w
            .convert_to_coordinates_async::<Address>(ConvertToCoordinates::new(bad_words))
            .await;
        mock.assert_async().await;
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(format!("{}", error), "W3W error: BadWords words must be a valid 3 word address, such as filled.count.soap or ///filled.count.soap");
    }

    #[test]
    fn test_convert_to_coordinates_with_locale() {
        let words = "seruuhen.zemseg.dagaldah";
        let mut mock_server = Server::new();
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/convert-to-coordinates")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("words".into(), words.into()),
                Matcher::UrlEncoded("format".into(), "json".into()),
                Matcher::UrlEncoded("locale".into(), "mn_la".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "country": "GB",
                    "square": {
                        "southwest": {
                            "lng": -0.195543,
                            "lat": 51.520833
                        },
                        "northeast": {
                            "lng": -0.195499,
                            "lat": 51.52086
                        }
                    },
                    "nearestPlace": "Лондон",
                    "coordinates": {
                        "lng": -0.195521,
                        "lat": 51.520847
                    },
                    "words": words,
                    "language": "mn",
                    "locale": "mn_la",
                    "map": format!("https://w3w.co/{}", words),
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: Address = w3w
            .convert_to_coordinates(ConvertToCoordinates::new(words).locale("mn_la"))
            .unwrap();
        mock.assert();
        assert_eq!(result.words, words);
        assert_eq!(result.locale, Some("mn_la".to_string()));
    }

    #[test]
    fn test_convert_to_coordinates_geojson() {
        let words = "filled.count.soap";
        let mut mock_server = Server::new();
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/convert-to-coordinates")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("words".into(), words.into()),
                Matcher::UrlEncoded("format".into(), "geojson".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "features": [
                        {
                            "bbox": [
                                -0.195543,
                                51.520833,
                                -0.195499,
                                51.52086
                            ],
                            "geometry": {
                                "coordinates": [
                                    -0.195521,
                                    51.520847
                                ],
                                "type": "Point"
                            },
                            "type": "Feature",
                            "properties": {
                                "country": "GB",
                                "nearestPlace": "Bayswater, London",
                                "words": words,
                                "language": "en",
                                "map": format!("https://w3w.co/{}", words)
                            }
                        }
                    ],
                    "type": "FeatureCollection"
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: AddressGeoJson = w3w
            .convert_to_coordinates(ConvertToCoordinates::new(words))
            .unwrap();
        mock.assert();
        let bbox = result.features[0].bbox.as_ref().unwrap();
        assert_eq!(bbox[0], -0.195543);
        assert_eq!(bbox[1], 51.520833);
        assert_eq!(bbox[2], -0.195499);
        assert_eq!(bbox[3], 51.52086);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_convert_to_coordinates_geojson_async() {
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/convert-to-coordinates")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("words".into(), "filled.count.soap".into()),
                Matcher::UrlEncoded("format".into(), "geojson".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "features": [
                        {
                            "bbox": [
                                -0.195543,
                                51.520833,
                                -0.195499,
                                51.52086
                            ],
                            "geometry": {
                                "coordinates": [
                                    -0.195521,
                                    51.520847
                                ],
                                "type": "Point"
                            },
                            "type": "Feature",
                            "properties": {
                                "country": "GB",
                                "nearestPlace": "Bayswater, London",
                                "words": "filled.count.soap",
                                "language": "en",
                                "map": "https://w3w.co/filled.count.soap"
                            }
                        }
                    ],
                    "type": "FeatureCollection"
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: AddressGeoJson = w3w
            .convert_to_coordinates_async(ConvertToCoordinates::new("filled.count.soap"))
            .await
            .unwrap();
        mock.assert_async().await;
        let bbox = result.features[0].bbox.as_ref().unwrap();
        assert_eq!(bbox[0], -0.195543);
        assert_eq!(bbox[1], 51.520833);
        assert_eq!(bbox[2], -0.195499);
        assert_eq!(bbox[3], 51.52086);
    }

    #[test]
    fn test_available_languages() {
        let mut mock_server = Server::new();
        let url = mock_server.url();

        let mock = mock_server
            .mock("GET", "/available-languages")
            .with_status(200)
            .with_body(
                json!({
                    "languages": [
                        {
                            "nativeName": "English",
                            "code": "en",
                            "name": "English"
                        },
                        {
                            "nativeName": "Français",
                            "code": "fr",
                            "name": "French"
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result = w3w.available_languages().unwrap();
        mock.assert();
        assert_eq!(result.languages.len(), 2);
        assert_eq!(result.languages[0].code, "en");
        assert_eq!(result.languages[1].code, "fr");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_available_languages_async() {
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();

        let mock = mock_server
            .mock("GET", "/available-languages")
            .with_status(200)
            .with_body(
                json!({
                    "languages": [
                        {
                            "nativeName": "English",
                            "code": "en",
                            "name": "English"
                        },
                        {
                            "nativeName": "Français",
                            "code": "fr",
                            "name": "French"
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result = w3w.available_languages_async().await.unwrap();
        mock.assert_async().await;
        assert_eq!(result.languages.len(), 2);
        assert_eq!(result.languages[0].code, "en");
        assert_eq!(result.languages[1].code, "fr");
    }

    #[test]
    fn test_grid_section() {
        let mut mock_server = Server::new();
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/grid-section")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded(
                    "bounding-box".into(),
                    "52.207988,0.116126,52.208867,0.11754".into(),
                ),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "lines": [
                        {
                            "start": {
                                "lng": 0.116126,
                                "lat": 52.207988
                            },
                            "end": {
                                "lng": 0.11754,
                                "lat": 52.208867
                            }
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: GridSection = w3w
            .grid_section(BoundingBox::new(52.207988, 0.116126, 52.208867, 0.11754))
            .unwrap();
        mock.assert();
        assert_eq!(result.lines.len(), 1);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_grid_section_async() {
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/grid-section")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded(
                    "bounding-box".into(),
                    "52.207988,0.116126,52.208867,0.11754".into(),
                ),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "lines": [
                        {
                            "start": {
                                "lng": 0.116126,
                                "lat": 52.207988
                            },
                            "end": {
                                "lng": 0.11754,
                                "lat": 52.208867
                            }
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: GridSection = w3w
            .grid_section_async(BoundingBox::new(52.207988, 0.116126, 52.208867, 0.11754))
            .await
            .unwrap();
        mock.assert_async().await;
        assert_eq!(result.lines.len(), 1);
    }

    #[test]
    fn test_autosuggest() {
        let mut mock_server = Server::new();
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/autosuggest")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "input".into(),
                "filled.count.soap".into(),
            )]))
            .with_status(200)
            .with_body(
                json!({
                    "suggestions": [
                        {
                            "country": "GB",
                            "nearestPlace": "Bayswater, London",
                            "words": "filled.count.soap",
                            "rank": 1,
                            "language": "en"
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result = w3w
            .autosuggest(&Autosuggest::new("filled.count.soap"))
            .unwrap();
        mock.assert();
        assert_eq!(result.suggestions.len(), 1);
        assert_eq!(result.suggestions[0].words, "filled.count.soap");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_autosuggest_async() {
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/autosuggest")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "input".into(),
                "filled.count.soap".into(),
            )]))
            .with_status(200)
            .with_body(
                json!({
                    "suggestions": [
                        {
                            "country": "GB",
                            "nearestPlace": "Bayswater, London",
                            "words": "filled.count.soap",
                            "rank": 1,
                            "language": "en"
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result = w3w
            .autosuggest_async(&Autosuggest::new("filled.count.soap"))
            .await
            .unwrap();
        mock.assert_async().await;
        assert_eq!(result.suggestions.len(), 1);
        assert_eq!(result.suggestions[0].words, "filled.count.soap");
    }

    #[test]
    fn test_autosuggest_with_coordinates() {
        let mut mock_server = Server::new();
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/autosuggest-with-coordinates")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "input".into(),
                "filled.count.soap".into(),
            )]))
            .with_status(200)
            .with_body(
                json!({
                    "suggestions": [
                        {
                            "country": "GB",
                            "nearestPlace": "Bayswater, London",
                            "words": "filled.count.soap",
                            "rank": 1,
                            "language": "en"
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result = w3w
            .autosuggest_with_coordinates(&Autosuggest::new("filled.count.soap"))
            .unwrap();

        mock.assert();
        assert_eq!(result.suggestions.len(), 1);
        assert_eq!(result.suggestions[0].words, "filled.count.soap");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_autosuggest_with_coordinates_async() {
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/autosuggest-with-coordinates")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "input".into(),
                "filled.count.soap".into(),
            )]))
            .with_status(200)
            .with_body(
                json!({
                    "suggestions": [
                        {
                            "country": "GB",
                            "nearestPlace": "Bayswater, London",
                            "words": "filled.count.soap",
                            "rank": 1,
                            "language": "en"
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result = w3w
            .autosuggest_with_coordinates_async(&Autosuggest::new("filled.count.soap"))
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(result.suggestions.len(), 1);
        assert_eq!(result.suggestions[0].words, "filled.count.soap");
    }

    #[test]
    fn test_autosuggest_selection() {
        let mut mock_server = Server::new();
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/autosuggest-selection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("selection".into(), "filled.count.soap".into()),
                Matcher::UrlEncoded("rank".into(), "1".into()),
                Matcher::UrlEncoded("raw-input".into(), "i.h.r".into()),
            ]))
            .with_status(200)
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let suggestion = Suggestion {
            words: "filled.count.soap".to_string(),
            country: "GB".to_string(),
            nearest_place: "Bayswater, London".to_string(),
            distance_to_focus_km: None,
            rank: 1,
            square: None,
            coordinates: None,
            language: "en".to_string(),
            map: None,
        };
        let result = w3w.autosuggest_selection(AutosuggestSelection::new("i.h.r", &suggestion));
        mock.assert();
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_autosuggest_selection_async() {
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/autosuggest-selection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("selection".into(), "filled.count.soap".into()),
                Matcher::UrlEncoded("rank".into(), "1".into()),
                Matcher::UrlEncoded("raw-input".into(), "i.h.r".into()),
            ]))
            .with_status(200)
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let suggestion = Suggestion {
            words: "filled.count.soap".to_string(),
            country: "GB".to_string(),
            nearest_place: "Bayswater, London".to_string(),
            distance_to_focus_km: None,
            rank: 1,
            square: None,
            coordinates: None,
            language: "en".to_string(),
            map: None,
        };
        let result = w3w
            .autosuggest_selection_async(AutosuggestSelection::new("i.h.r", &suggestion))
            .await;
        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_valid_3wa_true() {
        let words = "filled.count.soap";
        let mut mock_server = Server::new();
        let url = mock_server.url();

        let mock = mock_server
            .mock("GET", "/autosuggest")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("input".into(), words.into()),
                Matcher::UrlEncoded("n-results".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "suggestions": [
                        {
                            "country": "GB",
                            "nearestPlace": "Bayswater, London",
                            "words": "filled.count.soap",
                            "rank": 1,
                            "language": "en"
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w: What3words = What3words::new("TEST_API_KEY").hostname(&url);
        assert!(w3w.is_valid_3wa(words));
        mock.assert();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_is_valid_3wa_async_true() {
        let words = "filled.count.soap";
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();

        let mock = mock_server
            .mock("GET", "/autosuggest")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("input".into(), words.into()),
                Matcher::UrlEncoded("n-results".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "suggestions": [
                        {
                            "country": "GB",
                            "nearestPlace": "Bayswater, London",
                            "words": "filled.count.soap",
                            "rank": 1,
                            "language": "en"
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w: What3words = What3words::new("TEST_API_KEY").hostname(&url);
        assert!(w3w.is_valid_3wa_async(words).await);
        mock.assert_async().await;
    }

    #[test]
    fn test_is_valid_3wa_false() {
        let words = "filled.count";
        let w3w: What3words = What3words::new("TEST_API_KEY");
        assert!(!w3w.is_valid_3wa(words));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_is_valid_3wa_async_false() {
        let words = "filled.count";
        let w3w: What3words = What3words::new("TEST_API_KEY");
        assert!(!w3w.is_valid_3wa_async(words).await);
    }

    #[test]
    fn test_is_valid_3wa_false_doesnt_match() {
        let words = "rust.is.cool";
        let mut mock_server = Server::new();
        let url = mock_server.url();

        let mock = mock_server
            .mock("GET", "/autosuggest")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("input".into(), words.into()),
                Matcher::UrlEncoded("n-results".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "suggestions": [
                        {
                            "country": "US",
                            "nearestPlace": "Huntington Station, New York",
                            "words": "rust.this.cool",
                            "rank": 1,
                            "language": "en"
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w: What3words = What3words::new("TEST_API_KEY").hostname(&url);
        assert!(!w3w.is_valid_3wa(words));
        mock.assert();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_is_valid_3wa_async_false_doesnt_match() {
        let words = "rust.is.cool";
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();

        let mock = mock_server
            .mock("GET", "/autosuggest")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("input".into(), words.into()),
                Matcher::UrlEncoded("n-results".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                json!({
                    "suggestions": [
                        {
                            "country": "US",
                            "nearestPlace": "Huntington Station, New York",
                            "words": "rust.this.cool",
                            "rank": 1,
                            "language": "en"
                        }
                    ]
                })
                .to_string(),
            )
            .create();

        let w3w: What3words = What3words::new("TEST_API_KEY").hostname(&url);
        assert!(!w3w.is_valid_3wa_async(words).await);
        mock.assert();
    }

    #[test]
    fn test_did_you_mean_true() {
        let w3w = What3words::new("TEST_API_KEY");
        assert!(w3w.did_you_mean("filled｡count｡soap"));
        assert!(w3w.did_you_mean("filled count soap"));
    }

    #[test]
    fn test_did_you_mean_false() {
        let w3w = What3words::new("TEST_API_KEY");
        assert!(!w3w.did_you_mean("filledcountsoap"));
    }

    #[test]
    fn test_is_possible_3wa_true() {
        let w3w = What3words::new("TEST_API_KEY");
        assert!(w3w.is_possible_3wa("filled.count.soap"));
    }

    #[test]
    fn test_is_possible_3wa_false() {
        let w3w = What3words::new("TEST_API_KEY");
        assert!(!w3w.is_possible_3wa("filled count soap"));
    }

    #[test]
    fn test_find_possible_3wa_true() {
        let w3w = What3words::new("TEST_API_KEY");
        let result = w3w.find_possible_3wa("This is a test with filled.count.soap in it.");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "filled.count.soap");
    }

    #[test]
    fn test_find_possible_3wa_false() {
        let w3w = What3words::new("TEST_API_KEY");
        let result = w3w.find_possible_3wa("This is a test with filled count soap in it.");
        assert_eq!(result.len(), 0);
    }
}
