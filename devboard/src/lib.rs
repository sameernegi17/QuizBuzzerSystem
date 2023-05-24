#![no_std]

use defmt::Format;
use heapless::{mpmc::Q64, Vec};
use serde::{Deserialize, Serialize};

pub mod button_tasks;
pub mod http;

pub const NUM_BUTTONS: usize = 6;
pub const NUM_BUTTON_PRESSES_PER_MSG: usize = 20;
pub const DEBOUNCE_MS: u64 = 200;
pub const STATE_PERIOD_MS: u64 = 100;
pub const BUFFER_SIZE: usize = 100 + (NUM_BUTTON_PRESSES_PER_MSG * 50);
pub static Q: Q64<(usize, u64)> = Q64::new();

#[derive(Serialize, Debug)]
pub struct State {
    pub time: u64,
    pub button_presses: Vec<(usize, u64), NUM_BUTTON_PRESSES_PER_MSG>,
}

#[derive(Debug, Serialize)]
pub struct DevboardEvents {
    pub number_of_buttons: usize,
    pub ms_since_reset: u64,
    pub button_events: Vec<DevboardEvent, NUM_BUTTON_PRESSES_PER_MSG>,
}

#[derive(Debug, Serialize)]
pub struct DevboardEvent {
    pub button_index: usize,
    pub event_type: DevboardEventType,
    pub timestamp: u64,
}

#[derive(Debug, Serialize)]
pub enum DevboardEventType {
    Pressed,
    Released,
}

#[derive(Debug, Deserialize)]
pub struct DevboardButtonLeds {
    pub button_leds: Vec<DevboardButtonLed, NUM_BUTTONS>,
}

#[derive(Debug, Deserialize, Clone, Format)]
pub struct DevboardButtonLed {
    pub enabled: bool,
}
