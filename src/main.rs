use actix_web::web::{Data, ServiceConfig};

use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use inventory_system::{domains, structs::State};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
    #[shuttle_shared_db::Postgres] conn: AsyncPgConnection,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let state = Data::new(State { conn });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(state.clone()).configure(domains::config);
    };

    Ok(config.into())
}
