#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::State;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use mysql::*;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};

// Estrutura para conexão com o MySQL
struct DbPool(Pool);

// Estrutura para armazenar um usuário
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Usuario {
    id: u32,
    nome: String,
    sobrenome: String,
    cpf: String,
    email: String,
    telefone: String,
    login: String,
    senha: String,
}

// Estrutura para capturar os dados do formulário
#[derive(FromForm)]
struct UserInput {
    nome: String,
    sobrenome: String,
    cpf: String,
    email: String,
    telefone: String,
    login: String,
    senha: String,
}

// Rota para listar usuários do banco de dados
#[get("/listar_usuarios")]
fn listar_usuarios(pool: &State<DbPool>) -> Template {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    let usuarios: Vec<Usuario> = conn.query_map(
        "SELECT id, nome, sobrenome, cpf, email, telefone, login, senha FROM usuarios",
        |(id, nome, sobrenome, cpf, email, telefone, login, senha)| Usuario { id, nome, sobrenome, cpf, email, telefone, login, senha },
    ).expect("Falha ao buscar usuários");

    Template::render("usuarios", context! {
        title: "Lista de Usuários",
        usuarios
    })
}

// Rota para adicionar um novo usuário via formulário
#[post("/add-user", data = "<user_input>")]
fn add_usuario(pool: &State<DbPool>, user_input: Form<UserInput>) -> Template {
    let user = user_input.into_inner();
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    conn.exec_drop(
        "INSERT INTO usuarios (nome, sobrenome, cpf, email, telefone, login, senha) VALUES (?, ?, ?, ?, ?, ?, ?)",
        (&user.nome, &user.sobrenome, &user.cpf, &user.email, &user.telefone, &user.login, &user.senha),
    ).expect("Erro ao inserir usuário");

    Template::render("success", context! {
        title: "Usuário Adicionado",
        message: format!("Usuário {} cadastrado com sucesso!", user.nome)
    })
}

#[get("/")]
fn index() -> Template {
    // Aqui, você pode passar dados opcionais para o template usando o contexto
    Template::render("index", context! {
        title: "Página Inicial",
        message: "Bem-vindo à página inicial!"
    })
}

// Estrutura para capturar os dados do formulário
//#[derive(FromForm)]
/*struct UserInput {
    first_name: String,
    last_name: String,
}*/

// Rota POST para processar o formulário e renderizar a página de saudação
#[post("/submit", data = "<user_input>")]
fn submit(user_input: Form<UserInput>) -> Template {
    let user = user_input.into_inner();
    Template::render("greeting", context! {
        title: "Saudação",
        greeting_message: format!("Olá, {} {}!", user.nome, user.sobrenome)
    })
}

// Nova rota para a lista de músicas favoritas
#[get("/favorite-songs")]
fn favorite_songs() -> Template {


    let songs = vec![
        "Imagine - John Lennon",
        "Bohemian Rhapsody - Queen",
        "Stairway to Heaven - Led Zeppelin",
        "Hotel California - Eagles",
        "Hey Jude - The Beatles",
    ];

    Template::render("favorite_songs", context! {
        title: "Minhas Músicas Favoritas",
        songs
    })
}


// Deletar usuário
#[get("/delete-user/<id>")]
fn delete_usuario(pool: &State<DbPool>, id: u32) -> Redirect {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    conn.exec_drop("DELETE FROM usuarios WHERE id = ?", (id,))
        .expect("Erro ao deletar usuário");

    Redirect::to("/listar_usuarios")
}


// Página de edição de usuário
#[get("/edit-user/<id>")]
fn edit_usuario_page(pool: &State<DbPool>, id: u32) -> Template {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    let usuarios: Vec<Usuario> = conn.exec_map(
        "SELECT id, nome, sobrenome, cpf, email, telefone, login, senha FROM usuarios WHERE id = ? LIMIT 1",
        (id,),
        |(id, nome, sobrenome, cpf, email, telefone, login, senha)| Usuario { id, nome, sobrenome, cpf, email, telefone, login, senha },
    ).expect("Erro ao buscar usuário");

    if let Some(user) = usuarios.into_iter().next() {
        Template::render("edit_user", context! { title: "Editar Usuário", user })
    } else {
        Template::render("error", context! { message: "Usuário não encontrado!" })
    }
}

// Atualizar usuário
#[post("/update-user/<id>", data = "<user_input>")]
fn update_usuario(pool: &State<DbPool>, id: u32, user_input: Form<UserInput>) -> Redirect {
    let user = user_input.into_inner();
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    conn.exec_drop(
        "UPDATE usuarios SET nome = ?, sobrenome = ?, cpf = ?, email = ?, telefone = ?, login = ?, senha = ? WHERE id = ?",
        (&user.nome, &user.sobrenome, &user.cpf, &user.email, &user.telefone, &user.login, &user.senha, id),
    ).expect("Erro ao atualizar usuário");

    Redirect::to("/listar_usuarios")
}



#[launch]
fn rocket() -> _ {

    let url = "mysql://root:@localhost:3306/riseonmusic";
    let pool = Pool::new(url).expect("Falha ao criar conexão com MySQL");

    // Criando a tabela se não existir
    let mut conn = pool.get_conn().expect("Falha ao conectar ao banco");
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS usuarios (
            id INT AUTO_INCREMENT PRIMARY KEY,
            nome VARCHAR(100),
            sobrenome VARCHAR(100),
            cpf VARCHAR(14) UNIQUE,
            email VARCHAR(100) UNIQUE,
            telefone VARCHAR(15),
            login VARCHAR(50) UNIQUE,
            senha VARCHAR(255)
        )"
    ).expect("Erro ao criar tabela");


    rocket::build()
        .manage(DbPool(pool)) // Adiciona a conexão ao estado do Rocket
        .mount("/", routes![index, submit, favorite_songs, listar_usuarios, add_usuario, edit_usuario_page, update_usuario, delete_usuario])
        .attach(Template::fairing()) // Anexa o fairing do Handlebars para processar templates
}
