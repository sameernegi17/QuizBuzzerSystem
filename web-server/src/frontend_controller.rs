use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws::{self, WebsocketContext};

pub async fn websocket_route(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(WebSocketActor { peer: None }, &req, stream)
}

struct WebSocketActor {
    peer: Option<WebsocketContext<WebSocketActor>>,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl WebSocketActor {
    fn updateButtonEvents(&self) {
        // TODO: button events as input, convert it to JSON, send it to self.peer
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
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

    fn started(&mut self, ctx: &mut Self::Context) {
        // self.peer = Some(ctx);
        println!("Someone connected to the socket!");
    }
}

pub(crate) async fn reset_route(game_state: web::Data<crate::GameState>) -> impl Responder {
    print!("Resetting game state...");

    *game_state.0.lock().unwrap() = crate::game::ReactionTimeGame {};

    HttpResponse::Ok()
}
