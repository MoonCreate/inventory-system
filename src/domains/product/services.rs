use actix_web::Result;
use sqlx::{query, query_as, types::Uuid, PgPool};

use crate::{
    errors::UserError,
    structs::models::product::{Product, ProductNew, ProductUpdate},
};

pub async fn retrieve_product_by_id(pool: &PgPool, id: &Uuid) -> Result<Product> {
    let product = query_as("SELECT * FROM products WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("{}", e);
            match e {
                sqlx::Error::RowNotFound => UserError::NotFound,
                _ => UserError::InternalError,
            }
        })?;
    Ok(product)
}

pub async fn retrieve_product_all(pool: &PgPool) -> Result<Vec<Product>> {
    let products = query_as("SELECT * FROM products")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("{}", e);
            match e {
                _ => UserError::InternalError,
            }
        })?;
    Ok(products)
}

pub async fn create_product<'a>(pool: &PgPool, data: ProductNew<'a>) -> Result<Product> {
    let product = query_as(
        "
        INSERT INTO products
        (name, description, price, user_id) = ($1, $2, $3, $4)
        RETURNING *
        ",
    )
    .bind(data.name)
    .bind(data.description)
    .bind(data.price)
    .bind(data.user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        log::error!("{}", e);
        match e {
            _ => UserError::InternalError,
        }
    })?;
    Ok(product)
}

pub async fn update_product_by_id<'a>(
    pool: &PgPool,
    id: &Uuid,
    data: ProductUpdate<'a>,
) -> Result<Product> {
    let product = query_as(
        "UPDATE products 
        SET name = COALESCE(NULLIF($2, ''), name),
            description = COALESCE(NULLIF($3, ''), description),
            price = COALESCE(NULLIF($4, ''), price),
        WHERE id = $1
        RETURNING *",
    )
    .bind(id)
    .bind(data.name)
    .bind(data.description)
    .bind(data.price)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        log::error!("{}", e);
        match e {
            sqlx::Error::RowNotFound => UserError::NotFound,
            _ => UserError::InternalError,
        }
    })?;

    Ok(product)
}

pub async fn delete_product_by_id(pool: &PgPool, id: &Uuid) -> Result<u64> {
    let result = query("DELETE * FROM products WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("{}", e);
            match e {
                _ => UserError::InternalError,
            }
        })?;

    Ok(result.rows_affected())
}
