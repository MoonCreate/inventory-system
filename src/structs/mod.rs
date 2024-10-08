mod base_response;
mod user;

pub mod models;

use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};

pub use user::User;

pub use base_response::BaseResponse;

pub type DbPool = Pool<AsyncPgConnection>;

pub struct State {
    pub conn: AsyncPgConnection,
}
