use actix_web::{
    get, post, put, delete,
    web::{Data, Json, Path},
    Responder, HttpResponse,
};
use serde::Deserialize;
use crate::{
    messages::{BuscarConductoras, CrearConductora, ActualizarConductora, EliminarConductora},
    AppState, DbActor
};
use actix::Addr;

#[derive(Deserialize)]
pub struct CrearConductoraBody {
    pub nombre: String,
    pub edad: i32,
    pub telefono: String,
    pub correo: String,
    pub area: String,
}

#[derive(Deserialize)]
pub struct ActualizarConductoraBody {
    pub id: i32,
    pub nombre: String,
    pub edad: i32,
    pub telefono: String,
    pub correo: String,
    pub area: String,
}

#[get("/conductoras")]
pub async fn buscar_conductoras(state: Data<AppState>) -> impl Responder {

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(BuscarConductoras).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No se encontraron conductoras"),
        _ => HttpResponse::InternalServerError().json("Error al buscar conductoras"),
    }
}

#[post("/crearconductora")]
pub async fn crear_conductora(state: Data<AppState>, body: Json<CrearConductoraBody>) -> impl Responder {

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

#[put("/conductoras/{id}")]
pub async fn actualizar_conductora(state: Data<AppState>, body: Json<ActualizarConductoraBody>, id: Path<i32>) -> impl Responder {

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(ActualizarConductora {
        id: id.into_inner(),
        nombre: body.nombre.to_string(),
        edad: body.edad,
        telefono: body.telefono.to_string(),
        correo: body.correo.to_string(),
        area: body.area.to_string(),
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Error al actualizar conductora"),
    }
}

#[delete("/conductoras/{id}")]
pub async fn eliminar_conductora(state: Data<AppState>, id: Path<i32>) -> impl Responder {

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(EliminarConductora {
        id: id.into_inner(),
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Error al eliminar conductora"),
    }
}