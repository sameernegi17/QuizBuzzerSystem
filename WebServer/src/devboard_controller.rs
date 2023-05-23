use actix_web::{HttpResponse, Responder, web};
use chrono::{Utc, DateTime};

// Request

#[derive(Debug, serde::Deserialize)]
pub struct DevboardEvents {
  number_of_buttons: i32,
  button_events: Vec<DevboardEvent>,
}

#[derive(Debug, serde::Deserialize)]
struct DevboardEvent {
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
struct DevboardButtonLeds {
  button_leds: Vec<DevboardButtonLed>,
}

#[derive(Debug, serde::Serialize)]
struct DevboardButtonLed {
  enabled: bool,
}


pub async fn handle_devboard_request(devboard_events: web::Json<DevboardEvents>) -> impl Responder {
  
  println!("Number of buttons: {}", devboard_events.number_of_buttons);
  for devboard_event in &devboard_events.button_events {
    println!("buttonIndex {}, eventType {:?}, timestamp {}", devboard_event.button_index, devboard_event.event_type, devboard_event.timestamp);
  }

  let devboard_button_leds = DevboardButtonLeds {
    button_leds: vec![
      DevboardButtonLed { enabled: true, },
      DevboardButtonLed { enabled: true, },
      DevboardButtonLed { enabled: false, },
    ]
  };

  HttpResponse::Ok().json(devboard_button_leds)
}

