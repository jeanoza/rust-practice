use super::models::{CreateEntryData, UpdateEntryData};
use crate::{AppState, TodolistEntry};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/")]
async fn get_entries(data: web::Data<AppState>) {
	
}
