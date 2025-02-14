#[macro_use] extern crate rocket;

mod config; // Configuração da conexão com o banco de dados
mod models; // Modelos de dados (structs)
mod routes; // Rotas da aplicação
mod guards; // Guards de autenticação e autorização
mod utils; // Funções auxiliares (JWT, Hashing)

use rocket::form::Form;
use rocket::State;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use mysql::*;
use mysql::prelude::*;
//use serde::{Deserialize, Serialize};

use crate::models::feiticeiro::Feiticeiro; 
//use models::feiticeiros::listar_feiticeiros; 


// Estrutura para conexão com o MySQL
struct DbPool(Pool);

// Estrutura para capturar os dados do formulário
#[derive(FromForm)]
struct FeiticeiroInput {
    nome: String,
    grau: String, // Grau do feiticeiro, ex: "Jujutsu" ou "Especial"
    tecnica: String, // Técnica amaldiçoada
    afiliacao: String, // Jujutsu High, Associação de Feiticeiros, etc.
    login: String,
    senha: String,
}

// Rota para listar feiticeiros do banco de dados
#[get("/listar-feiticeiros")]
fn listar_feiticeiros(pool: &State<DbPool>) -> Template {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    let feiticeiros: Vec<Feiticeiro> = conn.query_map(
        "SELECT id, nome, grau, tecnica, afiliacao, login, senha FROM feiticeiros",
        |(id, nome, grau, tecnica, afiliacao, login, senha)| Feiticeiro::new(id, nome, grau, tecnica, afiliacao, login, senha),
    ).expect("Falha ao buscar feiticeiros");

    Template::render("feiticeiros", context! {
        title: "Lista de Feiticeiros Jujutsu",
        feiticeiros
    })
}

// Rota para adicionar um novo feiticeiro via formulário
#[post("/add-feiticeiro", data = "<feiticeiro_input>")]
fn add_feiticeiro(pool: &State<DbPool>, feiticeiro_input: Form<FeiticeiroInput>) -> Template {
    let feiticeiro = feiticeiro_input.into_inner();
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    conn.exec_drop(
        "INSERT INTO `feiticeiros`(`id`, `nome`, `grau`, `tecnica`, `afiliacao`, `login`, `senha`) VALUES ('[value-1]','[value-2]','[value-3]','[value-4]','[value-5]','[value-6]','[value-7]')",
        (&feiticeiro.nome, &feiticeiro.grau, &feiticeiro.tecnica, &feiticeiro.afiliacao, &feiticeiro.login, &feiticeiro.senha),
    ).expect("Erro ao inserir feiticeiro");

    Template::render("success", context! {
        title: "Feiticeiro Cadastrado",
        message: format!("Feiticeiro {} registrado com sucesso!", feiticeiro.nome)
    })
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Página Inicial",
        message: "Bem-vindo à escola Jujutsu!"
    })
}

// Nova rota para listar feiticeiros famosos
#[get("/feiticeiros-famosos")]
fn feiticeiros_famosos() -> Template {
    let feiticeiros = vec![
        "Gojo Satoru - Técnica: Infinito",
        "Itadori Yuji - Técnica: Sukuna",
        "Fushiguro Megumi - Técnica: Shikigami",
        "Nobara Kugisaki - Técnica: Straw Doll",
    ];

    Template::render("feiticeiros_famosos", context! {
        title: "Feiticeiros Famosos",
        feiticeiros
    })
}

// Rota para expulsar um feiticeiro
#[get("/expulsar-feiticeiro/<id>")]
fn expulsar_feiticeiro(pool: &State<DbPool>, id: u32) -> Redirect {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    conn.exec_drop("DELETE FROM feiticeiros WHERE id = ?", (id,))
        .expect("Erro ao expulsar feiticeiro");

    Redirect::to("/listar-feiticeiros")
}

// Página de edição de feiticeiro
#[get("/editar-feiticeiro/<id>")]
fn editar_feiticeiro(pool: &State<DbPool>, id: u32) -> Template {
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    let feiticeiros: Vec<Feiticeiro> = conn.exec_map(
        "SELECT id, nome, grau, tecnica, afiliacao, login, senha FROM feiticeiros WHERE id =  LIMIT 1",
        (id,),
        |(id, nome, grau, tecnica, afiliacao, login, senha)| Feiticeiro::new(id, nome, grau, tecnica, afiliacao, login, senha),
    ).expect("Erro ao buscar feiticeiro");

    if let Some(feiticeiro) = feiticeiros.into_iter().next() {
        Template::render("editar_feiticeiro", context! { title: "Editar Feiticeiro", feiticeiro })
    } else {
        Template::render("error", context! { message: "Feiticeiro não encontrado!" })
    }
}

// Atualizar feiticeiro
#[post("/atualizar-feiticeiro/<id>", data = "<feiticeiro_input>")]
fn atualizar_feiticeiro(pool: &State<DbPool>, id: u32, feiticeiro_input: Form<FeiticeiroInput>) -> Redirect {
    let feiticeiro = feiticeiro_input.into_inner();
    let mut conn = pool.0.get_conn().expect("Falha ao conectar ao banco");

    conn.exec_drop(
        "UPDATE feiticeiros SET nome = ?, grau = ?, tecnica = ?, afiliacao = ?, login = ?, senha = ? WHERE id = ?",
        (&feiticeiro.nome, &feiticeiro.grau, &feiticeiro.tecnica, &feiticeiro.afiliacao, &feiticeiro.login, &feiticeiro.senha, id),
    ).expect("Erro ao atualizar feiticeiro");

    Redirect::to("/listar-feiticeiros")
}

#[launch]
fn rocket() -> _ {
    let url = "mysql://root:@localhost:3306/jujutsu_kaisen";
    let pool = Pool::new(url).expect("Falha ao criar conexão com MySQL");

    // Criando a tabela se não existir
    let mut conn = pool.get_conn().expect("Falha ao conectar ao banco");
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS feiticeiros (
            id INT AUTO_INCREMENT PRIMARY KEY,
            nome VARCHAR(100),
            grau VARCHAR(50),
            tecnica VARCHAR(255),
            afiliacao VARCHAR(100),
            login VARCHAR(50) UNIQUE,
            senha VARCHAR(255)
        )"
    ).expect("Erro ao criar tabela");

    rocket::build()
        .manage(DbPool(pool)) // Adiciona a conexão ao estado do Rocket
        .mount("/", routes![index, listar_feiticeiros, add_feiticeiro, feiticeiros_famosos, expulsar_feiticeiro, editar_feiticeiro, atualizar_feiticeiro])
        .attach(Template::fairing()) // Anexa o fairing do Handlebars para processar templates
}
