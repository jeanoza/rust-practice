use actix_web::guard::GuardContext;
use actix_web::{get, guard, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::sync::Mutex;

async fn handle_hello_world() -> impl Responder {
    "Hello world!"
}

#[derive(Debug)]
struct AppState {
    app_name: String,
}
#[derive(Debug)]
struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("hello {:?}!", app_name)
}

#[get("/mutable")]
async fn mutable_index(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

#[get("/mutable2")]
async fn mutable_index2(data: web::Data<AppStateWithCounter>) -> impl Responder {
    format!("counter check(mutable2): {}", data.counter.lock().unwrap())
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()>{
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0),
//     });
//     HttpServer::new(move ||{
//         App::new()
//             // state which will be used globally at the routes in same scope
//             .app_data(web::Data::new(AppState {
//                 app_name:String::from("Actix Web2")
//             }))
//             .app_data(counter.clone())
//             .service(web::scope("/app")
//                 .route("/index.html", web::get().to(handle_hello_world)))
//             .service(index)
//             .service(mutable_index)
//             .service(mutable_index2)
//         })
//         .bind(("localhost", 8080))?
//         .run()
//         .await
// }

//Use Guard ex
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(
                web::scope("/")
                    .guard(guard::Host("users.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("users") })),
            )
            .service(web::scope("/").guard(guard::Host("localhost")).route(
                "",
                web::to(|| async { HttpResponse::Ok().json(json!({"test": "localhost"})) }),
            ))
            .route("/", web::to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

//USE guard
// #[get("")]
// async fn get_all_users() -> HttpResponse {
//     HttpResponse::Ok().json(json!({"test": "test"}))
// }
//
// pub fn verify_token(ctx: &GuardContext) -> bool {
//     let auth_header = ctx.head().headers().get("authorization");
//
//     if auth_header.is_none() {
//         false
//     }
//     true
// }
//
// pub fn user_scoped_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/user")
//             .guard(guard::fn_guard(verify_token))
//             .service(get_all_users),
//     );
// }
//
// pub async fn handle_unauthorized() -> HttpResponse {
//     HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}))
// }
//
// struct ApiGlobalState;
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let app_data = web::Data::new(ApiGlobalState);
//
//     HttpServer::new(move || {
//         App::new()
//             .app_data(app_data.clone())
//             .configure(user_scoped_config)
//             .default_service(web::route().to(handle_unauthorized))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
