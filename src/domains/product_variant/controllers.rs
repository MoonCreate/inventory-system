use actix_web::{http, HttpResponse};
use actix_web::{patch, post, web, Responder, Result};
use sqlx::types::Uuid;

use crate::structs::models::product_variant::{ProductVariantNew, ProductVariantUpdate};
use crate::structs::{AppState, BaseResponse};

use super::dtos;
use super::services;

#[post("")]
pub async fn create_product_variant(
    state: web::Data<AppState>,
    body: web::Json<dtos::ProductVariantNew>,
) -> Result<impl Responder> {
    let product = ProductVariantNew {
        name: &body.name,
        description: &body.description,
        product_id: body.product_id.to_owned(),
    };
    let data = services::create_product_variant(&state.pool, product).await?;
    let code = http::StatusCode::CREATED;
    Ok(HttpResponse::build(code).json(BaseResponse {
        data,
        code: code.as_u16(),
        message: "Product Variant Created".into(),
    }))
}

#[patch("")]
pub async fn update_product_variant(
    state: web::Data<AppState>,
    path: web::Path<(Uuid,)>,
    body: web::Json<dtos::ProductVariantUpdate>,
) -> Result<impl Responder> {
    let product = ProductVariantUpdate {
        name: body.name.as_deref(),
        description: body.description.as_deref(),
    };
    let data = services::update_product_variant(&state.pool, &path.0, product).await?;
    let code = http::StatusCode::OK;
    Ok(HttpResponse::build(code).json(BaseResponse {
        data,
        code: code.as_u16(),
        message: "Product Variant Updated".into(),
    }))
}
