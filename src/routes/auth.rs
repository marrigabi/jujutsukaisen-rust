use rocket::form::Form;
use rocket::response::status;
use rocket::State;
use rocket::serde::json::Json;
use crate::config::database::DbPool;
use crate::models::usuario::Usuario;
use crate::utils::{hashing, jwt};

#[derive(FromForm)]
pub struct LoginInput {
    pub login: String,
    pub senha: String,
}

#[post("/login", data = "<login_input>")]
pub fn login(pool: &State<DbPool>, login_input: Form<LoginInput>) -> Option<Json<String>> {
    let user = login_input.into_inner();
    let mut conn = pool.get_conn();

    let result: Option<(String, String)> = conn.exec_first(
        "SELECT senha, role FROM usuarios WHERE login = ?",
        (&user.login,),
    ).ok()?;

    if let Some((senha_hash, role)) = result {
        if hashing::verify_password(&user.senha, &senha_hash) {
            let token = jwt::generate_jwt(&user.login, &role);
            return Some(Json(token));
        }
    }

    None
}