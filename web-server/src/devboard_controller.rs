use std::sync::Mutex;

use crate::GameState;
use actix_web::{web, HttpResponse, Responder};

// Request

#[derive(Debug, serde::Deserialize)]
pub struct DevboardEvents {
    pub number_of_buttons: i32,
    pub ms_since_reset: usize,
    pub button_events: Vec<DevboardEvent>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DevboardEvent {
    pub button_index: usize,
    pub event_type: DevboardEventType,
    pub timestamp: usize,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub enum DevboardEventType {
    Pressed,
    Released,
}

// Response

#[derive(Debug, serde::Serialize)]
pub struct DevboardButtonLeds {
    pub button_leds: Vec<DevboardButtonLed>,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct DevboardButtonLed {
    pub enabled: bool,
}

pub async fn handle_devboard_request(
    devboard_events: web::Json<DevboardEvents>,
    game_state: web::Data<GameState>,
    transmitter: web::Data<Mutex<flume::Sender<String>>>,
) -> impl Responder {
    // println!("Number of buttons: {}", devboard_events.number_of_buttons);
    for devboard_event in &devboard_events.button_events {
        println!(
            "buttonIndex {}, eventType {:?}, timestamp {}",
            devboard_event.button_index, devboard_event.event_type, devboard_event.timestamp
        );
    }

    // one tick of game loop
    let devboard_button_leds = game_state.lock().unwrap().update(devboard_events.0);

    // serialize game_state to json
    let frontend_info = game_state.lock().unwrap().serialize();

    transmitter.lock().unwrap().send(frontend_info).unwrap();

    HttpResponse::Ok().json(devboard_button_leds)
}
