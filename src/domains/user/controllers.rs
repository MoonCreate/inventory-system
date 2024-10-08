use actix_web::{delete, error, get, http, patch, post, web, Responder, Result};

use crate::structs::State;
use crate::structs::{models, BaseResponse, DbPool};

use super::dtos;
use super::services;

#[get("/all")]
pub async fn user_get(state: web::Data<State>) -> Result<impl Responder> {
    let mut conn = state.get().await.map_err(error::ErrorInternalServerError)?;
    let data = services::get_all_user(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let code = http::StatusCode::OK;
    Ok((
        web::Json(BaseResponse {
            data,
            code: code.as_u16(),
            message: "Success retrieving user".into(),
        }),
        code,
    ))
}

#[get("/{id}")]
pub async fn user_get_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder> {
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    let data = services::get_user(&mut conn, path.into_inner().0)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let code = http::StatusCode::OK;
    Ok((
        web::Json(BaseResponse {
            data,
            code: code.as_u16(),
            message: "Success getting user".into(),
        }),
        code,
    ))
}

#[post("")]
pub async fn user_create(
    pool: web::Data<DbPool>,
    body: web::Json<dtos::UserCreateReq>,
) -> Result<impl Responder> {
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    let user = models::NewUser {
        name: &body.name,
        email: &body.email,
        password: &body.password,
    };

    let data = services::create_user(&mut conn, &user)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let code = http::StatusCode::CREATED;
    Ok((
        web::Json(BaseResponse {
            data,
            code: code.as_u16(),
            message: "Success creating user".into(),
        }),
        code,
    ))
}

#[delete("/{id}")]
pub async fn user_delete(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder> {
    let id = path.into_inner().0;
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    let data = services::delete_user(&mut conn, id)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let code = http::StatusCode::OK;
    Ok((
        web::Json(BaseResponse {
            data,
            code: code.as_u16(),
            message: format!("Success deleting user with id \"{}\"", id),
        }),
        code,
    ))
}

#[patch("/{id}")]
pub async fn update_user(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
    body: web::Json<dtos::UserUpdateReq>,
) -> Result<impl Responder> {
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    let data = models::UpdateUser {
        name: body.name.as_deref(),
        password: body.password.as_deref(),
        email: body.email.as_deref(),
    };
    let data = services::update_user(&mut conn, path.into_inner().0, &data)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let code = http::StatusCode::CREATED;
    Ok((
        web::Json(BaseResponse {
            data,
            code: code.as_u16(),
            message: "Success Creating user".into(),
        }),
        code,
    ))
}
