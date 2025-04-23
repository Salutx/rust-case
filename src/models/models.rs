use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Category {
    pub category: String,
    pub items: Vec<String>,
}
