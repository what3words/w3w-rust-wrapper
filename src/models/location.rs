use serde::Deserialize;
use std::{collections::HashMap, fmt};

use crate::service::ToHashMap;

use super::feature::Feature;

pub trait FormattedAddress {
    fn format() -> &'static str;
}

#[derive(Debug, Clone, Default)]
pub struct ConvertTo3wa {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub locale: Option<String>,
    pub language: Option<String>,
}

impl ToHashMap for ConvertTo3wa {
    fn to_hash_map<'a>(&self) -> HashMap<&'a str, String> {
        let mut map = HashMap::new();
        if let (Some(lat), Some(lng)) = (self.lat, self.lng) {
            map.insert("coordinates", format!("{},{}", lat, lng));
        }
        if let Some(ref locale) = &self.locale {
            map.insert("locale", locale.clone());
        }
        if let Some(ref language) = &self.language {
            map.insert("language", language.clone());
        }
        map
    }
}

impl ConvertTo3wa {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self {
            lat: Some(lat),
            lng: Some(lng),
            locale: None,
            language: None,
        }
    }

    pub fn locale(&mut self, locale: impl Into<String>) -> &Self {
        self.locale = Some(locale.into());
        self
    }

    pub fn language(&mut self, language: impl Into<String>) -> &Self {
        self.language = Some(language.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct ConvertToCoordinates {
    pub locale: Option<String>,
    pub words: Option<String>,
}

impl ToHashMap for ConvertToCoordinates {
    fn to_hash_map<'a>(&self) -> HashMap<&'a str, String> {
        let mut map = HashMap::new();
        if let Some(ref locale) = &self.locale {
            map.insert("locale", locale.clone());
        }
        if let Some(ref words) = &self.words {
            map.insert("words", words.clone());
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
    pub fn locale(mut self, locale: String) -> Self {
        self.locale = Some(locale);
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
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Square {
    pub southwest: Coordinates,
    pub northeast: Coordinates,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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
    pub map: String,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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

impl fmt::Display for AddressGeoJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FormattedAddress for AddressGeoJson {
    fn format() -> &'static str {
        "geojson"
    }
}
