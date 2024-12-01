use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use std::fs::{self, File};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Aluguel {
    pub id: u32,
    pub nome: String,
    pub titulo: String,
    pub data: NaiveDate,
    pub devolucao: bool,
}

impl Aluguel {
    /// Cria um novo aluguel.
    pub fn novo(id: u32, nome: String, titulo: String, data: NaiveDate) -> Self {
        Aluguel {
            id,
            nome,
            titulo,
            data,
            devolucao: false,
        }
    }

    /// Salva aluguéis no arquivo `aluguel.bin`.
    pub fn salvar(alugueis: &Vec<Aluguel>) -> std::io::Result<()> {
        let encoded = bincode::serialize(alugueis).expect("Erro ao serializar aluguéis.");
        let mut file = File::create("aluguel.bin")?;
        file.write_all(&encoded)?;
        Ok(())
    }

    /// Carrega aluguéis do arquivo `aluguel.bin`.
    pub fn carregar() -> Vec<Aluguel> {
        let mut file = match File::open("aluguel.bin") {
            Ok(f) => f,
            Err(_) => return Vec::new(),
        };
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        bincode::deserialize(&buffer).unwrap_or_else(|_| Vec::new())
    }

    /// Atualiza o status de devolução de um aluguel.
    pub fn atualizar_devolucao(&mut self) {
        self.devolucao = true;
    }
}
