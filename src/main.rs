use actix_web::web::{Data, ServiceConfig};

use inventory_system::{domains, structs::AppState};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] _secrets: shuttle_runtime::SecretStore,
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(state).configure(domains::config);
    };

    Ok(config.into())
}
