use rocket::form::Form;
use rocket::response::status;
use rocket::State;
use rocket::serde::json::Json;
use crate::config::database::DbPool;
use crate::models::feiticeiro::Feiticeiro;
use crate::utils::{hashing, jwt};

#[derive(FromForm)]
pub struct LoginInput {
    pub nome: String,
    pub senha: String,
}

#[post("/login", data = "<login_input>")]
pub fn login(pool: &State<DbPool>, login_input: Form<LoginInput>) -> Option<Json<String>> {
    let feiticeiro = login_input.into_inner();
    let mut conn = pool.get_conn();

    let result: Option<(String, String)> = conn.exec_first(
        "SELECT senha, grau_jujutsu FROM feiticeiros WHERE nome = ?",
        (&feiticeiro.nome,),
    ).ok()?;

    if let Some((senha_hash, grau_jujutsu)) = result {
        if hashing::verify_password(&feiticeiro.senha, &senha_hash) {
            let token = jwt::generate_jwt(&feiticeiro.nome, &grau_jujutsu);
            return Some(Json(token));
        }
    }

    None
}