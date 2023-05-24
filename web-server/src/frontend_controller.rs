use std::sync::{mpsc, Mutex};

use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws::{self, WebsocketContext};

use crate::{AudioSender, GameState};

pub async fn start_reaction_time_game_route(
    game_state: web::Data<GameState>,
    audio: web::Data<Option<AudioSender>>,
) -> impl Responder {
    println!("Resetting game state, starting new reaction time game...");
    log::debug!("Resetting game state, starting new reaction time game...");

    // Start a new game. Also allow the game to play some fun audio if we have audio output.
    let game = crate::game::reaction_time_game::ReactionTimeGame::new(
        audio.get_ref().as_ref().map(|m| m.lock().unwrap().clone()),
    );

    *game_state.lock().unwrap() = Box::new(game);

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

pub async fn start_sound_check_route(
    game_state: web::Data<GameState>,
    audio: web::Data<Option<AudioSender>>,
) -> impl Responder {
    print!("Resetting game state, starting new sound check...");

    // Start a new game. Also allow the game to play some fun audio if we have audio output.
    *game_state.lock().unwrap() = Box::new(crate::game::sound_check::SoundCheck::new(
        audio.get_ref().as_ref().map(|m| m.lock().unwrap().clone()),
    ));

    HttpResponse::Ok()
}
