use actix_web::{web, get, Result, App, HttpServer, Responder,HttpRequest, HttpResponse, http::StatusCode};
use std::sync::Mutex;
use actix_files as fs;
use actix_files::NamedFile;
use serde::{Serialize,Deserialize};
use std::path::PathBuf;

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
    .body(include_str!("./webpages/index.html")))
}

async fn score_page(req: HttpRequest,requestpoint: web::Data<RequestPoint>) -> impl Responder {
    let mutreqpoint = requestpoint.point.lock().unwrap();
    let serialized = serde_json::to_string(&(*mutreqpoint));
    serialized
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    let point: web::Data<RequestPoint> = web::Data::new(RequestPoint {
        point : Mutex::new(Point { x: (0), y: (0) }),
    });


    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .app_data(point.clone())
            .route("/", web::to(index))
            .route("/add", web::to(add_one))
            .route("/score_page", web::to(score_page))
            .service(show_point)
            .service(
                fs::Files::new("/", "webpages")
                    .show_files_listing()
                    .use_last_modified(true),
            )

    })
    .bind(("192.168.100.1", 8000))?
    .run()
    .await
}