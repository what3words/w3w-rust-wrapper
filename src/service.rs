use crate::models::{
    autosuggest::{Autosuggest, AutosuggestResult, AutosuggestSelection},
    error::ErrorResult,
    gridsection::{BoundingBox, FormattedGridSection},
    language::AvailableLanguages,
    location::{ConvertTo3wa, ConvertToCoordinates, FormattedAddress},
};
use http::{HeaderMap, HeaderName, HeaderValue};
use regex::Regex;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::{collections::HashMap, env, fmt};

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

    pub async fn convert_to_3wa<T: DeserializeOwned + FormattedAddress>(
        &self,
        conversion_options: ConvertTo3wa,
    ) -> Result<T> {
        let url = format!("{}/convert-to-3wa", self.host);
        let mut params = conversion_options.to_hash_map();
        params.insert("format", T::format().to_string());
        self.request(url, Some(params)).await
    }

    pub async fn convert_to_coordinates<T: DeserializeOwned + FormattedAddress>(
        &self,
        conversion_options: ConvertToCoordinates,
    ) -> Result<T> {
        let url = format!("{}/convert-to-coordinates", self.host);
        let mut params = conversion_options.to_hash_map();
        params.insert("format", T::format().to_string());
        self.request(url, Some(params)).await
    }

    pub async fn available_languages(&self) -> Result<AvailableLanguages> {
        let url = format!("{}/available-languages", self.host);
        self.request(url, None).await
    }

    pub async fn grid_section<T: DeserializeOwned + FormattedGridSection>(
        &self,
        bounding_box: BoundingBox,
    ) -> Result<T> {
        let mut params = HashMap::new();
        params.insert("bounding-box", bounding_box.to_string());
        let url = format!("{}/grid-section", self.host);
        params.insert("format", T::format().to_string());
        self.request(url, Some(params)).await
    }

    pub async fn autosuggest(&self, autosuggest: &Autosuggest) -> Result<AutosuggestResult> {
        let params = autosuggest.clone().to_hash_map();
        let url = format!("{}/autosuggest", self.host);
        self.request(url, Some(params)).await
    }

    pub async fn autosuggest_with_coordinates(
        &self,
        autosuggest: &Autosuggest,
    ) -> Result<AutosuggestResult> {
        let params = autosuggest.clone().to_hash_map();
        let url = format!("{}/autosuggest-with-coordinates", self.host);
        self.request(url, Some(params)).await
    }

    pub async fn autosuggest_selection(&self, selection: AutosuggestSelection) -> Result<()> {
        let params = selection.to_hash_map();
        let url = format!("{}/autosuggest-selection", self.host);
        self.request(url, Some(params)).await
    }

    pub fn did_you_mean(&self, input: impl Into<String>) -> bool {
        let pattern = Regex::new(
            r#"^\/?[^0-9`~!@#$%^&*()+\-=\[\{\]}\\|'<>.,?\/\"";:£§º©®\s]{1,}[.\uFF61\u3002\uFF65\u30FB\uFE12\u17D4\u0964\u1362\u3002:။^_۔։ ,\\\/+'&\\:;|\u3000-]{1,2}[^0-9`~!@#$%^&*()+\-=\[\{\]}\\|'<>.,?\/\";:£§º©®\s]{1,}[.\uFF61\u3002\uFF65\u30FB\uFE12\u17D4\u0964\u1362\u3002:။^_۔։ ,\\\/+'&\\:;|\u3000-]{1,2}[^0-9`~!@#$%^&*()+\-=\[\{\]}\\|'<>.,?\/\";:£§º©®\s]{1,}$"#,
        );
        match pattern {
            Ok(pattern) => pattern.is_match(&input.into()),
            Err(_) => false,
        }
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
                .map(|matched| matched.as_str().to_string())
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
                    .map_or(false, |suggestion| suggestion.words == input_str);
            }
        }
        false
    }

    async fn request<T: DeserializeOwned>(
        &self,
        url: String,
        params: Option<HashMap<&str, String>>,
    ) -> Result<T> {
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

        // panic!("{:?}", response.text().await);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{
            autosuggest::Autosuggest,
            location::{ConvertTo3wa, ConvertToCoordinates},
        },
        Address, GridSection, Suggestion,
    };

    use mockito::{Matcher, Server};
    use serde_json::json;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_convert_to_3wa() {
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
                    "words": "filled.count.soap",
                    "language": "en",
                    "map": "https://w3w.co/filled.count.soap"
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: Address = w3w
            .convert_to_3wa(ConvertTo3wa::new(51.521251, -0.203586))
            .await
            .unwrap();
        mock.assert_async().await;
        assert_eq!(result.words, "filled.count.soap");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_convert_to_coordinates() {
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();
        let mock = mock_server
            .mock("GET", "/convert-to-coordinates")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("words".into(), "filled.count.soap".into()),
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
                    "words": "filled.count.soap",
                    "language": "en",
                    "map": "https://w3w.co/filled.count.soap"
                })
                .to_string(),
            )
            .create();

        let w3w = What3words::new("TEST_API_KEY").hostname(&url);
        let result: Address = w3w
            .convert_to_coordinates(ConvertToCoordinates::new("filled.count.soap"))
            .await
            .unwrap();
        mock.assert_async().await;
        assert_eq!(result.coordinates.lng, -0.203586);
        assert_eq!(result.coordinates.lat, 51.521251);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_available_languages() {
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
        let result = w3w.available_languages().await.unwrap();
        mock.assert_async().await;
        assert_eq!(result.languages.len(), 2);
        assert_eq!(result.languages[0].code, "en");
        assert_eq!(result.languages[1].code, "fr");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_grid_section() {
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
            .grid_section(BoundingBox::new(52.207988, 0.116126, 52.208867, 0.11754))
            .await
            .unwrap();
        mock.assert_async().await;
        assert_eq!(result.lines.len(), 1);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_autosuggest() {
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
            .autosuggest(&Autosuggest::new("filled.count.soap"))
            .await
            .unwrap();
        mock.assert_async().await;
        assert_eq!(result.suggestions.len(), 1);
        assert_eq!(result.suggestions[0].words, "filled.count.soap");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_did_you_mean() {
        let w3w = What3words::new("TEST_API_KEY");
        assert!(w3w.did_you_mean("filled｡count｡soap"));
        assert!(w3w.did_you_mean("filled count soap"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_is_possible_3wa() {
        let w3w = What3words::new("TEST_API_KEY");
        assert!(w3w.is_possible_3wa("filled.count.soap"));
        assert!(!w3w.is_possible_3wa("invalid.3wa.address"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_find_possible_3wa() {
        let w3w = What3words::new("TEST_API_KEY");
        let result = w3w.find_possible_3wa("This is a test with filled.count.soap in it.");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "filled.count.soap");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_is_valid_3wa() {
        let mut mock_server = Server::new_async().await;
        let url = mock_server.url();

        let mock = mock_server
            .mock("GET", "/autosuggest")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("input".into(), "filled.count.soap".into()),
                Matcher::UrlEncoded("n-result".into(), "1".into()),
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
        let result = w3w.is_valid_3wa("filled.count.soap");
        mock.assert_async().await;
        assert!(result);
        assert!(!w3w.is_valid_3wa("invalid.3wa.address"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_custom_headers() {
        let w3w = What3words::new("TEST_API_KEY").header("Custom-Header", "CustomValue");

        assert_eq!(
            w3w.headers.get("Custom-Header"),
            Some(&HeaderValue::from_static("CustomValue"))
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_custom_hostname() {
        let w3w = What3words::new("TEST_API_KEY").hostname("https://custom.api.url");
        assert_eq!(w3w.host, "https://custom.api.url");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_autosuggest_with_coordinates() {
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
            .autosuggest_with_coordinates(&Autosuggest::new("filled.count.soap"))
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(result.suggestions.len(), 1);
        assert_eq!(result.suggestions[0].words, "filled.count.soap");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_autosuggest_selection() {
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
            .autosuggest_selection(AutosuggestSelection::new("i.h.r", &suggestion))
            .await;
        mock.assert_async().await;
        assert!(result.is_ok());
    }
}
