use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Language {
    #[serde(rename = "nativeName")]
    pub native_name: String,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AvailableLanguages {
    pub languages: Vec<Language>,
}
