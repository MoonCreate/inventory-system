use actix_web::{delete, get, patch, post, web, Responder, Result};
use actix_web::{http, HttpResponse};
use sqlx::types::Uuid;

use crate::structs::models::product::{ProductNew, ProductUpdate};
use crate::structs::{AppState, BaseResponse};

use super::dtos;
use super::services;

#[get("/all")]
async fn retrieve_all_products_controller(state: web::Data<AppState>) -> Result<impl Responder> {
    let data = services::retrieve_product_all(&state.pool).await?;
    let code = http::StatusCode::OK;
    Ok(HttpResponse::build(code).json(BaseResponse {
        data,
        code: code.as_u16(),
        message: "User Retrived".into(),
    }))
}

#[get("/{id}")]
async fn retrieve_product_by_id_controller(
    state: web::Data<AppState>,
    path: web::Path<(Uuid,)>,
) -> Result<impl Responder> {
    let data = services::retrieve_product_by_id(&state.pool, &path.0).await?;
    let code = http::StatusCode::OK;
    Ok(HttpResponse::build(code).json(BaseResponse {
        data,
        code: code.as_u16(),
        message: "User Retrived".into(),
    }))
}

#[delete("/{id}")]
async fn delete_product_by_id_controller(
    state: web::Data<AppState>,
    path: web::Path<(Uuid,)>,
) -> Result<impl Responder> {
    let data = services::delete_product_by_id(&state.pool, &path.0).await?;
    let code = http::StatusCode::OK;
    Ok(HttpResponse::build(code).json(BaseResponse {
        data,
        code: code.as_u16(),
        message: "User Deleted".into(),
    }))
}

#[patch("/{id}")]
async fn update_product_by_id_controller(
    state: web::Data<AppState>,
    path: web::Path<(Uuid,)>,
    body: web::Json<dtos::UpdateProductById>,
) -> Result<impl Responder> {
    let data = services::update_product_by_id(
        &state.pool,
        &path.0,
        ProductUpdate {
            name: body.name.as_deref(),
            description: body.description.as_deref(),
            price: body.price,
        },
    )
    .await?;
    let code = http::StatusCode::OK;
    Ok(HttpResponse::build(code).json(BaseResponse {
        data,
        code: code.as_u16(),
        message: "User Deleted".into(),
    }))
}

#[post("")]
async fn create_product_controller(
    state: web::Data<AppState>,
    body: web::Json<dtos::CreateProduct>,
) -> Result<impl Responder> {
    let data = services::create_product(
        &state.pool,
        ProductNew {
            name: &body.name,
            description: &body.description,
            price: body.price,
            user_id: body.user_id.to_owned(),
        },
    )
    .await?;
    let code = http::StatusCode::OK;
    Ok(HttpResponse::build(code).json(BaseResponse {
        data,
        code: code.as_u16(),
        message: "User Deleted".into(),
    }))
}
