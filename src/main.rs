use actix_web::{
    error::InternalError,
    web::{Data, JsonConfig, ServiceConfig},
    HttpResponse,
};

use inventory_system::{
    domains,
    structs::{AppState, BaseResponse},
};
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
        cfg.app_data(JsonConfig::default().error_handler(|err, _| {
            let error_json = BaseResponse {
                data: "Ewwow".to_owned(),
                code: 404,
                message: "Ewwow".into(),
            };
            println!("{}", err);
            InternalError::from_response(err, HttpResponse::Conflict().json(error_json)).into()
        }))
        .app_data(state)
        .configure(domains::config);
    };

    Ok(config.into())
}
