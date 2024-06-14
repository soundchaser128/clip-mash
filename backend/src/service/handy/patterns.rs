use tokio::sync::mpsc;
use tracing::error;

use super::client::HandyClient;
use crate::Result;

pub enum Message {
    TogglePause,
    Stop,
}

pub struct Range {
    pub min: u32,
    pub max: u32,
}

pub struct CycleParameters {}

pub struct CycleIncrementParameters {
    pub start_range: Range,
    pub end_range: Range,
    pub stroke_range: Range,
}

pub struct RandomParameters {}

pub enum HandyPattern {
    Cycle(CycleParameters),
    CycleIncrement(CycleIncrementParameters),
    Random(RandomParameters),
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

        match pattern {
            HandyPattern::CycleIncrement(parameters) => {
                let mut controller = CycleIncrementController {
                    client: self.client,
                    receiver,
                };

                tokio::spawn(async move {
                    if let Err(e) = controller.run(parameters).await {
                        error!("Failed to run cycle increment controller: {}", e)
                    }
                });
            }
            _ => unimplemented!(),
        }

        Ok(sender)
    }
}

pub struct CycleIncrementController {
    client: HandyClient,
    receiver: mpsc::Receiver<Message>,
}

impl CycleIncrementController {
    pub async fn run(&mut self, parameters: CycleIncrementParameters) -> Result<()> {
        self.client
            .set_stroke_range(parameters.stroke_range.min, parameters.stroke_range.max)
            .await?;

        loop {
            let message = self.receiver.try_recv();
            if let Ok(message) = message {
                // TODO
            }
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
