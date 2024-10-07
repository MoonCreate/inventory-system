use actix_web::web::{Data, ServiceConfig};

use inventory_system::{domains, states::AppState};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let state = Data::new(AppState::new());
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(state.clone()).configure(domains::config);
    };

    Ok(config.into())
}
