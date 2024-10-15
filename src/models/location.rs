use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Square {
    pub southwest: Coordinates,
    pub northeast: Coordinates,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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
    pub map: String,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
