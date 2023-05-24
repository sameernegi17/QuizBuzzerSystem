use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{
    get, http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use web_server::app_config;
use web_server::devboard_controller::handle_devboard_request;
use web_server::frontend_controller::reset_route;
use web_server::frontend_controller::websocket_route;
use web_server::game::ReactionTimeGame;
use web_server::AudioSender;
use web_server::GameState;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct RequestPoint {
    point: Mutex<Point>,
}

async fn add_one(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[get("/show/{data}")]
async fn show_point(req: HttpRequest, requestpoint: web::Data<RequestPoint>) -> impl Responder {
    let data = req.match_info().get("data").unwrap();

    let decoded_string: Point = serde_json::from_str(data).unwrap();
    let mut mutreqpoint = requestpoint.point.lock().unwrap();
    *mutreqpoint = decoded_string;
    println!("{:?}", *mutreqpoint);
    format!("{:?}", mutreqpoint)
}

async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/html/index.html")))
}

async fn score_page(_req: HttpRequest, requestpoint: web::Data<RequestPoint>) -> impl Responder {
    let mutreqpoint = requestpoint.point.lock().unwrap();
    serde_json::to_string(&(*mutreqpoint))
}

async fn reaction_game_page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/html/reaction-game.html")?) // Modify the path as per your file structure
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    let point: web::Data<RequestPoint> = web::Data::new(RequestPoint {
        point: Mutex::new(Point { x: (0), y: (0) }),
    });

    #[cfg(feature = "audio")]
    let audio_channel: web::Data<Option<AudioSender>> = {
        use web_server::audio::spawn_audio_thread;
        web::Data::new(spawn_audio_thread().map(Mutex::new))
    };

    #[cfg(not(feature = "audio"))]
    let audio_channel: web::Data<Option<AudioSender>> = web::Data::new(None);

    let game_state: web::Data<GameState> =
        web::Data::new(Mutex::new(Box::new(ReactionTimeGame::new(
            audio_channel
                .get_ref()
                .as_ref()
                .map(|m| m.lock().unwrap().clone()),
        ))));

    let conf = app_config::load_config().expect("Failed to load configuration");

    println!("Host IP: {}:{}", conf.host_ip, conf.host_port);

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .app_data(point.clone())
            .app_data(game_state.clone())
            .app_data(audio_channel.clone())
            .route("/", web::to(index))
            .route("/devboard", web::post().to(handle_devboard_request))
            .route("/websocket", web::get().to(websocket_route))
            .route("/reset", web::to(reset_route))
            .route("/play/reaction-game", web::to(reaction_game_page))
            .service(show_point)
            .service(
                fs::Files::new("/static", "static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
        //.service(fs::Files::new("/static", "./static").show_files_listing())
    })
    //.bind(("192.168.100.1", 8000))?
    .bind((&conf.host_ip as &str, conf.host_port as u16))?
    .run()
    .await
}
