use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub title: String,
}
