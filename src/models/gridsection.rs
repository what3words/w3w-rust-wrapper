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
