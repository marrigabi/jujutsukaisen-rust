use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feiticeiro {
    pub id: u32,
    pub nome: String,
    pub grau: String,
    pub tecnica: String,
    pub afiliacao: String,
    pub login: String,
    pub senha: String,
}

impl Feiticeiro {
    pub fn new(id: u32, nome: String, grau: String, tecnica: String, afiliacao: String, login: String, senha: String) -> Self {
        Feiticeiro { id, nome, grau, tecnica, afiliacao, login, senha }
    }
}
