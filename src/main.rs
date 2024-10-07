use actix_web::web::ServiceConfig;

use inventory_system::domains;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    Ok(domains::config.into())
}
