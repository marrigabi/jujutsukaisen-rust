use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usuario {
    id: u32,
    nome: String,
    sobrenome: String,
    cpf: String,
    email: String,
    telefone: String,
    login: String,
    senha: String,
    pub role: String, // "estudante", "professor" ou "admin"
}

impl Usuario {
    pub fn new(id: u32, nome: String, sobrenome: String, cpf: String, email: String, telefone: String, login: String, senha: String, role: String) -> Self {
        Usuario { id, nome, sobrenome, cpf, email, telefone, login, senha, role }
    }
}