pub use self::error::Error;
pub use self::models::{
    autosuggest::{Autosuggest, AutosuggestOptions, Suggestion},
    gridsection::{GridSectionGeoJson, GridSectionJson},
    location::{Address, Coordinates},
};
pub use self::service::What3words;

mod error;
mod models;
mod service;
