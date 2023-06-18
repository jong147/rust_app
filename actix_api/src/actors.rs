use crate::db_models::{Conductora};
use crate::db_utils::DbActor;
use crate::schema::conductoras::dsl::*;
use crate::messages::{BuscarConductoras, CrearConductora};
use crate::insertables::NuevaConductora;
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<BuscarConductoras> for DbActor {
  type Result = QueryResult<Vec<Conductora>>;

  fn handle(&mut self, _msg: BuscarConductoras, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Buscar Conductora: No se pudo establecer la conexión");

    conductoras.get_results::<Conductora>(&mut conn)
  }
}

impl Handler<CrearConductora> for DbActor {
  type Result = QueryResult<Conductora>;

  fn handle(&mut self, msg: CrearConductora, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Crear Conductora: No se pudo establecer la conexión");

    let nueva_conductora = NuevaConductora {
        nombre: msg.nombre,
        edad: msg.edad,
        telefono: msg.telefono,
        correo: msg.correo,
        area: msg.area,
    };

    diesel::insert_into(conductoras)
      .values(nueva_conductora)
      .returning((
        id,
        nombre,
        edad.nullable(),
        telefono,
        correo,
        area,
      ))
      .get_result::<Conductora>(&mut conn)
  }
}