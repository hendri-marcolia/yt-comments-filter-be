use actix_web::*;
use actix_web::http::header;
use actix_cors::Cors;
use std::sync::Arc;
use dotenv::dotenv;
use dashmap::DashMap;
use tokio::sync::Semaphore;

mod services;
use services::analyzer;
use services::utils;

#[derive(Clone)]
struct AppState {
    cache: Arc<DashMap<String, analyzer::AnalyzeResponse>>,
    limiter: Arc<Semaphore>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/analyze")]
async fn analyze(
    req: web::Json<analyzer::AnalyzeRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    println!("Received comment: {}", req.comment);

    let comment_hash = utils::hash_comment(&req.comment);

    // Check cache
    if let Some(response) = data.cache.get(&comment_hash) {
        println!("Cache hit!");
        return HttpResponse::Ok().json(response.clone());
    }

    // Acquire permit for concurrency control
    let permit = match data.limiter.clone().acquire_owned().await {
        Ok(p) => p,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Limiter acquisition failed");
        }
    };

    let comment = req.comment.clone();
    let result = analyzer::analyze_comment(&comment).await;

    drop(permit); // Release the permit

    match result {
        Ok(response) => {
            data.cache.insert(comment_hash.clone(), response.clone());
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            println!("Error calling analyzer: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[get("/cache/{key}")]
async fn cache(
    key: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Some(value) = data.cache.get(&key.into_inner()) {
        HttpResponse::Ok().json(value.clone())
    } else {
        HttpResponse::NotFound().body("Cache miss")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let api_port = std::env::var("API_PORT").expect("API_PORT not set");
    let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".into());

    println!("API running on port {} in {} environment", api_port, environment);

    let state = web::Data::new(AppState {
        cache: Arc::new(DashMap::new()),
        limiter: Arc::new(Semaphore::new(10)), // adjust to desired concurrency limit
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://www.youtube.com")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .service(hello)
            .service(analyze)
            .service(health)
            .service(web::scope("/cache").service(cache))
    })
    .workers(4) // Optional tuning
    .bind(("127.0.0.1", api_port.parse().unwrap()))?
    .run()
    .await
}
