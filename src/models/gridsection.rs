use serde::Deserialize;

use crate::Coordinates;

use super::feature::Feature;

pub trait FormattedGridSection {}

#[derive(Debug, Deserialize)]
pub struct Line {
    pub start: Coordinates,
    pub end: Coordinates,
}

#[derive(Debug, Deserialize)]
pub struct GridSection {
    pub lines: Vec<Line>,
}

#[derive(Debug, Deserialize)]
pub struct GridSectionGeoJson {
    pub features: Vec<Feature<Geometry>>,
    #[serde(rename = "type")]
    pub kind: String,
}

impl FormattedGridSection for GridSectionGeoJson {}
impl FormattedGridSection for GridSection {}

#[derive(Debug, Deserialize)]
pub struct Geometry {
    pub coordinates: Vec<Vec<Vec<f32>>>,
    #[serde(rename = "type")]
    pub kind: String,
}
