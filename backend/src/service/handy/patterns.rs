use std::time::Duration;

use handy_api::models::Mode;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, error, info};
use utoipa::openapi::info;

use super::client::HandyClient;
use crate::Result;

#[derive(Debug)]
pub enum Message {
    TogglePause,
    Stop,
}

#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug)]
pub struct CycleParameters {}

#[derive(Debug)]
pub struct CycleIncrementParameters {
    pub start_range: Range,
    pub end_range: Range,
    pub stroke_range: Range,
    pub update_interval: Duration,
    pub session_duration: Duration,
    pub cycle_duration: Duration,
}

#[derive(Debug)]
pub struct RandomParameters {}

#[derive(Debug)]
pub struct AccellerateParameters {}

#[derive(Debug)]
pub enum HandyPattern {
    Cycle(CycleParameters),
    CycleIncrement(CycleIncrementParameters),
    Random(RandomParameters),
    Accellerate(AccellerateParameters),
}

pub struct HandyController {
    client: HandyClient,
}

impl HandyController {
    pub fn new(key: String) -> Self {
        let client = HandyClient::new(key);

        Self { client }
    }

    pub async fn start(self, pattern: HandyPattern) -> Result<mpsc::Sender<Message>> {
        let (sender, receiver) = mpsc::channel(1);

        global::store(sender.clone());

        match pattern {
            HandyPattern::CycleIncrement(parameters) => {
                let mut controller =
                    CycleIncrementController::new(self.client, receiver, parameters);

                tokio::spawn(async move {
                    if let Err(e) = controller.run().await {
                        error!("Failed to run cycle increment controller: {}", e)
                    }
                });
            }
            _ => unimplemented!(),
        }

        Ok(sender)
    }
}

struct CycleIncrementController {
    client: HandyClient,
    receiver: mpsc::Receiver<Message>,
    parameters: CycleIncrementParameters,

    // current time in milliseconds
    current_time: u64,
    current_velocity: u32,
    paused: bool,
}

impl CycleIncrementController {
    fn new(
        client: HandyClient,
        receiver: mpsc::Receiver<Message>,
        parameters: CycleIncrementParameters,
    ) -> Self {
        Self {
            current_time: 0,
            current_velocity: parameters.start_range.min.round() as u32,

            client,
            receiver,
            parameters,
            paused: false,
        }
    }

    fn get_cycle_position(&self) -> f64 {
        let cycle_x = (self.current_time % self.parameters.cycle_duration.as_millis() as u64)
            as f64
            / self.parameters.cycle_duration.as_millis() as f64;
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
        let total_position =
            self.current_time as f64 / self.parameters.session_duration.as_secs_f64();
        let min = math::lerp(start.min, end.min, total_position);
        let max = math::lerp(start.max, end.max, total_position);

        Range { min, max }
    }

    async fn tick(&mut self) -> Result<bool> {
        let cycle_value = self.get_cycle_position();
        info!("cycle_value: {}", cycle_value);

        let speed_bounds = self.get_speed_bounds();
        info!("speed_bounds: {:?}", speed_bounds);

        let new_speed = math::lerp(speed_bounds.min, speed_bounds.max, cycle_value).round() as u32;
        if new_speed != self.current_velocity {
            info!("Setting new speed: {}", new_speed);
            self.client.set_velocity(new_speed as f64);
            self.current_velocity = new_speed;
        }

        let session_duration = self.parameters.session_duration.as_millis() as u64;
        if self.current_time >= session_duration {
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
        self.client.stop().await?;

        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        info!(
            "Starting cycle increment controller with parameters: {:?}",
            self.parameters
        );

        self.client
            .set_stroke_range(
                self.parameters.stroke_range.min,
                self.parameters.stroke_range.max,
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
                info!("Paused, skipping tick");
            }

            info!("sleeping for {:?}", self.parameters.update_interval);
            tokio::time::sleep(self.parameters.update_interval).await;
            self.current_time += self.parameters.update_interval.as_millis() as u64;
        }
    }
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

    use super::Message;

    lazy_static! {
        static ref SENDER: Mutex<Option<mpsc::Sender<Message>>> = Mutex::new(None);
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
        let mut global = SENDER.lock().await;
        global.clone()
    }
}
