use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usuario {
    pub id: u32,
    pub nome: String,
    pub sobrenome: String,
    pub email: String,
    pub login: String,
    pub senha: String,
    pub role: String, // "estudante", "professor" ou "admin"
}
