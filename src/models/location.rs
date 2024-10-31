use serde::Deserialize;
use std::{collections::HashMap, fmt};

use crate::service::ToHashMap;

use super::feature::Feature;

pub trait FormattedAddress {
    fn format() -> &'static str;
}

#[derive(Debug, Clone, Default)]
pub struct ConvertTo3wa {
    coordinates: Option<Coordinates>,
    locale: Option<String>,
    language: Option<String>,
}

impl ToHashMap for ConvertTo3wa {
    fn to_hash_map<'a>(&self) -> HashMap<&'a str, String> {
        let mut map = HashMap::new();
        if let Some(coordinates) = &self.coordinates {
            map.insert(
                "coordinates",
                format!("{},{}", coordinates.lat, coordinates.lng),
            );
        }
        if let Some(ref locale) = &self.locale {
            map.insert("locale", locale.into());
        }
        if let Some(ref language) = &self.language {
            map.insert("language", language.into());
        }
        map
    }
}

impl ConvertTo3wa {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self {
            coordinates: Some(Coordinates::new(lat, lng)),
            locale: None,
            language: None,
        }
    }

    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct ConvertToCoordinates {
    locale: Option<String>,
    words: Option<String>,
}

impl ToHashMap for ConvertToCoordinates {
    fn to_hash_map<'a>(&self) -> HashMap<&'a str, String> {
        let mut map = HashMap::new();
        if let Some(ref locale) = &self.locale {
            map.insert("locale", locale.into());
        }
        if let Some(ref words) = &self.words {
            map.insert("words", words.into());
        }
        map
    }
}

impl ConvertToCoordinates {
    pub fn new(words: impl Into<String>) -> Self {
        Self {
            locale: None,
            words: Some(words.into()),
        }
    }
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.lat, self.lng)
    }
}

impl Coordinates {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Square {
    pub southwest: Coordinates,
    pub northeast: Coordinates,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Address {
    pub country: String,
    pub square: Square,
    #[serde(rename = "nearestPlace")]
    pub nearest_place: String,
    pub coordinates: Coordinates,
    pub words: String,
    pub language: String,
    pub locale: Option<String>,
    pub map: String,
}

impl FormattedAddress for Address {
    fn format() -> &'static str {
        "json"
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Geometry {
    pub coordinates: Vec<f64>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct AddressGeoJson {
    pub features: Vec<Feature<Geometry>>,
    #[serde(rename = "type")]
    pub kind: String,
}

impl FormattedAddress for AddressGeoJson {
    fn format() -> &'static str {
        "geojson"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates_display() {
        let coordinates = Coordinates {
            lat: 51.521251,
            lng: -0.203586,
        };
        assert_eq!(format!("{}", coordinates), "51.521251,-0.203586");
    }

    #[test]
    fn test_convert_to_3wa_to_hash_map() {
        let convert = ConvertTo3wa::new(51.521251, -0.203586)
            .locale("en")
            .language("en");
        let map = convert.to_hash_map();
        assert_eq!(
            map.get("coordinates"),
            Some(&"51.521251,-0.203586".to_string())
        );
        assert_eq!(map.get("locale"), Some(&"en".to_string()));
        assert_eq!(map.get("language"), Some(&"en".to_string()));
    }

    #[test]
    fn test_convert_to_coordinates_to_hash_map() {
        let convert = ConvertToCoordinates::new("index.home.raft").locale("en");
        let map = convert.to_hash_map();
        assert_eq!(map.get("locale"), Some(&"en".to_string()));
        assert_eq!(map.get("words"), Some(&"index.home.raft".to_string()));
    }

    #[test]
    fn test_convert_to_coordinates_new() {
        let convert = ConvertToCoordinates::new("index.home.raft");
        assert_eq!(convert.words, Some("index.home.raft".to_string()));
        assert_eq!(convert.locale, None);
    }

    #[test]
    fn test_convert_to_coordinates_locale() {
        let convert = ConvertToCoordinates::new("index.home.raft").locale("en");
        assert_eq!(convert.words, Some("index.home.raft".to_string()));
        assert_eq!(convert.locale, Some("en".to_string()));
    }
}
