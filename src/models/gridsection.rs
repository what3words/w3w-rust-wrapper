use std::fmt;

use serde::Deserialize;

use crate::Coordinates;

use super::feature::Feature;

pub trait FormattedGridSection {
    fn format() -> &'static str;
}

#[derive(Debug, Deserialize)]
pub struct Line {
    pub start: Coordinates,
    pub end: Coordinates,
}

#[derive(Debug, Deserialize)]
pub struct GridSection {
    pub lines: Vec<Line>,
}

impl FormattedGridSection for GridSection {
    fn format() -> &'static str {
        "json"
    }
}

#[derive(Debug, Deserialize)]
pub struct GridSectionGeoJson {
    pub features: Vec<Feature<Geometry>>,
    #[serde(rename = "type")]
    pub kind: String,
}

impl FormattedGridSection for GridSectionGeoJson {
    fn format() -> &'static str {
        "geojson"
    }
}

#[derive(Debug, Deserialize)]
pub struct Geometry {
    pub coordinates: Vec<Vec<Vec<f32>>>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug)]
pub struct BoundingBox {
    southwest: Coordinates,
    northeast: Coordinates,
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.southwest.lat, self.southwest.lng, self.northeast.lat, self.northeast.lng
        )
    }
}

impl BoundingBox {
    pub fn new(sw_lat: f64, sw_lng: f64, ne_lat: f64, ne_lng: f64) -> Self {
        Self {
            southwest: Coordinates {
                lat: sw_lat,
                lng: sw_lng,
            },
            northeast: Coordinates {
                lat: ne_lat,
                lng: ne_lng,
            },
        }
    }
}
