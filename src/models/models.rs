use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Category {
    pub category: String,
    pub items: Vec<String>,
}
