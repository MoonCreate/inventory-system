use actix_web::{delete, get, http, patch, post, web, Responder};

use crate::structs::BaseResponse;
use crate::{states::AppState, structs::User};

use super::dtos;
use super::services;

#[get("/all")]
pub async fn user_get(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let collected: Vec<User> = users.iter().map(|x| x.clone()).collect();
    let code = http::StatusCode::OK;
    (
        web::Json(BaseResponse {
            data: collected,
            code: code.as_u16(),
            message: "Success retrieving user".into(),
        }),
        code,
    )
}

#[get("/{id}")]
pub async fn user_get_by_id(data: web::Data<AppState>, path: web::Path<(i64,)>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let result = services::get_user(&users, path.into_inner().0);
    let (code, message) = if let Some(_) = result {
        (http::StatusCode::OK, "Success Retrieving user")
    } else {
        (http::StatusCode::BAD_REQUEST, "User not found")
    };
    (
        web::Json(BaseResponse {
            data: result,
            code: code.as_u16(),
            message: message.into(),
        }),
        code,
    )
}

#[post("")]
pub async fn user_create(
    data: web::Data<AppState>,
    body: web::Json<dtos::UserCreateReq>,
) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    services::create_user(
        &mut users,
        body.name.to_owned(),
        body.email.to_owned(),
        body.password.to_owned(),
    );
    let code = http::StatusCode::CREATED;
    (
        web::Json(BaseResponse::<Option<String>> {
            data: None,
            code: code.as_u16(),
            message: "Success creating user".into(),
        }),
        code,
    )
}

#[delete("/{id}")]
pub async fn user_delete(data: web::Data<AppState>, path: web::Path<(i64,)>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let success = services::delete_user(&mut users, path.into_inner().0);
    let (code, message) = if success {
        (http::StatusCode::OK, "Success deleting user")
    } else {
        (http::StatusCode::BAD_REQUEST, "User not found")
    };
    (
        web::Json(BaseResponse::<Option<String>> {
            data: None,
            code: code.as_u16(),
            message: message.into(),
        }),
        code,
    )
}

#[patch("/{id}")]
pub async fn update_user(
    data: web::Data<AppState>,
    path: web::Path<(i64,)>,
    body: web::Json<dtos::UserUpdateReq>,
) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let result = services::update_user(&mut users, path.into_inner().0, body);
    let (code, message) = if result {
        (http::StatusCode::OK, "User Updated")
    } else {
        (http::StatusCode::BAD_REQUEST, "User not found")
    };
    (
        web::Json(BaseResponse::<Option<String>> {
            data: None,
            code: code.as_u16(),
            message: message.into(),
        }),
        code,
    )
}
