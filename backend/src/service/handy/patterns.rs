use std::time::Duration;

use color_eyre::eyre::bail;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds, DurationSecondsWithFrac};
use tokio::sync::mpsc;
use tracing::{debug, error, info};
use utoipa::ToSchema;

use super::client::{HandyClient, IHandyClient, Mode};
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

    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub update_interval: Duration,

    #[serde_as(as = "DurationSeconds<u64>")]
    pub session_duration: Duration,

    #[serde_as(as = "DurationSeconds<u64>")]
    pub cycle_duration: Duration,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum HandyPattern {
    CycleIncrement {
        parameters: CycleIncrementParameters,
    },
    // Cycle(CycleParameters),
    // Random(RandomParameters),
    // Accellerate(AccellerateParameters),
}

pub struct HandyController {
    client: HandyClient,
}

impl HandyController {
    pub fn new(key: String) -> Self {
        let client = HandyClient::new(key);

        Self { client }
    }

    pub async fn start(self, pattern: HandyPattern) -> Result<()> {
        let is_connected = self.client.is_connected().await?;
        if !is_connected {
            bail!("Handy is not connected");
        }

        let (sender, receiver) = mpsc::channel(1);
        global::store(sender).await;

        match pattern {
            HandyPattern::CycleIncrement { parameters } => {
                let mut controller =
                    CycleIncrementController::new(self.client, receiver, parameters);

                tokio::spawn(async move {
                    if let Err(e) = controller.run().await {
                        error!("Failed to run cycle increment controller: {e:?}")
                    }
                });
            }
        }

        Ok(())
    }
}

trait SpeedController {
    fn next_speed(&mut self) -> f64;
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CycleIncrementStatus {
    pub elapsed: Duration,
    pub current_velocity: u32,
    pub paused: bool,
    pub current_speed_bounds: Range,
}

struct CycleIncrementController {
    client: HandyClient,
    receiver: mpsc::Receiver<Message>,
    parameters: CycleIncrementParameters,

    elapsed: Duration,
    current_velocity: u32,
    paused: bool,
}

impl SpeedController for CycleIncrementController {
    fn next_speed(&mut self) -> f64 {
        let cycle_value = self.get_cycle_position();
        debug!("current cycle value: {}", cycle_value);

        let speed_bounds = self.get_speed_bounds();
        debug!("current speed bounds: {:?}", speed_bounds);

        math::lerp(speed_bounds.min, speed_bounds.max, cycle_value)
    }
}

impl CycleIncrementController {
    fn new(
        client: HandyClient,
        receiver: mpsc::Receiver<Message>,
        parameters: CycleIncrementParameters,
    ) -> Self {
        Self {
            elapsed: Duration::ZERO,
            current_velocity: parameters.start_range.min.round() as u32,

            client,
            receiver,
            parameters,
            paused: false,
        }
    }

    fn get_cycle_position(&self) -> f64 {
        let duration = self.parameters.cycle_duration.as_millis();
        let elapsed = self.elapsed.as_millis();
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

    fn get_speed_bounds(&self) -> Range {
        let start = self.parameters.start_range;
        let end = self.parameters.end_range;
        let t = self.elapsed.as_secs_f64();
        let duration = self.parameters.session_duration.as_secs_f64();
        let total_position = t / duration;
        debug!("total_position: {}", total_position);
        let min = math::lerp(start.min, end.min, total_position);
        let max = math::lerp(start.max, end.max, total_position);

        Range { min, max }
    }

    async fn tick(&mut self) -> Result<bool> {
        let cycle_value = self.get_cycle_position();
        debug!("current cycle value: {}", cycle_value);

        let speed_bounds = self.get_speed_bounds();
        debug!("current speed bounds: {:?}", speed_bounds);

        let new_speed = math::lerp(speed_bounds.min, speed_bounds.max, cycle_value).round() as u32;
        if new_speed != self.current_velocity {
            info!("Setting new speed: {new_speed}, current bounds: {speed_bounds:?}",);
            self.client.set_velocity(new_speed as f64).await?;
            self.current_velocity = new_speed;
        }

        if self.elapsed >= self.parameters.session_duration {
            info!("Session duration reached, stopping");
            self.stop().await?;
            Ok(false)
        } else {
            Ok(true)
        }
    }

    async fn stop(&mut self) -> Result<()> {
        info!("stopping motion");
        global::clear().await;
        global::clear_status().await;
        self.client.stop().await?;

        Ok(())
    }

    pub fn status(&self) -> CycleIncrementStatus {
        CycleIncrementStatus {
            elapsed: self.elapsed,
            current_velocity: self.current_velocity,
            paused: self.paused,
            current_speed_bounds: self.get_speed_bounds(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        info!(
            "Starting cycle increment controller with parameters: {:?}",
            self.parameters
        );

        self.client
            .set_slide_range(
                self.parameters.slide_range.min as u32,
                self.parameters.slide_range.max as u32,
            )
            .await?;
        self.client.set_mode(Mode::HAMP).await?;
        self.client.start(self.current_velocity as f64).await?;

        loop {
            let message = self.receiver.try_recv();
            if let Ok(message) = message {
                info!("Received message: {:?}", message);
                match message {
                    Message::TogglePause => {
                        self.paused = !self.paused;

                        if self.paused {
                            self.client.stop().await?;
                        } else {
                            self.client.start(self.current_velocity as f64).await?;
                        }
                    }
                    Message::Stop => {
                        self.stop().await?;
                        break Ok(());
                    }
                }
            }
            if !self.paused {
                let should_continue = self.tick().await?;
                if !should_continue {
                    break Ok(());
                }
            } else {
                debug!("Paused, skipping tick");
            }

            global::set_status(self.status()).await;

            debug!("sleeping for {:?}", self.parameters.update_interval);
            tokio::time::sleep(self.parameters.update_interval).await;
            self.elapsed += self.parameters.update_interval;
        }
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

pub async fn status() -> Option<CycleIncrementStatus> {
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

    use super::{CycleIncrementStatus, Message};

    lazy_static! {
        static ref SENDER: Mutex<Option<mpsc::Sender<Message>>> = Mutex::new(None);
        static ref STATUS: Mutex<Option<CycleIncrementStatus>> = Mutex::new(None);
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

    pub async fn set_status(status: CycleIncrementStatus) {
        let mut global = STATUS.lock().await;
        global.replace(status);
    }

    pub async fn get_status() -> Option<CycleIncrementStatus> {
        let global = STATUS.lock().await;
        global.clone()
    }

    pub async fn clear_status() {
        let mut global = STATUS.lock().await;
        global.take();
    }
}
