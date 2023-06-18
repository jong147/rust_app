use crate::db_models::{Conductora};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Conductora>>")]
pub struct BuscarConductoras;

#[derive(Message)]
#[rtype(result = "QueryResult<Conductora>")]
pub struct CrearConductora {
    pub nombre: String,
    pub edad: i32,
    pub telefono: String,
    pub correo: String,
    pub area: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Conductora>")]
pub struct ActualizarConductora {
    pub id: i32,
    pub nombre: String,
    pub edad: i32,
    pub telefono: String,
    pub correo: String,
    pub area: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Conductora>")]
pub struct EliminarConductora {
    pub id: i32,
}