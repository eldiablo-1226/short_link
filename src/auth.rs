use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use actix_web::*;

use once_cell::sync::OnceCell;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BasicAuthConfig {
    #[serde(rename = "USERNAME")]
    pub username: String,

    #[serde(rename = "PASSWORD")]
    pub password: String,
}

static BASIC_AUTH_CONF: OnceCell<BasicAuthConfig> = OnceCell::new();

pub fn init(conf: config::Config) {
    let conf = conf.try_deserialize::<BasicAuthConfig>().unwrap();
    BASIC_AUTH_CONF.set(conf).unwrap();
}

pub async fn basic_auth_validator(req: ServiceRequest, credentials: BasicAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match validate_credentials(credentials.user_id(), credentials.password().unwrap().trim()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err((AuthenticationError::from(config).into(), req))
            }
        }
        Err(_) => Err((AuthenticationError::from(config).into(), req)),
    }
}

fn validate_credentials(user_id: &str, user_password: &str) -> Result<bool, std::io::Error>
{
    let auth = BASIC_AUTH_CONF.get().unwrap();

    if user_id.eq(auth.username.as_str()) &&
        user_password.eq(auth.password.as_str())
    {
        return Ok(true);
    }

    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Authentication failed!"));
}