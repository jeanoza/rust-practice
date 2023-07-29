use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};

async fn handle_hello_world() -> impl Responder {
    "Hello world!"
}

struct AppState {
    app_name:String,
}

#[get("/")]
async fn index(data:web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("hello {}!",  app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            //state which will be used globally at the routes in same scope
            .app_data(web::Data::new(AppState {
                app_name:String::from("Actix Web2")
            }))
            .service(
            web::scope("/app")
                .route("/index.html", web::get().to(handle_hello_world))
            )
            .service(index)
        })
        .bind(("localhost", 8080))?
        .run()
        .await
}