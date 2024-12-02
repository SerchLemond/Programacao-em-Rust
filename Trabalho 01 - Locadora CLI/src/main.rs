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

fn cadastrar_cliente() {
    println!("==== Cadastrar Cliente ====");
    let mut nome = String::new();
    let mut email = String::new();

    // Captura o nome
    println!("Digite o nome do cliente:");
    io::stdin().read_line(&mut nome).expect("Erro ao ler entrada.");

    // Validação do email
    loop {
        println!("Digite o email do cliente:");
        io::stdin().read_line(&mut email).expect("Erro ao ler entrada.");
        let email_trimmed = email.trim(); // Remove espaços em branco e quebras de linha

        if email_trimmed.len() > 9 && email_trimmed.ends_with("@gmail.com") {
            break; // Email válido, sai do loop
        } else {
            println!("Email inválido. Certifique-se de que o email termina com '@gmail.com'. Tente novamente.");
        }
    }

    // Captura e valida o telefone como número
    let telefone: u64;
    loop {
        let mut telefone_input = String::new();
        println!("Digite o telefone do cliente:");
        io::stdin()
            .read_line(&mut telefone_input)
            .expect("Erro ao ler entrada.");
        let telefone_trimmed = telefone_input.trim();

        match telefone_trimmed.parse::<u64>() {
            Ok(num) => {
                // Verifica se o número está dentro dos intervalos permitidos
                if (num >= 219_0000_0000 && num <= 219_9999_9999)
                    || (num >= 229_0000_0000 && num <= 229_9999_9999)
                {
                    telefone = num;
                    break; // Telefone válido, sai do loop
                } else {
                    println!("Telefone inválido. Deve conter os DDDs 21 ou 22, e começar com 9. Tente novamente.");
                }
            }
            Err(_) => {
                println!("Telefone inválido. Deve conter apenas números. Tente novamente.");
            }
        }
    }

    // Adiciona o cliente e salva no arquivo
    let mut clientes = Cliente::carregar();
    clientes.push(Cliente::novo(
        nome.trim().to_string(),
        email.trim().to_string(),
        telefone,
    ));
    Cliente::salvar(&clientes).expect("Erro ao salvar cliente.");
    println!("Cliente cadastrado com sucesso!");
}

fn cadastrar_filme() {
    use chrono::NaiveDate;

    println!("==== Cadastrar Filme ====");
    let mut titulo = String::new();
    let mut genero = String::new();
    let mut duracao = String::new();
    let mut sinopse = String::new();

    println!("Digite o título do filme:");
    io::stdin().read_line(&mut titulo).expect("Erro ao ler entrada.");
    println!("Digite o gênero do filme:");
    io::stdin().read_line(&mut genero).expect("Erro ao ler entrada.");

    // Validação da duração
    let duracao: u32 = loop {
        println!("Digite a duração do filme (em minutos):");
        duracao.clear();
        io::stdin().read_line(&mut duracao).expect("Erro ao ler entrada.");

        match duracao.trim().parse::<u32>() {
            Ok(valor) if (30..=180).contains(&valor) => break valor,
            _ => println!("Duração inválida. Insira um número inteiro entre 30 e 180."),
        }
    };

    println!("Digite a sinopse do filme:");
    io::stdin().read_line(&mut sinopse).expect("Erro ao ler entrada.");

    // Validação do lançamento
    let lancamento = loop {
        println!("Digite a data de lançamento do filme (no formato yyyy-mm-dd):");
        let mut input_lancamento = String::new();
        io::stdin().read_line(&mut input_lancamento).expect("Erro ao ler entrada.");
        let input_lancamento = input_lancamento.trim();

        // Verifica se o formato é válido (4 dígitos, hífen, 2 dígitos, hífen, 2 dígitos)
        if input_lancamento.len() == 10
            && input_lancamento.chars().nth(4) == Some('-')
            && input_lancamento.chars().nth(7) == Some('-')
            && input_lancamento.chars().all(|c| c.is_digit(10) || c == '-')
        {
            match NaiveDate::parse_from_str(input_lancamento, "%Y-%m-%d") {
                Ok(data) => break data,
                Err(_) => println!("Data inválida. Certifique-se de que a data seja real."),
            }
        } else {
            println!("Formato inválido. Certifique-se de usar o formato yyyy-mm-dd.");
        }
    };

    // Adiciona o filme e salva no arquivo
    let mut filmes = Filme::carregar();
    filmes.push(Filme::novo(
        titulo.trim().to_string(),
        lancamento,
        genero.trim().to_string(),
        duracao,
        sinopse.trim().to_string(),
    ));
    Filme::salvar(&filmes).expect("Erro ao salvar filme.");
    println!("Filme cadastrado com sucesso!");
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
