use actix::SyncArbiter;
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};
use std::env;

mod services;
mod db_utils;
mod messages;
mod actors;
mod db_models;
mod schema;
mod insertables;

use db_utils::{get_pool, AppState, DbActor};
use services::{buscar_conductoras, crear_conductora, actualizar_conductora, eliminar_conductora};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
    
    HttpServer::new(move || {

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(Data::new(AppState { db: db_addr.clone() }))
            .wrap(cors)
            .service(buscar_conductoras)
            .service(crear_conductora)
            .service(actualizar_conductora)
            .service(eliminar_conductora)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}