use actix_web::{delete, error, get, http, patch, post, web, Responder, Result};

use crate::structs::AppState;
use crate::structs::{models, BaseResponse};

use super::dtos;
use super::services;

#[get("/all")]
pub async fn user_get_controller(state: web::Data<AppState>) -> Result<impl Responder> {
    let data = services::retrieve_user_all(&state.pool)
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
pub async fn user_get_by_id_controller(
    state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<impl Responder> {
    let data = services::retrive_user(&state.pool, &path.0)
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
pub async fn user_create_controller(
    state: web::Data<AppState>,
    body: web::Json<dtos::UserCreateReq>,
) -> Result<impl Responder> {
    let user = models::user::UserNew {
        first_name: &body.first_name,
        last_name: &body.last_name,
        email: &body.email,
        password: &body.password,
    };

    let data = services::add_user(&state.pool, user)
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
pub async fn user_delete_controller(
    state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<impl Responder> {
    let data = services::delete_user(&state.pool, &path.0)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let code = http::StatusCode::OK;
    Ok((
        web::Json(BaseResponse {
            data,
            code: code.as_u16(),
            message: format!("Success deleting user with id \"{}\"", path.0),
        }),
        code,
    ))
}

#[patch("/{id}")]
pub async fn update_user_controller(
    state: web::Data<AppState>,
    path: web::Path<(String,)>,
    body: web::Json<dtos::UserUpdateReq>,
) -> Result<impl Responder> {
    let data = models::user::UserUpdate {
        first_name: body.first_name.as_deref(),
        last_name: body.last_name.as_deref(),
        password: body.password.as_deref(),
        email: body.email.as_deref(),
    };
    let data = services::update_user(&state.pool, &path.0, data)
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
