mod cliente;
mod filme;
mod aluguel;

use cliente::Cliente;
use filme::Filme;
use aluguel::Aluguel;
use chrono::NaiveDate;
use std::io;

fn main() {
    loop {
        println!("==== Bem-Vindo à LOCADORA PILTOVER ====");
        println!("Selecione uma das opções a seguir:");
        println!("1. Fazer aluguel");
        println!("2. Cadastrar cliente");
        println!("3. Cadastrar filme");
        println!("4. Visualizar histórico");
        println!("0. Sair");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler entrada.");
        let opcao = opcao.trim();

        match opcao {
            "1" => fazer_aluguel(),
            "2" => cadastrar_cliente(),
            "3" => cadastrar_filme(),
            "4" => visualizar_historico(),
            "0" => {
                println!("==== Encerrando... ====");
                break;
            }
            _ => println!("Opção inválida. Tente novamente."),
        }
    }
}

fn fazer_aluguel() {
    println!("==== Fazer Aluguel ====");
    let clientes = Cliente::carregar();
    if clientes.is_empty() {
        println!("Nenhum cliente cadastrado. Cadastre um cliente primeiro.");
        return;
    }

    println!("Clientes disponíveis:");
    for (i, cliente) in clientes.iter().enumerate() {
        println!("{} - {}", i + 1, cliente.nome);
    }

    println!("Selecione o cliente pelo número:");
    let mut escolha_cliente = String::new();
    io::stdin().read_line(&mut escolha_cliente).expect("Erro ao ler entrada.");
    let escolha_cliente = escolha_cliente.trim().parse::<usize>().unwrap_or(0) - 1;

    if escolha_cliente >= clientes.len() {
        println!("Cliente inválido.");
        return;
    }

    let cliente = &clientes[escolha_cliente];
    println!("Verificando inventário...");
    if cliente.inventario {
        println!("O cliente já possui um filme alugado. Faça a devolução antes de alugar outro.");
        return;
    }

    let filmes = Filme::carregar();
    if filmes.is_empty() {
        println!("Nenhum filme disponível para aluguel.");
        return;
    }

    println!("Filmes disponíveis:");
    for (i, filme) in filmes.iter().enumerate() {
        println!("{} - {}", i + 1, filme.titulo);
    }

    println!("Selecione o filme pelo número:");
    let mut escolha_filme = String::new();
    io::stdin().read_line(&mut escolha_filme).expect("Erro ao ler entrada.");
    let escolha_filme = escolha_filme.trim().parse::<usize>().unwrap_or(0) - 1;

    if escolha_filme >= filmes.len() {
        println!("Filme inválido.");
        return;
    }

    let filme = &filmes[escolha_filme];
    let aluguel = Aluguel::novo(
        rand::random::<u32>(), // Gerar um ID aleatório para o aluguel
        cliente.nome.clone(),
        filme.titulo.clone(),
        NaiveDate::from_ymd_opt(2024, 11, 30).unwrap(), // Data do aluguel (substituir pela data atual)
    );

    let mut alugueis = Aluguel::carregar();
    alugueis.push(aluguel);
    Aluguel::salvar(&alugueis).expect("Erro ao salvar o aluguel.");
    println!("Aluguel realizado com sucesso!");
}

//parte do lucas aqui

//parte do arthur aqui

fn visualizar_historico() {
    println!("==== Histórico de Aluguéis ====");
    let alugueis = Aluguel::carregar();
    if alugueis.is_empty() {
        println!("Nenhum histórico de aluguéis disponível.");
        return;
    }

    for aluguel in alugueis {
        println!("{:?}", aluguel);
    }
}
