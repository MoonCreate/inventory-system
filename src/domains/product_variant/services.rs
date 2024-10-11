use actix_web::Result;
use sqlx::{query_as, types::Uuid, PgPool};

use crate::{
    errors::UserError,
    structs::models::product_variant::{ProductVariant, ProductVariantNew, ProductVariantUpdate},
};

pub async fn create_product_variant<'a>(
    pool: &PgPool,
    data: ProductVariantNew<'a>,
) -> Result<ProductVariant> {
    let data = query_as(
        "INSERT INTO product_variants (name, description, product_id) = ($1, $2, $3) RETURNING *",
    )
    .bind(data.name)
    .bind(data.description)
    .bind(data.product_id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        log::error!("{}", e);
        match e {
            _ => UserError::InternalError,
        }
    })?;
    Ok(data)
}

pub async fn update_product_variant<'a>(
    pool: &PgPool,
    id: &Uuid,
    data: ProductVariantUpdate<'a>,
) -> Result<ProductVariant> {
    let data = query_as(
        "UPDATE product_variants 
        SET name = COALESCE(NULLIF($2, ''), name),
            description = COALESCE(NULLIF($3, ''), description),
        WHERE id = $1
        RETURNING *",
    )
    .bind(id)
    .bind(data.name)
    .bind(data.description)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        log::error!("{}", e);
        match e {
            _ => UserError::InternalError,
        }
    })?;
    Ok(data)
}
