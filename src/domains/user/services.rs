use actix_web::web;

use crate::structs::User;

use super::dtos;

pub fn create_user(users: &mut Vec<User>, name: String, email: String, password: String) {
    let mut user = User::new();
    user.set_name(name);
    user.set_email(email);
    user.set_password(password);

    users.push(user);
}

pub fn get_user(users: &Vec<User>, id: i64) -> Option<User> {
    let user = users.iter().find(|x| x.id == id);
    Some(user?.clone())
}

pub fn delete_user(users: &mut Vec<User>, id: i64) -> bool {
    let index = users.iter().position(|x| x.id == id);
    if let Some(i) = index {
        users.remove(i);
        true
    } else {
        false
    }
}

pub fn update_user(users: &mut Vec<User>, id: i64, data: web::Json<dtos::UserUpdateReq>) -> bool {
    let user = users.iter_mut().find(|x| x.id == id);
    if let Some(user) = user {
        if let Some(email) = data.email.to_owned() {
            user.set_email(email)
        }
        if let Some(name) = data.name.to_owned() {
            user.set_name(name)
        }
        if let Some(password) = data.password.to_owned() {
            user.set_password(password)
        }
        true
    } else {
        false
    }
}
