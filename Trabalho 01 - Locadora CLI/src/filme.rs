use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use std::fs::{self, File};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Filme {
    pub titulo: String,
    pub lancamento: NaiveDate,
    pub genero: String,
    pub duracao: u32,
    pub sinopse: String,
}

impl Filme {
    /// Cria um novo filme.
    pub fn novo(titulo: String, lancamento: NaiveDate, genero: String, duracao: u32, sinopse: String) -> Self {
        Filme {
            titulo,
            lancamento,
            genero,
            duracao,
            sinopse,
        }
    }

    /// Salva filmes no arquivo `filme.bin`.
    pub fn salvar(filmes: &Vec<Filme>) -> std::io::Result<()> {
        let encoded = bincode::serialize(filmes).expect("Erro ao serializar filmes.");
        let mut file = File::create("filme.bin")?;
        file.write_all(&encoded)?;
        Ok(())
    }

    /// Carrega filmes do arquivo `filme.bin`.
    pub fn carregar() -> Vec<Filme> {
        let mut file = match File::open("filme.bin") {
            Ok(f) => f,
            Err(_) => return Vec::new(),
        };
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        bincode::deserialize(&buffer).unwrap_or_else(|_| Vec::new())
    }
}
