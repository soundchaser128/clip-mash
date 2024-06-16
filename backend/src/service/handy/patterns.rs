use std::time::Duration;

use color_eyre::eyre::bail;
use rand::rngs::StdRng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};
use tokio::sync::mpsc;
use tracing::{debug, error, info};
use utoipa::ToSchema;

use super::client::{HandyClient, IHandyClient, Mode};
use crate::helpers::random::{self, create_seeded_rng};
use crate::Result;

// TODO use messages to change parameters on the fly
#[derive(Debug)]
pub enum Message {
    TogglePause,
    Stop,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, ToSchema)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

#[serde_as]
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CycleIncrementParameters {
    pub start_range: Range,
    pub end_range: Range,
    pub slide_range: Range,

    #[serde_as(as = "DurationSeconds<u64>")]
    pub session_duration: Duration,

    #[serde_as(as = "DurationSeconds<u64>")]
    pub cycle_duration: Duration,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RandomParameters {
    pub speed_range: Range,
    pub slide_range: Range,
    pub jitter: f64,
    pub seed: Option<String>,
    pub interval_range: (Duration, Duration),
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum HandyPattern {
    CycleIncrement {
        parameters: CycleIncrementParameters,
    },
    Random {
        parameters: RandomParameters,
    },
    // Accellerate(AccellerateParameters),
    // Cycle(CycleParameters),
}

pub struct HandyController {
    client: HandyClient,
    paused: bool,
    update_interval: Duration,
}

impl HandyController {
    pub fn new(key: String) -> Self {
        let client = HandyClient::new(key);

        Self {
            client,
            paused: false,
            update_interval: Duration::from_millis(500),
        }
    }

    async fn stop(&mut self) -> Result<()> {
        info!("stopping motion");
        global::clear().await;
        global::clear_status().await;
        self.client.stop().await?;

        Ok(())
    }

    async fn tick(
        &mut self,
        controller: &mut impl SpeedController,
        current_velocity: f64,
        elapsed: Duration,
    ) -> Result<f64> {
        let next_speed = controller.next_speed(elapsed).round();
        if next_speed as u32 != current_velocity as u32 {
            info!("Setting new speed: {next_speed}");
            self.client.set_velocity(next_speed).await?;
            Ok(next_speed)
        } else {
            Ok(current_velocity)
        }
    }

    async fn run_loop(
        &mut self,
        mut controller: impl SpeedController,
        mut receiver: mpsc::Receiver<Message>,
    ) -> Result<()> {
        let slide_range = controller.slide_range();
        self.client
            .set_slide_range(slide_range.min as u32, slide_range.max as u32)
            .await?;
        self.client.set_mode(Mode::HAMP).await?;
        self.client.start(controller.initial_speed()).await?;

        let mut current_velocity = controller.initial_speed();
        let mut elapsed = Duration::ZERO;

        loop {
            let message = receiver.try_recv();
            if let Ok(message) = message {
                info!("Received message: {:?}", message);
                match message {
                    Message::TogglePause => {
                        self.paused = !self.paused;

                        if self.paused {
                            self.client.stop().await?;
                        } else {
                            self.client.start(current_velocity).await?;
                        }
                    }
                    Message::Stop => {
                        self.stop().await?;
                        break Ok(());
                    }
                }
            }
            if !self.paused {
                let should_continue = controller.should_continue(elapsed);
                if !should_continue {
                    break Ok(());
                }

                let next_speed = self
                    .tick(&mut controller, current_velocity, elapsed)
                    .await?;
                current_velocity = next_speed;
            } else {
                debug!("Paused, skipping tick");
            }

            global::set_status(ControllerStatus {
                elapsed,
                current_velocity: current_velocity as u32,
                paused: self.paused,
                // current_speed_bounds: controller.slide_range(),
            })
            .await;
            debug!("sleeping for {:?}", self.update_interval);
            tokio::time::sleep(self.update_interval).await;
            elapsed += self.update_interval;
        }
    }

    pub async fn start(mut self, pattern: HandyPattern) -> Result<()> {
        let is_connected = self.client.is_connected().await?;
        if !is_connected {
            bail!("Handy is not connected");
        }

        let (sender, receiver) = mpsc::channel(1);
        global::store(sender).await;

        let controller: Box<dyn SpeedController> = match pattern {
            HandyPattern::CycleIncrement { parameters } => {
                Box::new(CycleIncrementController::new(parameters))
            }
            HandyPattern::Random { parameters } => Box::new(RandomController::new(parameters)),
        };

        tokio::spawn(async move {
            if let Err(e) = self.run_loop(controller, receiver).await {
                error!("Failed to run controller: {e:?}");
            }
        });

        Ok(())
    }
}

trait SpeedController: Send {
    /// Returns the next speed to set. The speed should be rounded to the nearest integer.
    fn next_speed(&mut self, elapsed: Duration) -> f64;

    /// Returns the slide range to set.
    fn slide_range(&self) -> Range;

    /// Returns the initial speed to set.
    fn initial_speed(&self) -> f64;

    /// Returns true if the controller should continue running.
    fn should_continue(&self, elapsed: Duration) -> bool;
}

impl SpeedController for Box<dyn SpeedController> {
    fn next_speed(&mut self, elapsed: Duration) -> f64 {
        self.as_mut().next_speed(elapsed)
    }

    fn slide_range(&self) -> Range {
        self.as_ref().slide_range()
    }

    fn initial_speed(&self) -> f64 {
        self.as_ref().initial_speed()
    }

    fn should_continue(&self, elapsed: Duration) -> bool {
        self.as_ref().should_continue(elapsed)
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatus {
    pub elapsed: Duration,
    pub current_velocity: u32,
    pub paused: bool,
}

struct CycleIncrementController {
    parameters: CycleIncrementParameters,
}

impl SpeedController for CycleIncrementController {
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

impl CycleIncrementController {
    fn new(parameters: CycleIncrementParameters) -> Self {
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

struct RandomController {
    parameters: RandomParameters,
    last_speed: f64,
    rng: StdRng,
    last_change_at: Duration,
    next_interval: Duration,
}

impl RandomController {
    fn new(parameters: RandomParameters) -> Self {
        let seed = parameters
            .seed
            .clone()
            .unwrap_or_else(|| random::get_random_word());

        let mut rng = create_seeded_rng(Some(&seed));
        let next_interval = rng.gen_range(parameters.interval_range.0..parameters.interval_range.1);

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
        let (min, max) = self.parameters.interval_range;
        self.rng.gen_range(min..max)
    }
}

impl SpeedController for RandomController {
    fn next_speed(&mut self, elapsed: Duration) -> f64 {
        let next_change = self.last_change_at + self.next_interval;
        if elapsed >= next_change {
            let next_speed = self.speed();
            let next_interval = self.next_interval();
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

pub async fn stop() {
    if let Some(sender) = global::get().await {
        if let Err(e) = sender.send(Message::Stop).await {
            error!("Failed to send stop message: {:?}", e);
        }
    } else {
        info!("No controller to stop");
    }
}

pub async fn pause() {
    if let Some(sender) = global::get().await {
        if let Err(e) = sender.send(Message::TogglePause).await {
            error!("Failed to send pause message: {:?}", e);
        }
    } else {
        info!("No controller to pause");
    }
}

pub async fn status() -> Option<ControllerStatus> {
    global::get_status().await
}

mod math {
    pub fn clamp01(value: f64) -> f64 {
        return 0.0f64.max(1.0f64.min(value));
    }

    pub fn lerp(from: f64, to: f64, t: f64) -> f64 {
        return from + (to - from) * clamp01(t);
    }

    pub fn ease_in(t: f64) -> f64 {
        return t.powf(2.5);
    }

    pub fn ease_out(t: f64) -> f64 {
        let t = 1.0 - t;
        return t.powf(2.5);
    }
}

mod global {
    use lazy_static::lazy_static;
    use tokio::sync::{mpsc, Mutex};

    use super::{ControllerStatus, Message};

    lazy_static! {
        static ref SENDER: Mutex<Option<mpsc::Sender<Message>>> = Mutex::new(None);
        static ref STATUS: Mutex<Option<ControllerStatus>> = Mutex::new(None);
    }

    pub async fn store(sender: mpsc::Sender<Message>) {
        let mut global = SENDER.lock().await;
        global.replace(sender);
    }

    pub async fn clear() {
        let mut global = SENDER.lock().await;
        global.take();
    }

    pub async fn get() -> Option<mpsc::Sender<Message>> {
        let global = SENDER.lock().await;
        global.clone()
    }

    pub async fn set_status(status: ControllerStatus) {
        let mut global = STATUS.lock().await;
        global.replace(status);
    }

    pub async fn get_status() -> Option<ControllerStatus> {
        let global = STATUS.lock().await;
        global.clone()
    }

    pub async fn clear_status() {
        let mut global = STATUS.lock().await;
        global.take();
    }
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_random_controller() {
        use std::time::Duration;

        use super::{RandomController, SpeedController};

        let parameters = super::RandomParameters {
            speed_range: super::Range {
                min: 30.0,
                max: 79.0,
            },
            slide_range: super::Range {
                min: 0.0,
                max: 80.0,
            },
            seed: None,
            jitter: 5.0,
            interval_range: (Duration::from_secs(4), Duration::from_secs(15)),
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
