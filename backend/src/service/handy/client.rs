use std::fmt;

use color_eyre::eyre::{eyre, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::Result;

const DEFAULT_API_URL: &str = "https://www.handyfeeling.com/api/handy/v2";
const KEY_HEADER: &str = "X-Connection-Key";

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    HAMP = 0,
    HSSP = 1,
    HDSP = 2,
    MAINTENANCE = 3,
    HBSP = 4,
}

impl TryFrom<u8> for Mode {
    type Error = color_eyre::Report;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::HAMP),
            1 => Ok(Mode::HSSP),
            2 => Ok(Mode::HDSP),
            3 => Ok(Mode::MAINTENANCE),
            4 => Ok(Mode::HBSP),
            _ => Err(eyre!("Invalid mode value: {}", value)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ConnectedResponse {
    /// Machine connected status
    pub connected: bool,
}

/// ModeUpdate : Mode update payload
#[derive(Debug, Serialize)]
pub struct ModeUpdate {
    pub mode: u8,
}

#[derive(Debug, Deserialize)]
pub struct GetModeResponse {
    pub mode: u8,
}

#[derive(Debug, Serialize)]
pub struct SlideSettings {
    pub min: u32,
    pub max: u32,
    /// Flag to indicate if the slide operation is fixed. A fixed operation moves the active slider area (min-max) offset to the new min or max value.
    #[serde(rename = "fixed", skip_serializing_if = "Option::is_none")]
    pub fixed: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct HampVelocityPercent {
    pub velocity: f64,
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
    base_url: String,
}

impl HandyClient {
    pub fn new(key: String) -> Self {
        Self {
            key,
            client: Client::new(),
            base_url: DEFAULT_API_URL.to_string(),
        }
    }

    pub fn with_endpoint(key: String, endpoint: String) -> Self {
        Self {
            key,
            client: Client::new(),
            base_url: endpoint,
        }
    }
}

impl IHandyClient for HandyClient {
    async fn is_connected(&self) -> Result<bool> {
        let url = format!("{}/connected", self.base_url);
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
        let update = ModeUpdate { mode: mode as u8 };
        info!("json body: {}", serde_json::to_string(&update)?);
        let url = format!("{}/mode", self.base_url);
        let response = self
            .client
            .put(&url)
            .header(KEY_HEADER, &self.key)
            .json(&update)
            .send()
            .await?;
        if response.status().is_success() {
            info!("set_mode response: {:?}", response);
            Ok(())
        } else {
            let body = response.text().await?;
            Err(eyre!("Failed to set mode: '{}'", body))
        }
    }

    async fn get_mode(&self) -> Result<Mode> {
        let url = format!("{}/mode", self.base_url);
        let response: GetModeResponse = self
            .client
            .get(&url)
            .header(KEY_HEADER, &self.key)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        response.mode.try_into()
    }

    async fn start(&self, velocity: f64) -> Result<()> {
        let url = format!("{}/hamp/start", self.base_url);
        let response = self
            .client
            .put(url)
            .header(KEY_HEADER, &self.key)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        info!("start response: {:?}", response);

        self.set_velocity(velocity).await?;

        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let url = format!("{}/hamp/stop", self.base_url);
        let response = self
            .client
            .put(url)
            .header(KEY_HEADER, &self.key)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        info!("stop response: {:?}", response);

        Ok(())
    }

    async fn set_stroke_range(&self, min: u32, max: u32) -> Result<()> {
        let settings = SlideSettings {
            min,
            max,
            fixed: Some(true),
        };
        let url = format!("{}/slide", self.base_url);
        let response = self
            .client
            .put(url)
            .header(KEY_HEADER, &self.key)
            .json(&settings)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        info!("set_stroke_range response: {:?}", response);

        Ok(())
    }

    async fn set_velocity(&self, velocity: f64) -> Result<()> {
        let body = HampVelocityPercent { velocity };
        let url = format!("{}/hamp/velocity", self.base_url);
        let response = self
            .client
            .put(url)
            .header(KEY_HEADER, &self.key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        info!("set_velocity response: {:?}", response);
        Ok(())
    }
}
