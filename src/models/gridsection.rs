use serde::Deserialize;

use crate::Coordinates;

#[derive(Debug, Deserialize)]
pub struct Line {
    pub start: Coordinates,
    pub end: Coordinates,
}

#[derive(Debug, Deserialize)]
pub struct GridSectionJson {
    pub lines: Vec<Line>,
}

#[derive(Debug, Deserialize)]
pub struct GridSectionGeoJson {
    pub features: Vec<Feature>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct Geometry {
    pub coordinates: Vec<Vec<Vec<f32>>>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct Feature {
    pub geometry: Geometry,
    #[serde(rename = "type")]
    pub kind: String,
    pub properties: serde_json::Value,
}
