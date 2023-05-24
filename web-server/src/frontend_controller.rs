use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws::{self, WebsocketContext};

use crate::{AudioSender, GameState};

pub async fn websocket_route(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(WebSocketActor { peer: None }, &req, stream)
}

#[allow(dead_code)]
struct WebSocketActor {
    peer: Option<WebsocketContext<WebSocketActor>>,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl WebSocketActor {
    #[allow(dead_code)]
    fn update_button_events(&self) {
        // TODO: button events as input, convert it to JSON, send it to self.peer
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            if text == "reset" {
                // After this has been received, next devboard requests will be answered with all lights off
                println!("Received 'reset' message");
            } else {
                // Ignore
                println!("Received text message: {}", text);
            }
        } else {
            // ignore
        }
    }

    fn started(&mut self, _ctx: &mut Self::Context) {
        // self.peer = Some(ctx);
        println!("Someone connected to the socket!");
    }
}

pub async fn start_reaction_time_game_route(
    game_state: web::Data<GameState>,
    audio: web::Data<Option<AudioSender>>,
) -> impl Responder {
    print!("Resetting game state, starting new reaction time game...");

    // Start a new game. Also allow the game to play some fun audio if we have audio output.
    *game_state.lock().unwrap() = Box::new(crate::game::reaction_time_game::ReactionTimeGame::new(
        audio.get_ref().as_ref().map(|m| m.lock().unwrap().clone()),
    ));

    HttpResponse::Ok()
}

pub async fn start_quiz_game_route(
    game_state: web::Data<GameState>,
    audio: web::Data<Option<AudioSender>>,
) -> impl Responder {
    print!("Resetting game state, starting new quiz time game...");

    // Start a new game. Also allow the game to play some fun audio if we have audio output.
    *game_state.lock().unwrap() = Box::new(crate::game::reaction_time_game::ReactionTimeGame::new(
        audio.get_ref().as_ref().map(|m| m.lock().unwrap().clone()),
    ));

    HttpResponse::Ok()
}
