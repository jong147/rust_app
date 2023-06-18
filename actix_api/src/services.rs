use actix_web::{
    get, post,
    // web::{Data, Json, Path},
    web::{Data, Json},
    Responder, HttpResponse,
};
use serde::Deserialize;
use crate::{
    messages::{BuscarConductoras, CrearConductora},
    AppState, DbActor
};
use actix::Addr;

#[derive(Deserialize)]
pub struct CrearConductoraRequest {
    pub nombre: String,
    pub edad: i32,
    pub telefono: String,
    pub correo: String,
    pub area: String,
}

#[get("/conductoras")]
pub async fn buscar_conductoras(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(BuscarConductoras).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No se encontraron conductoras"),
        _ => HttpResponse::InternalServerError().json("Error al buscar conductoras"),
    }
}

#[post("/crearconductora")]
pub async fn crear_conductora(state: Data<AppState>, body: Json<CrearConductoraRequest>) -> impl Responder {

    // format!("POST /users/{id}/articles")
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CrearConductora {
        nombre: body.nombre.to_string(),
        edad: body.edad,
        telefono: body.telefono.to_string(),
        correo: body.correo.to_string(),
        area: body.area.to_string(),
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Error al agregar conductora"),
    }
}