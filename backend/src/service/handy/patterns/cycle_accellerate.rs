use std::time::Duration;

use serde::Deserialize;
use serde_with::{serde_as, DurationSeconds};
use tracing::debug;
use utoipa::ToSchema;

use super::{math, Range, SpeedController};

#[serde_as]
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CycleAccellerateParameters {
    pub start_range: Range,
    pub end_range: Range,
    pub slide_range: Range,

    #[serde_as(as = "DurationSeconds<u64>")]
    pub session_duration: Duration,

    #[serde_as(as = "DurationSeconds<u64>")]
    pub cycle_duration: Duration,
}

pub struct CycleAccellerateController {
    parameters: CycleAccellerateParameters,
}

impl SpeedController for CycleAccellerateController {
    fn next_speed(&mut self, elapsed: Duration) -> f64 {
        let cycle_value = self.get_cycle_position(elapsed);
        debug!("current cycle value: {}", cycle_value);

        let speed_bounds = self.get_speed_bounds(elapsed);
        debug!("current speed bounds: {:?}", speed_bounds);

        math::lerp(speed_bounds.min, speed_bounds.max, cycle_value)
    }

    fn slide_range(&self) -> Range {
        self.parameters.slide_range
    }

    fn initial_speed(&self) -> f64 {
        self.parameters.start_range.min
    }

    fn should_continue(&self, elapsed: Duration) -> bool {
        elapsed <= self.parameters.session_duration
    }
}

impl CycleAccellerateController {
    pub fn new(parameters: CycleAccellerateParameters) -> Self {
        Self { parameters }
    }

    fn get_cycle_position(&self, elapsed: Duration) -> f64 {
        let duration = self.parameters.cycle_duration.as_millis();
        let elapsed = elapsed.as_millis();
        let cycle_x = (elapsed % duration) as f64 / duration as f64;
        debug!("cycle_x: {}", cycle_x);
        let threshold = 0.5f64;
        let in_mul = threshold.powf(-1.0);
        let out_mul = (1.0 - threshold).powf(-1.0);

        if cycle_x < threshold {
            math::ease_in(cycle_x * in_mul)
        } else {
            math::ease_out((cycle_x - threshold) * out_mul)
        }
    }

    fn get_speed_bounds(&self, elapsed: Duration) -> Range {
        let start = self.parameters.start_range;
        let end = self.parameters.end_range;
        let t = elapsed.as_secs_f64();
        let duration = self.parameters.session_duration.as_secs_f64();
        let total_position = t / duration;
        debug!("total_position: {}", total_position);
        let min = math::lerp(start.min, end.min, total_position);
        let max = math::lerp(start.max, end.max, total_position);

        Range { min, max }
    }
}
