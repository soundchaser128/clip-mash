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
