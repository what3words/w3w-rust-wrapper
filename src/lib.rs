pub use self::models::{
    autosuggest::{Autosuggest, AutosuggestResult, AutosuggestSelection, Suggestion},
    gridsection::{BoundingBox, GridSection, GridSectionGeoJson},
    language::{AvailableLanguages, Language},
    location::{Address, AddressGeoJson, ConvertTo3wa, ConvertToCoordinates, Coordinates, Square},
};
pub use self::service::{Error, What3words};

mod models;
mod service;
