use actix_web::{web, get, Result, App, HttpServer, Responder,HttpRequest, HttpResponse, http::StatusCode, dev::AppConfig};
use std::sync::Mutex;
use actix_files as fs;
use actix_files::NamedFile;
use serde::{Serialize,Deserialize};
use std::path::PathBuf;

mod app_config;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct RequestPoint
{
    point : Mutex<Point>
}

async fn add_one(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[get("/show/{data}")]
async fn show_point(req: HttpRequest,requestpoint: web::Data<RequestPoint>) -> impl Responder {
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

async fn score_page(req: HttpRequest,requestpoint: web::Data<RequestPoint>) -> impl Responder {
    let mutreqpoint = requestpoint.point.lock().unwrap();
    let serialized = serde_json::to_string(&(*mutreqpoint));
    serialized
}

async fn scorepage() -> actix_web::Result<NamedFile> {
  Ok(NamedFile::open("../static/html/scorepage.html")?) // Modify the path as per your file structure
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    let point: web::Data<RequestPoint> = web::Data::new(RequestPoint {
        point : Mutex::new(Point { x: (0), y: (0) }),
    });

    let conf = app_config::load_config().expect("Failed to load configuration");

    println!("Host IP: {}:{}", conf.host_ip, conf.host_port);


    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .app_data(point.clone())
            .route("/", web::to(index))
            .route("/add", web::to(add_one))
            .route("/score_page", web::to(score_page))
            .route("/scorepage", web::to(scorepage))
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