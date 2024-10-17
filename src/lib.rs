pub use self::models::{
    autosuggest::{Autosuggest, AutosuggestResult, AutosuggestSelection, Suggestion},
    gridsection::{GridSection, GridSectionGeoJson, Line},
    language::{AvailableLanguages, Language},
    location::{Address, AddressGeoJson, ConvertTo3wa, ConvertToCoordinates, Coordinates, Square},
};
pub use self::service::{Result, What3words};

mod models;
mod service;
