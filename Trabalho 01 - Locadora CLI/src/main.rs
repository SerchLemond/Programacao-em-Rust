mod cliente;
mod filme;
mod aluguel;

use cliente::Cliente;
use aluguel::Aluguel;
use chrono::NaiveDate;
use std::io::{self, Write};
use crate::filme::{Filme, Genero}; // Certifique-se de que os imports estão corretos
use std::str::FromStr;
use regex::Regex; // Adicionando para usar expressões regulares

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

fn cadastrar_cliente() {
    println!("==== Cadastrar Cliente ====");
    
    let mut nome = String::new();
    let mut email = String::new();
    let mut telefone = String::new();

    // Validação do nome (não pode ser vazio)
    loop {
        println!("Digite o nome do cliente:");
        io::stdin().read_line(&mut nome).expect("Erro ao ler entrada.");
        if nome.trim().is_empty() {
            println!("Nome inválido. O nome não pode ser vazio. Tente novamente.");
            nome.clear(); // Limpa o campo de nome para nova entrada
        } else {
            break;
        }
    }

    // Validação do e-mail (formato básico)
    loop {
        println!("Digite o email do cliente:");
        io::stdin().read_line(&mut email).expect("Erro ao ler entrada.");
        let email_trimmed = email.trim();
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+.[a-zA-Z]{2,}$").unwrap();
        if email_regex.is_match(email_trimmed) {
            break;
        } else {
            println!("E-mail inválido. Certifique-se de que o e-mail esteja no formato correto.");
            email.clear(); // Limpa o campo de e-mail para nova entrada
        }
    }

    // Validação do telefone (deve ter exatamente 11 dígitos)
    let telefone_numero: u64;
    loop {
        println!("Digite o telefone do cliente:");
        io::stdin().read_line(&mut telefone).expect("Erro ao ler entrada.");
        let telefone_trimmed = telefone.trim(); // Remove a quebra de linha ou espaços extras

        let telefone_regex = Regex::new(r"^\d{11}$").unwrap(); // Regex para 11 dígitos
        if telefone_regex.is_match(telefone_trimmed) {
            // Tente converter para u64 após a validação
            match telefone_trimmed.parse::<u64>() {
                Ok(tel) => {
                    // Se a conversão for bem-sucedida, armazena o número e sai do loop
                    telefone_numero = tel;
                    break; // Agora podemos sair do loop com telefone_numero do tipo u64
                }
                Err(_) => {
                    println!("Erro ao parsear telefone. Certifique-se de que é um número válido.");
                    telefone.clear(); // Limpa o campo de telefone para nova entrada
                }
            }
        } else {
            println!("Telefone inválido. Certifique-se de que o telefone tenha 11 dígitos.");
            telefone.clear(); // Limpa o campo de telefone para nova entrada
        }
    }

    // Aqui a variável `telefone_numero` já é do tipo u64
    // Agora, cria-se o cliente com os dados válidos
    let nome = nome.trim().to_string(); // Remove espaços extras ao redor do nome
    let email = email.trim().to_string(); // Remove espaços extras ao redor do email

    let mut clientes = Cliente::carregar();
    clientes.push(Cliente::novo(nome, email, telefone_numero));
    Cliente::salvar(&clientes).expect("Erro ao salvar cliente.");
    println!("Cliente cadastrado com sucesso!");
}

pub fn cadastrar_filme() {
    println!("Cadastro de Filme");

    // Variáveis para armazenar os dados do filme (agora mutáveis)
    let mut nome: String;
    let mut data_lancamento: NaiveDate;
    let mut duracao: u32;
    let mut sinopse: String;
    let genero: Genero;

    // Validar nome do filme
    loop {
        print!("Digite o nome do filme: ");
        io::stdout().flush().unwrap();
        let mut nome_input = String::new();
        io::stdin().read_line(&mut nome_input).expect("Erro ao ler entrada.");
        nome = nome_input.trim().to_string();
        
        if nome.is_empty() {
            println!("O nome do filme não pode ser vazio. Tente novamente.");
        } else {
            break; // Sai do loop se o nome for válido
        }
    }

    // Validar data de lançamento
    loop {
        print!("Digite a data de lançamento (dd/mm/aaaa): ");
        io::stdout().flush().unwrap();
        let mut data_input = String::new();
        io::stdin().read_line(&mut data_input).expect("Erro ao ler entrada.");
        match NaiveDate::parse_from_str(data_input.trim(), "%d/%m/%Y") {
            Ok(data) => {
                data_lancamento = data;
                break; // Sai do loop se a data for válida
            }
            Err(_) => println!("Data inválida! Use o formato dd/mm/aaaa e tente novamente."),
        }
    }

    // Validar duração
    loop {
        print!("Digite a duração do filme (em minutos): ");
        io::stdout().flush().unwrap();
        let mut duracao_input = String::new();
        io::stdin().read_line(&mut duracao_input).expect("Erro ao ler entrada.");
        match duracao_input.trim().parse::<u32>() {
            Ok(d) if d > 30 => {
                duracao = d;
                break; // Sai do loop se a duração for válida
            }
            _ => println!("A duração deve ser maior de 30 minutos. Tente novamente."),
        }
    }

    // Validar sinopse (agora mutável)
    loop {
        print!("Digite a sinopse do filme: ");
        io::stdout().flush().unwrap();
        let mut sinopse_input = String::new();
        io::stdin().read_line(&mut sinopse_input).expect("Erro ao ler entrada.");
        sinopse = sinopse_input.trim().to_string();

        if sinopse.is_empty() {
            println!("A sinopse não pode ser vazia. Tente novamente.");
        } else {
            break; // Sai do loop se a sinopse for válida
        }
    }

    // Validar gênero (já corrigido anteriormente)
    genero = loop {
        let mut genero_input = String::new();
        print!("Digite o gênero do filme (Comedia, Romance, Drama, Acao, SuperHeroi, Terror): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut genero_input).expect("Erro ao ler entrada.");
        let genero_input = genero_input.trim().to_lowercase();

        let genero_option = match genero_input.as_str() {
            "comedia" => Some(Genero::Comedia),
            "romance" => Some(Genero::Romance),
            "drama" => Some(Genero::Drama),
            "acao" => Some(Genero::Acao),
            "superheroi" => Some(Genero::SuperHeroi),
            "terror" => Some(Genero::Terror),
            _ => {
                println!("Gênero inválido. Escolha um gênero válido.");
                None
            }
        };

        if let Some(g) = genero_option {
            break g; // Sai do loop quando um gênero válido for encontrado
        }
    };

    // Criar o filme com os dados informados
    let filme = Filme::novo(nome, data_lancamento, genero, duracao, sinopse);
    println!("Filme cadastrado com sucesso: {:?}", filme);
}

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
