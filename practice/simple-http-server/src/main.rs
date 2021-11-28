use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {} !", app_name))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn reverse_echo(req_body: String) -> impl Responder {
    let reversed = req_body.chars().rev().collect::<String>();
    HttpResponse::Ok().body(reversed)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState { app_name: String::from("Actix-web") })
            .service(hello)
            .service(echo)
            .route("/echo/reverse", web::post().to(reverse_echo))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

