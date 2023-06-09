use std::sync::{mpsc, Mutex};

use game::GameMode;

pub mod app_config;
#[cfg(feature = "audio")]
pub mod audio;
pub mod devboard_controller;
pub mod frontend_controller;
pub mod game;

pub type GameState = Mutex<Box<dyn GameMode>>;

/// Audio sender that will be available when audio file paths defined in audio module are present
pub type AudioSender = Mutex<mpsc::Sender<String>>;
