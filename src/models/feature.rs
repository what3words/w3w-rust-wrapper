use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Feature<T> {
    pub bbox: Option<Vec<f64>>,
    pub geometry: T,
    #[serde(rename = "type")]
    pub kind: String,
    pub properties: serde_json::Value,
}
