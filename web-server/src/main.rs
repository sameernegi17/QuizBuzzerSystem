use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{
    dev::AppConfig, get, http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder, Result,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

mod app_config;
mod devboard_controller;
mod frontend_controller;
mod game;

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

struct GameState(Mutex<game::ReactionTimeGame>);

async fn add_one(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[get("/show/{data}")]
async fn show_point(req: HttpRequest, requestpoint: web::Data<RequestPoint>) -> impl Responder {
    let data = req.match_info().get("data").unwrap();

    let decoded_string: Point = serde_json::from_str(&data).unwrap();
    let mut mutreqpoint = requestpoint.point.lock().unwrap();
    *mutreqpoint = decoded_string;
    println!("{:?}", *mutreqpoint);
    format!("{:?}", mutreqpoint)
}

async fn index(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/html/index.html")))
}

async fn score_page(req: HttpRequest, requestpoint: web::Data<RequestPoint>) -> impl Responder {
    let mutreqpoint = requestpoint.point.lock().unwrap();
    let serialized = serde_json::to_string(&(*mutreqpoint));
    serialized
}

async fn game_page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("../static/html/game.html")?) // Modify the path as per your file structure
}

async fn scorepage() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("../static/html/scorepage.html")?) // Modify the path as per your file structure
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

    let game_state: web::Data<GameState> =
        web::Data::new(GameState(Mutex::new(game::ReactionTimeGame {})));

    let conf = app_config::load_config().expect("Failed to load configuration");

    println!("Host IP: {}:{}", conf.host_ip, conf.host_port);

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .app_data(point.clone())
            .app_data(game_state.clone())
            .route("/", web::to(index))
            .route("/add", web::to(add_one))
            .route("/score_page", web::to(score_page))
            .route("/scorepage", web::to(scorepage))
            .route(
                "/devboard",
                web::post().to(devboard_controller::handle_devboard_request),
            )
            .route("/game", web::to(game_page))
            .route(
                "/websocket",
                web::get().to(frontend_controller::websocket_route),
            )
            .route("/reset", web::to(frontend_controller::reset_route))
            .service(show_point)
            .service(
                fs::Files::new("/static", "../static")
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
