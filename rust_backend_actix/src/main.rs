use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};

async fn get_app_index() -> impl Responder {
    "Hello world!"
}
//adfasdf
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().service(
            web::scope("/app")
                .route("/index.html", web::get().to(get_app_index))
            )
        })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}