use crate::structs::models;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub async fn delete_user(conn: &mut AsyncPgConnection, uuid: i32) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;

    let result = diesel::delete(users.find(uuid)).execute(conn).await?;

    Ok(result)
}

pub async fn get_all_user(conn: &mut AsyncPgConnection) -> Result<Vec<models::User>, DbError> {
    use crate::schema::users::dsl::*;

    let results = users.get_results::<models::User>(conn).await?;

    Ok(results)
}

pub async fn update_user<'a>(
    conn: &mut AsyncPgConnection,
    uuid: i32,
    data: &models::UpdateUser<'a>,
) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;

    let user = diesel::update(users.find(uuid))
        .set(data)
        .execute(conn)
        .await?;

    Ok(user)
}

pub async fn create_user<'a>(
    conn: &mut AsyncPgConnection,
    data: &models::NewUser<'a>,
) -> Result<models::User, DbError> {
    use crate::schema::users;

    let result = diesel::insert_into(users::table)
        .values(data)
        .returning(models::User::as_returning())
        .get_result(conn)
        .await?;

    Ok(result)
}

pub async fn get_user(conn: &mut AsyncPgConnection, uuid: i32) -> Result<models::User, DbError> {
    use crate::schema::users::dsl::*;

    let user = users.find(uuid).get_result(conn).await?;

    Ok(user)
}
