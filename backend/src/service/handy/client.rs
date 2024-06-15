use std::fmt;

use crate::Result;
use color_eyre::eyre::Context;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

const API_URL: &str = "https://www.handyfeeling.com/api/handy/v2";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "0")]
    HAMP,
    #[serde(rename = "1")]
    HSSP,
    #[serde(rename = "2")]
    HDSP,
    #[serde(rename = "3")]
    MAINTENANCE,
    #[serde(rename = "4")]
    HBSP,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectedResponse {
    /// Machine connected status
    pub connected: bool,
}

pub trait IHandyClient {
    async fn is_connected(&self) -> Result<bool>;
    async fn set_mode(&self, mode: Mode) -> Result<()>;
    async fn get_mode(&self) -> Result<Mode>;
    async fn start(&self, velocity: f64) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn set_stroke_range(&self, min: u32, max: u32) -> Result<()>;
    async fn set_velocity(&self, velocity: f64) -> Result<()>;
}

pub struct HandyClient {
    key: String,
    client: Client,
}

impl HandyClient {
    pub fn new(key: String) -> Self {
        Self {
            key,
            client: Client::new(),
        }
    }
}

impl IHandyClient for HandyClient {
    async fn is_connected(&self) -> Result<bool> {
        // let result = is_connected(&self.config, &self.key).await?;
        let url = format!("{API_URL}/connected");
        let response: ConnectedResponse = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response.connected)
    }

    async fn set_mode(&self, mode: Mode) -> Result<()> {
        use handy_api::apis::base_api::set_mode;

        info!("setting mode to {mode:?}");
        let result = set_mode(&self.config, &self.key, ModeUpdate { mode }).await?;
        match result {
            handy_api::models::SetMode200Response::ModeUpdateResponse(_) => Ok(()),
            handy_api::models::SetMode200Response::ErrorResponse(e) => {
                Err(ErrorResponse(e.error).into())
            }
        }
    }

    async fn get_mode(&self) -> Result<Mode> {
        use handy_api::apis::base_api::get_mode;

        let result = get_mode(&self.config, &self.key).await?;
        match result {
            GetMode200Response::GetMode200ResponseOneOf(r) => Ok(r.mode),
            GetMode200Response::ErrorResponse(e) => Err(ErrorResponse(e.error).into()),
        }
    }

    async fn start(&self, velocity: f64) -> Result<()> {
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

    async fn stop(&self) -> Result<()> {
        use handy_api::apis::hamp_api::hamp_stop;
        let response = hamp_stop(&self.config, &self.key).await?;
        match response {
            handy_api::models::HampStop200Response::ErrorResponse(e) => {
                Err(ErrorResponse(e.error).into())
            }
            handy_api::models::HampStop200Response::HampStopResponse(_) => Ok(()),
        }
    }

    async fn set_stroke_range(&self, min: u32, max: u32) -> Result<()> {
        use handy_api::apis::slide_api::set_slide;

        let body = SlideSettings {
            min,
            max,
            fixed: Some(true),
        };
        let result = set_slide(&self.config, &self.key, body)
            .await
            .wrap_err("setting slide settings")?;
        match result {
            SetSlide200Response::ErrorResponse(e) => Err(ErrorResponse(e.error).into()),
            SetSlide200Response::SlideUpdateResponse(_) => Ok(()),
        }
    }

    async fn set_velocity(&self, velocity: f64) -> Result<()> {
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
