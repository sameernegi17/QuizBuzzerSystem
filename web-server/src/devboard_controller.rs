use crate::{game::QuizMode, GameState};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};

// Request

#[derive(Debug, serde::Deserialize)]
pub struct DevboardEvents {
    pub number_of_buttons: i32,
    pub button_events: Vec<DevboardEvent>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DevboardEvent {
    button_index: i32,
    event_type: DevboardEventType,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    timestamp: DateTime<Utc>,
}

#[derive(Debug, serde::Deserialize)]
enum DevboardEventType {
    Pressed,
    Released,
}

// Response

#[derive(Debug, serde::Serialize)]
pub struct DevboardButtonLeds {
    pub button_leds: Vec<DevboardButtonLed>,
}

#[derive(Debug, serde::Serialize)]
pub struct DevboardButtonLed {
    pub enabled: bool,
}

pub(crate) async fn handle_devboard_request(
    devboard_events: web::Json<DevboardEvents>,
    game_state: web::Data<crate::GameState>,
) -> impl Responder {
    println!("Number of buttons: {}", devboard_events.number_of_buttons);
    for devboard_event in &devboard_events.button_events {
        println!(
            "buttonIndex {}, eventType {:?}, timestamp {}",
            devboard_event.button_index, devboard_event.event_type, devboard_event.timestamp
        );
    }

    let devboard_button_leds = game_state.0.lock().unwrap().update(devboard_events.0);

    HttpResponse::Ok().json(devboard_button_leds)
}
