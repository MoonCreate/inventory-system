use actix_web::Result;
use sqlx::{query, query_as, types::Uuid, PgPool};

use crate::{
    errors::UserError,
    structs::models::user::{User, UserNew, UserUpdate},
};

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
    .bind(bcrypt::hash(data.password, bcrypt::DEFAULT_COST).map_err(|_| UserError::InternalError)?)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        log::info!("{}", e);
        match e {
            sqlx::Error::Database(e)
                if e.constraint() == Some("users_email_key") && e.is_unique_violation() =>
            {
                UserError::EmailTaken
            }
            _ => UserError::InternalError,
        }
    })?;

    Ok(result)
}

pub async fn delete_user(pool: &PgPool, id: &Uuid) -> Result<u64> {
    let result = query("DELETE * FROM users where id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            log::info!("{}", e);
            match e {
                sqlx::Error::RowNotFound => UserError::NotFound,
                _ => UserError::InternalError,
            }
        })?;

    Ok(result.rows_affected())
}

pub async fn update_user<'a>(pool: &PgPool, id: &Uuid, data: UserUpdate<'a>) -> Result<User> {
    let result = query_as(
        "UPDATE users
        SET first_name = COALESCE(NULLIF($2, ''), first_name),
            last_name = COALESCE(NULLIF($3, ''), last_name),
            email = COALESCE(NULLIF($4, ''), email),
            password = COALESCE(NULLIF($5, ''), password),
            role = COALESCE(NULLIF($6, ''), role),
        WHERE id = $1
        RETURNING *",
    )
    .bind(id)
    .bind(data.first_name)
    .bind(data.last_name)
    .bind(data.email)
    .bind(if let Some(pw) = data.password {
        Some(bcrypt::hash(pw, bcrypt::DEFAULT_COST).map_err(|_| UserError::InternalError)?)
    } else {
        None
    })
    .bind(data.role)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        log::info!("{}", e);
        match e {
            sqlx::Error::Database(e)
                if e.constraint() == Some("users_email_key") && e.is_unique_violation() =>
            {
                UserError::EmailTaken
            }
            sqlx::Error::RowNotFound => UserError::NotFound,
            _ => UserError::InternalError,
        }
    })?;

    Ok(result)
}

pub async fn retrieve_user_all(pool: &PgPool) -> Result<Vec<User>> {
    let result = query_as("SELECT * FROM users")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::info!("{}", e);
            match e {
                _ => UserError::InternalError,
            }
        })?;

    Ok(result)
}

pub async fn retrive_user(pool: &PgPool, id: &Uuid) -> Result<User> {
    let result = query_as("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::info!("{}", e);
            match e {
                sqlx::Error::RowNotFound => UserError::NotFound,
                _ => UserError::InternalError,
            }
        })?;

    Ok(result)
}
