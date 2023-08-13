use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InserShortLink{
    pub url: String,
    pub tag: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InserShortLinkResult{
    pub url: String
}