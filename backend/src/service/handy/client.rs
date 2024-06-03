use std::fmt;

use handy_api::apis::configuration::Configuration;
use handy_api::models::{GetMode200Response, Mode, ModeUpdate};

use crate::Result;

#[derive(Debug)]
pub struct ErrorResponse(Option<Box<handy_api::models::ErrorResponseError>>);

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(e) => write!(
                f,
                "Encountered Handy error: {}, message: '{}', code={:?}, connected={}, additional data: {:?}",
                e.name, e.message, e.code, e.connected, e.data
            ),
            None => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for ErrorResponse {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

pub struct HandyClient {
    config: Configuration,
    key: String,
}

impl HandyClient {
    pub fn new(key: String) -> Self {
        Self {
            config: Default::default(),
            key,
        }
    }

    pub async fn is_connected(&self) -> Result<bool> {
        use handy_api::apis::base_api::is_connected;

        let result = is_connected(&self.config, &self.key).await?;
        Ok(result.connected)
    }

    pub async fn set_mode(&self, mode: Mode) -> Result<()> {
        use handy_api::apis::base_api::set_mode;

        set_mode(&self.config, &self.key, ModeUpdate { mode }).await?;
        Ok(())
    }

    pub async fn get_mode(&self) -> Result<Mode> {
        use handy_api::apis::base_api::get_mode;

        let result = get_mode(&self.config, &self.key).await?;
        match result {
            GetMode200Response::GetMode200ResponseOneOf(r) => Ok(r.mode),
            GetMode200Response::ErrorResponse(e) => Err(ErrorResponse(e.error).into()),
        }
    }
}
