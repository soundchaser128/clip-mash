use std::time::Duration;

use color_eyre::eyre::bail;
use cycle_increment::{CycleIncrementController, CycleIncrementParameters};
use random::{RandomController, RandomParameters};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, error, info};
use utoipa::ToSchema;

use super::client::{HandyClient, IHandyClient, Mode};
use crate::Result;

pub mod cycle_increment;
mod global;
mod math;
pub mod random;

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

pub trait SpeedController: Send {
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
