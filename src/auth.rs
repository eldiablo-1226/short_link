use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use actix_web::*;

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
    let user_name = std::env::var("USERNAME").expect("USERNAME must be set");
    let password = std::env::var("PASSWORD").expect("PASSWORD must be set");

    if user_id.eq(user_name.as_str()) && user_password.eq(password.as_str())
    {
        return Ok(true);
    }

    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Authentication failed!"));
}