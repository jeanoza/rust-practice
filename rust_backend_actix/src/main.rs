use std::sync::Mutex;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};

async fn handle_hello_world() -> impl Responder {
    "Hello world!"
}

#[derive(Debug)]
struct AppState {
    app_name:String,
}
#[derive(Debug)]
struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data:web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("hello {:?}!",app_name)
}

#[get("/mutable")]
async fn mutable_index(data:web::Data<AppStateWithCounter>) -> impl Responder{
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

#[get("/mutable2")]
async fn mutable_index2(data:web::Data<AppStateWithCounter>) -> impl Responder{
    format!("counter check(mutable2): {}", data.counter.lock().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move ||{
        App::new()
            // state which will be used globally at the routes in same scope
            .app_data(web::Data::new(AppState {
                app_name:String::from("Actix Web2")
            }))
            .app_data(counter.clone())
            .service(web::scope("/app")
                .route("/index.html", web::get().to(handle_hello_world)))
            .service(index)
            .service(mutable_index)
            .service(mutable_index2)
        })
        .bind(("localhost", 8080))?
        .run()
        .await
}