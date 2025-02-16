use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cliente {
    pub nome: String,
    pub email: String,
    pub telefone: u64,
    pub inventario: bool,
}

impl Cliente {
    /// Cria um novo cliente.
    pub fn novo(nome: String, email: String, telefone: u64) -> Self {
        Cliente {
            nome,
            email,
            telefone,
            inventario: false,
        }
    }

    /// Salva clientes no arquivo `cliente.bin`.
    pub fn salvar(clientes: &Vec<Cliente>) -> std::io::Result<()> {
        let encoded = bincode::serialize(clientes).expect("Erro ao serializar clientes.");
        let mut file = File::create("cliente.bin")?;
        file.write_all(&encoded)?;
        Ok(())
    }

    /// Carrega clientes do arquivo `cliente.bin`.
    pub fn carregar() -> Vec<Cliente> {
        let mut file = match File::open("cliente.bin") {
            Ok(f) => f,
            Err(_) => return Vec::new(),
        };
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        bincode::deserialize(&buffer).unwrap_or_else(|_| Vec::new())
    }
}
