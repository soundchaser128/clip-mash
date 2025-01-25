use std::time::Duration;

use rand::rngs::StdRng;
use rand::Rng;
use serde::Deserialize;
use tracing::info;
use utoipa::ToSchema;

use super::{Range, SpeedController};
use crate::helpers::random::{create_seeded_rng, get_random_word};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RandomParameters {
    pub speed_range: Range,
    pub slide_range: Range,
    pub jitter: f64,
    pub seed: Option<String>,
    pub interval_range: Range,
}

pub struct RandomController {
    parameters: RandomParameters,
    last_speed: f64,
    rng: StdRng,
    last_change_at: Duration,
    next_interval: Duration,
}

fn pick_duration(Range { min, max }: Range, rng: &mut impl Rng) -> Duration {
    let secs = rng.gen_range(min..max).round();
    Duration::from_secs_f64(secs)
}

impl RandomController {
    pub fn new(parameters: RandomParameters) -> Self {
        let seed = parameters.seed.clone().unwrap_or_else(|| get_random_word());

        let mut rng = create_seeded_rng(Some(&seed));
        let next_interval = pick_duration(parameters.interval_range, &mut rng);

        Self {
            last_speed: parameters.speed_range.min,
            parameters,
            rng,
            last_change_at: Duration::ZERO,
            next_interval,
        }
    }

    fn speed(&mut self) -> f64 {
        let increment = self
            .rng
            .gen_range(-self.parameters.jitter..self.parameters.jitter);
        let Range { min, max } = self.parameters.speed_range;
        (self.last_speed + increment).clamp(min, max)
    }

    fn next_interval(&mut self) -> Duration {
        pick_duration(self.parameters.interval_range, &mut self.rng)
    }
}

impl SpeedController for RandomController {
    fn next_speed(&mut self, elapsed: Duration) -> f64 {
        let next_change = self.last_change_at + self.next_interval;
        info!("elapsed: {:?}, next_change: {:?}", elapsed, next_change);
        if elapsed >= next_change {
            let next_speed = self.speed();
            let next_interval = self.next_interval();
            info!(
                "changing speed from {} to {} at {:?}, next change in {:?}",
                self.last_speed, next_speed, elapsed, next_interval
            );
            self.last_speed = next_speed;
            self.last_change_at = elapsed;
            self.next_interval = next_interval;
            next_speed
        } else {
            self.last_speed
        }
    }

    fn slide_range(&self) -> Range {
        self.parameters.slide_range
    }

    fn initial_speed(&self) -> f64 {
        self.parameters.speed_range.min
    }

    fn should_continue(&self, _elapsed: Duration) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    use super::RandomParameters;
    use crate::service::handy::patterns::Range;

    #[test]
    #[traced_test]
    fn test_random_controller() {
        use std::time::Duration;

        use super::{RandomController, SpeedController};

        let parameters = RandomParameters {
            speed_range: Range {
                min: 30.0,
                max: 79.0,
            },
            slide_range: Range {
                min: 0.0,
                max: 80.0,
            },
            seed: None,
            jitter: 5.0,
            interval_range: Range {
                min: 4.0,
                max: 15.0,
            },
        };

        let mut controller = RandomController::new(parameters);

        let mut samples = vec![];
        for i in 0..10 {
            let speed = controller.next_speed(Duration::from_secs(i));
            samples.push(speed);
        }

        info!("samples: {:?}", samples);

        let min = samples
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max = samples
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        assert!(*min >= 30.0);
        assert!(*max <= 79.0);
    }
}
