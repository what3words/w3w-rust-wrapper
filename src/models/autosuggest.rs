use super::location::{Coordinates, Square};
use crate::service::ToHashMap;
use serde::Deserialize;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct Autosuggest {
    pub input: Option<String>,
    pub n_result: Option<String>,
    pub focus: Option<String>,
    pub n_focus_result: Option<String>,
    pub clip_to_country: Option<String>,
    pub clip_to_bounding_box: Option<String>,
    pub clip_to_circle: Option<String>,
    pub clip_to_polygon: Option<String>,
    pub input_type: Option<String>,
    pub language: Option<String>,
    pub prefer_land: Option<bool>,
    pub locale: Option<String>,
}

impl ToHashMap for Autosuggest {
    fn to_hash_map<'a>(&self) -> HashMap<&'a str, String> {
        let mut map = HashMap::new();
        if let Some(ref input) = &self.input {
            map.insert("input", input.clone());
        }
        if let Some(ref n_result) = &self.n_result {
            map.insert("n-result", n_result.clone());
        }
        if let Some(ref focus) = &self.focus {
            map.insert("focus", focus.clone());
        }
        if let Some(ref n_focus_result) = &self.n_focus_result {
            map.insert("n-focus-result", n_focus_result.clone());
        }
        if let Some(ref clip_to_country) = &self.clip_to_country {
            map.insert("clip-to-country", clip_to_country.clone());
        }
        if let Some(ref clip_to_bounding_box) = &self.clip_to_bounding_box {
            map.insert("clip-to-bounding-box", clip_to_bounding_box.clone());
        }
        if let Some(ref clip_to_circle) = &self.clip_to_circle {
            map.insert("clip-to-circle", clip_to_circle.clone());
        }
        if let Some(ref clip_to_polygon) = &self.clip_to_polygon {
            map.insert("clip-to-polygon", clip_to_polygon.clone());
        }
        if let Some(ref input_type) = &self.input_type {
            map.insert("input-type", input_type.clone());
        }
        if let Some(ref language) = &self.language {
            map.insert("language", language.clone());
        }
        if let Some(ref locale) = &self.locale {
            map.insert("locale", locale.clone());
        }
        if let Some(ref prefer_land) = &self.prefer_land {
            map.insert("prefer-land", prefer_land.to_string());
        }
        map
    }
}

impl Autosuggest {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: Some(input.into()),
            n_result: None,
            focus: None,
            n_focus_result: None,
            clip_to_country: None,
            clip_to_bounding_box: None,
            clip_to_circle: None,
            clip_to_polygon: None,
            input_type: None,
            language: None,
            prefer_land: None,
            locale: None,
        }
    }
    pub fn n_result(mut self, n_result: impl Into<String>) -> Self {
        self.n_result = Some(n_result.into());
        self
    }

    pub fn focus(mut self, focus: impl Into<String>) -> Self {
        self.focus = Some(focus.into());
        self
    }

    pub fn n_focus_result(mut self, n_focus_result: impl Into<String>) -> Self {
        self.n_focus_result = Some(n_focus_result.into());
        self
    }

    pub fn clip_to_country(mut self, clip_to_country: impl Into<String>) -> Self {
        self.clip_to_country = Some(clip_to_country.into());
        self
    }

    pub fn clip_to_bounding_box(mut self, clip_to_bounding_box: impl Into<String>) -> Self {
        self.clip_to_bounding_box = Some(clip_to_bounding_box.into());
        self
    }

    pub fn clip_to_circle(mut self, clip_to_circle: impl Into<String>) -> Self {
        self.clip_to_circle = Some(clip_to_circle.into());
        self
    }

    pub fn clip_to_polygon(mut self, clip_to_polygon: impl Into<String>) -> Self {
        self.clip_to_polygon = Some(clip_to_polygon.into());
        self
    }

    pub fn input_type(mut self, input_type: impl Into<String>) -> Self {
        self.input_type = Some(input_type.into());
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn prefer_land(mut self, prefer_land: impl Into<bool>) -> Self {
        self.prefer_land = Some(prefer_land.into());
        self
    }

    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }
}

impl fmt::Display for Autosuggest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct AutosuggestSelection {
    pub raw_input: Option<String>,
    pub options: Option<Autosuggest>,
    pub suggestion: Option<Suggestion>,
}

impl ToHashMap for AutosuggestSelection {
    fn to_hash_map<'a>(&self) -> HashMap<&'a str, String> {
        let mut map = HashMap::new();
        if let Some(ref raw_input) = &self.raw_input {
            map.insert("raw-input", raw_input.clone());
        }
        if let Some(ref suggestion) = &self.suggestion {
            map.insert("rank", suggestion.rank.to_string());
            map.insert("selection", suggestion.words.clone());
        }
        if let Some(ref options) = &self.options {
            let options_map = options.to_hash_map();
            map.extend(options_map);
        }
        map
    }
}

impl AutosuggestSelection {
    pub fn new(raw_input: impl Into<String>, suggestion: &Suggestion) -> Self {
        Self {
            raw_input: Some(raw_input.into()),
            options: None,
            suggestion: Some(suggestion.clone()),
        }
    }
    pub fn options(mut self, options: &Autosuggest) -> Self {
        self.options = Some(options.clone());
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Suggestion {
    pub country: String,
    #[serde(rename = "nearestPlace")]
    pub nearest_place: String,
    pub words: String,
    pub rank: u32,
    pub language: String,
    #[serde(rename = "distanceToFocusKm")]
    pub distance_to_focus_km: Option<u32>,
    pub square: Option<Square>,
    pub coordinates: Option<Coordinates>,
    pub map: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AutosuggestResult {
    pub suggestions: Vec<Suggestion>,
}
