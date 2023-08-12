use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InserShortLink{
    pub url: String,
    pub tag: Option<String>
}