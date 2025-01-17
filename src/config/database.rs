use mysql::*;
use mysql::prelude::*;

pub struct DbPool(pub Pool);

impl DbPool {
    pub fn new(url: &str) -> Self {
        let pool = Pool::new(url).expect("Falha ao conectar ao MySQL");
        DbPool(pool)
    }

    pub fn get_conn(&self) -> PooledConn {
        self.0.get_conn().expect("Falha ao obter conexão")
    }
}
