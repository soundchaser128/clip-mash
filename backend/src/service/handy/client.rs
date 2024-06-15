use std::fmt;

use handy_api::apis::configuration::Configuration;
use handy_api::models::{
    GetMode200Response, HampVelocityPercent, Mode, ModeUpdate, SetHampVelocityPercent200Response,
    SetSlide200Response, SlideSettings,
};

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

    pub async fn start(&self, velocity: f64) -> Result<()> {
        use handy_api::apis::hamp_api::start;

        let response = start(&self.config, &self.key).await?;
        match response {
            handy_api::models::Start200Response::ErrorResponse(e) => {
                return Err(ErrorResponse(e.error).into())
            }
            _ => {}
        }
        self.set_velocity(velocity).await?;

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        use handy_api::apis::hamp_api::hamp_stop;
        let response = hamp_stop(&self.config, &self.key).await?;
        match response {
            handy_api::models::HampStop200Response::ErrorResponse(e) => {
                Err(ErrorResponse(e.error).into())
            }
            handy_api::models::HampStop200Response::HampStopResponse(_) => Ok(()),
        }
    }

    pub async fn set_stroke_range(&self, min: f64, max: f64) -> Result<()> {
        use handy_api::apis::slide_api::set_slide;

        let body = SlideSettings {
            min,
            max,
            fixed: Some(true),
        };
        let result = set_slide(&self.config, &self.key, body).await?;
        match result {
            SetSlide200Response::ErrorResponse(e) => Err(ErrorResponse(e.error).into()),
            SetSlide200Response::SlideUpdateResponse(_) => Ok(()),
        }
    }

    pub async fn set_velocity(&self, velocity: f64) -> Result<()> {
        use handy_api::apis::hamp_api::set_hamp_velocity_percent;

        let body = HampVelocityPercent { velocity };
        let result = set_hamp_velocity_percent(&self.config, &self.key, body).await?;
        match result {
            SetHampVelocityPercent200Response::RpcResult(_) => Ok(()),
            SetHampVelocityPercent200Response::ErrorResponse(e) => {
                Err(ErrorResponse(e.error).into())
            }
        }
    }
}
