use crate::schema::conductoras;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=conductoras)]
pub struct NuevaConductora {
    pub nombre: String,
    pub edad: i32,
    pub telefono: String,
    pub correo: String,
    pub area: String,
}