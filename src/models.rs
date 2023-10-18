use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct InserShortLink{
    #[validate(url)]
    pub url: String,
    pub tag: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InserShortLinkResult{
    pub url: String
}