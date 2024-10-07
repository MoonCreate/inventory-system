use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub id: i64,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: chrono::Utc::now().timestamp(),
            name: "".into(),
            email: "".into(),
            password: "".into(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}
