use actix_web::{error, Result};
use sqlx::{query, query_as, PgPool};

use crate::structs::models::user::{User, UserNew, UserUpdate};

pub async fn add_user<'a>(pool: &PgPool, data: UserNew<'a>) -> Result<User> {
    let result = query_as(
        "INSERT INTO users
        (first_name, last_name, email, password)
        VALUES ($1, $2, $3, $4)
        RETURNING *",
    )
    .bind(data.first_name)
    .bind(data.last_name)
    .bind(data.email)
    .bind(data.password)
    .fetch_one(pool)
    .await
    .map_err(error::ErrorBadRequest)?;

    Ok(result)
}

pub async fn delete_user(pool: &PgPool, id: &str) -> Result<u64> {
    let result = query("DELETE * FROM users where id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(error::ErrorBadRequest)?;

    Ok(result.rows_affected())
}

pub async fn update_user<'a>(pool: &PgPool, id: &str, data: UserUpdate<'a>) -> Result<User> {
    let result = query_as(
        "UPDATE users
        WHERE id = $1
        SET first_name = COALESCE(NULLIF(?, ''), $2)
        SET last_name = COALESCE(NULLIF(?, ''), $3)
        SET email = COALESCE(NULLIF(?, ''), $4)
        SET password = COALESCE(NULLIF(?, ''), $5)
        ",
    )
    .bind(id)
    .bind(data.first_name)
    .bind(data.last_name)
    .bind(data.email)
    .bind(data.password)
    .fetch_one(pool)
    .await
    .map_err(error::ErrorBadRequest)?;

    Ok(result)
}

pub async fn retrieve_user_all(pool: &PgPool) -> Result<Vec<User>> {
    let result = query_as("SELECT * FROM users")
        .fetch_all(pool)
        .await
        .map_err(error::ErrorBadRequest)?;

    Ok(result)
}

pub async fn retrive_user(pool: &PgPool, id: &str) -> Result<Option<User>> {
    let result = query_as("SELECT * FROM user WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(error::ErrorBadRequest)?;

    Ok(result)
}
