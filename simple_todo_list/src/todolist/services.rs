use super::models::{CreateEntryData, UpdateEntryData};
use crate::{AppState, TodolistEntry};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}

#[post("/todolist/entries")]
async fn create_entry(
    data: web::Data<AppState>,
    body: web::Json<CreateEntryData>,
) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let mut max_id = 0;
    for i in 0..todolist_entries.len() {
        let current_id = todolist_entries[i].id;
        if current_id > max_id {
            max_id = current_id;
        }
    }
    todolist_entries.push(TodolistEntry {
        id: max_id + 1,
        title: body.title.clone(),
        date: body.date,
    });
    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[put("/todolist/entries/{id}")]
async fn update_entry(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<UpdateEntryData>,
) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let id = path.into_inner();
    // println!("into {:?}", path.into());
    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id == id {
            todolist_entries[i].title = body.title.clone();
            break;
        }
    }
    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[delete("/todolist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let id = path.into_inner();

    *todolist_entries = todolist_entries
        .to_vec()
        .into_iter()
        .filter(|x| x.id != id)
        .collect();
    HttpResponse::Ok().json(todolist_entries.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);
}
