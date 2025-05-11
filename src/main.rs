use std::env;
use actix_web::*;
use actix_web::http::header;
use actix_cors::Cors;
use std::collections::HashMap;
use std::sync::Mutex;
use reqwest;
use tokio::task;
use std::error::Error;
use std::fmt;
mod services;
use services::analyzer;
use services::utils;

struct AppState {
    cache: Mutex<HashMap<String, analyzer::AnalyzeResponse>>,
}

#[derive(Debug, Clone)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError(format!("IO error: {}", err))
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> Self {
        CustomError(format!("Reqwest error: {}", err))
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> Self {
        CustomError(format!("Serde JSON error: {}", err))
    }
}

impl From<&str> for CustomError {
    fn from(err: &str) -> Self {
        CustomError(format!("Generic error: {}", err))
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/analyze")]
async fn analyze(req: web::Json<analyzer::AnalyzeRequest>, data: web::Data<AppState>) -> impl Responder {
    println!("Received comment: {}", req.comment);

    let mut cached_data = data.cache.lock().unwrap();
    let comment_hash = utils::hash_comment(&req.comment);
    if let Some(response) = cached_data.get(&comment_hash) {
        println!("Cache hit!");
        return HttpResponse::Ok().json(response.clone());
    }

    let comment = req.comment.clone();
    let result = task::spawn(async move {
        analyzer::analyze_comment(&comment).await
    }).await.unwrap();

    match result {
        Ok(response) => {
            // Cache the response using the comment hash
            cached_data.insert(utils::hash_comment(&req.comment).clone(), response.clone());
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            println!("Error calling DeepSeek API: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Health")
}

#[get("/cache/{keyword}")]
async fn cache(keyword: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let cached_data = data.cache.lock().unwrap();
    let keyword_str = keyword.into_inner();
    if let Some(response) = cached_data.get(&keyword_str) {
        println!("Cache hit for keyword: {}", keyword_str);
        HttpResponse::Ok().json(response.clone())
    } else {
        println!("Cache miss for keyword: {}", keyword_str);
        HttpResponse::NotFound().body("Cache miss")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let gemini_token = env::var("AI_TOKEN_GEMINI").expect("AI_TOKEN_GEMINI not found in .env");
    let api_port = env::var("API_PORT").expect("API_PORT not found in .env");
    let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT not found in .env");

    println!("Gemini Token: {}", gemini_token);
    println!("API Port: {}", api_port);
    println!("Environment: {}", environment);

    let app_state = web::Data::new(AppState {
        cache: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
        .allowed_origin("https://www.youtube.com") // You can use "*" for dev/testing
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(hello)
            .service(analyze)
            .service(health)
            .service(web::scope("/cache")
                .app_data(app_state.clone())
                .service(cache))
    })
    .bind(("127.0.0.1", api_port.parse::<u16>().unwrap()))?
    .shutdown_timeout(5)
    .workers(8)
    .run()
    .await
}
