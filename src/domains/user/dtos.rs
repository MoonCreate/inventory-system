use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserCreateReq {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserUpdateReq {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
