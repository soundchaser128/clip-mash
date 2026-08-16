use std::time::Duration;

use serde::Deserialize;
use tracing::debug;
use utoipa::ToSchema;

use super::{Range, SpeedController};
use crate::helpers::math;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccellerateParameters {
    pub slide_range: Range,
    pub session_duration: f64,
    pub start_speed: f64,
    pub end_speed: f64,
}

impl AccellerateParameters {
    pub fn session_duration(&self) -> Duration {
        Duration::from_secs_f64(self.session_duration)
    }
}

pub struct AccellerateController {
    parameters: AccellerateParameters,
}

impl AccellerateController {
    pub fn new(parameters: AccellerateParameters) -> Self {
        Self { parameters }
    }
}

impl SpeedController for AccellerateController {
    fn next_speed(&mut self, elapsed: Duration) -> f64 {
        let t = elapsed.as_secs_f64() / self.parameters.session_duration().as_secs_f64();
        let speed = math::lerp(self.parameters.start_speed, self.parameters.end_speed, t);
        debug!("next speed: {speed}");
        speed
    }

    fn slide_range(&self) -> Range {
        self.parameters.slide_range
    }

    fn initial_speed(&self) -> f64 {
        self.parameters.start_speed
    }

    fn should_continue(&self, elapsed: Duration) -> bool {
        elapsed <= self.parameters.session_duration()
    }
}
