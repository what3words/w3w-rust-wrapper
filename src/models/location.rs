use serde::Deserialize;
use std::{collections::HashMap, fmt};

use crate::service::{Error, ToHashMap, Validator};

use super::feature::Feature;

pub trait FormattedAddress {
    fn format() -> &'static str;
}

#[derive(Debug, Clone)]
pub struct ConvertTo3wa {
    coordinates: Option<Coordinates>,
    locale: Option<String>,
    language: Option<String>,
}

impl ToHashMap for ConvertTo3wa {
    fn to_hash_map<'a>(&self) -> Result<HashMap<&'a str, String>, Error> {
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
        Ok(map)
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
    fn to_hash_map<'a>(&self) -> Result<HashMap<&'a str, String>, Error> {
        let mut map = HashMap::new();
        if let Some(ref locale) = &self.locale {
            map.insert("locale", locale.into());
        }
        if let Some(ref words) = &self.words {
            map.insert("words", words.into());
        }
        Ok(map)
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

#[derive(Debug, Clone, Deserialize, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct Circle {
    lat: f64,
    lng: f64,
    radius: u32,
}

impl Circle {
    pub fn new(lat: f64, lng: f64, radius: u32) -> Self {
        Self { lat, lng, radius }
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.lat, self.lng, self.radius)
    }
}

#[derive(Debug, Clone)]
pub struct Polygon {
    coordinates: Vec<Coordinates>,
}

impl Polygon {
    pub fn new(coordinates: Vec<Coordinates>) -> Self {
        Self { coordinates }
    }
}

impl Validator for Polygon {
    fn validate(&self) -> Result<(), Error> {
        if self.coordinates.len() < 4 {
            return Err(Error::InvalidParameter(
                "A polygon must have at least 4 coordinates.",
            ));
        }
        if self.coordinates.len() > 25 {
            return Err(Error::InvalidParameter(
                "A polygon must have no more than 25 coordinates.",
            ));
        }
        if self.coordinates.first() != self.coordinates.last() {
            return Err(Error::InvalidParameter(
                "The first and last coordinates must be the same to form a closed polygon.",
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let coords = self
            .coordinates
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{coords}")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Square {
    pub southwest: Coordinates,
    pub northeast: Coordinates,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
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
mod location_tests {
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
        if let Ok(map) = convert.to_hash_map() {
            assert_eq!(
                map.get("coordinates"),
                Some(&"51.521251,-0.203586".to_string())
            );
            assert_eq!(map.get("locale"), Some(&"en".to_string()));
            assert_eq!(map.get("language"), Some(&"en".to_string()));
        }
    }

    #[test]
    fn test_convert_to_coordinates_to_hash_map() {
        let convert = ConvertToCoordinates::new("index.home.raft").locale("en");
        if let Ok(map) = convert.to_hash_map() {
            assert_eq!(map.get("locale"), Some(&"en".to_string()));
            assert_eq!(map.get("words"), Some(&"index.home.raft".to_string()));
        }
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
