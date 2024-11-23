use crate::{Filme, Genero};
use std::io;
use crate::iocontroller::{append_filme_to_file};
use chrono::NaiveDate;


pub fn create_movie(){
    loop{
        let nomeF = definir_nome();
        let bilhetes = definir_bilhetes();      
        let data = definir_data();
        let gen = definir_genero();

        let filme = Filme{
            nome: nomeF,
            bilhetes_vendidos: bilhetes,
            data_lancamento: data,
            genero: gen,
        };
        println!("Filme criado: {:#?}", filme);
    
        if let Err(e) = append_filme_to_file(&filme, "filmes.bin") { 
            eprintln!("Erro ao adicionar o filme: {}", e); 
        } 

        println!("Gostaria de adicionar outro filme?\n0 - Não\n1 - Sim");
	    let mut opcao = String::new();
        io::stdin()
            .read_line(&mut opcao)
            .expect("Error reading this line!");

        match opcao.trim(){
            "0" => break,
            "1" => println!("Iniciando outro filme a ser criado..."),
            _ => println!("Iniciando outro filme a ser criado..."),
        };
    }
}

fn definir_nome() -> String{
    println!("Digite o nome do filme a incluir:");
	let mut nomeF = String::new();
    io::stdin()
        .read_line(&mut nomeF)
        .expect("Error reading this line!");

    nomeF.trim().to_string()
}

fn definir_bilhetes() -> u32{
    loop{
        println!("Digite quantos bilhetes já foram vendidos:");
	    let mut bilhetes = String::new();
        io::stdin()
            .read_line(&mut bilhetes)
            .expect("Error reading this line!");

        let bilhetes: u32 = match bilhetes.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Por favor, insira um valor numérico e não negativo!");
                    continue;
                },
        };
        return bilhetes;
    }
}
fn definir_data() ->NaiveDate{
    loop{
        println!("Digite o ano de lançamento do filme:");
	    let mut ano = String::new();
        io::stdin()
            .read_line(&mut ano)
            .expect("Error reading this line!");
        let ano: u32 = match ano.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Por favor, insira um ano válido!");
                    continue;
                },
        };
        println!("Digite o mês de lançamento do filme (entre 1 e 12):");
	    let mut mes = String::new();
        io::stdin()
            .read_line(&mut mes)
            .expect("Error reading this line!");
        let mes: u32 = match mes.trim().parse() {
                Ok(num) if num >= 1 && num <= 12 => num, 
                _ => {
                    println!("Por favor, insira um mês válido (entre 1 e 12)!");
                    continue;
                },
        };
        println!("Digite o dia de lançamento do filme:");
	    let mut dia = String::new();
        io::stdin()
            .read_line(&mut dia)
            .expect("Error reading this line!");
        let dia: u32 = match dia.trim().parse() {
                Ok(num) if num >= 1 && num <= 31 => num,
                _ => {
                    println!("Por favor, insira um dia válido (entre 1 e 31)!");
                    continue;
                },
        };

        let data = NaiveDate::from_ymd_opt(ano as i32, mes, dia);
        match data { 
            Some(dat) => println!("Data de lançamento: {}", dat),
            None => {
                println!("Por favor, insira uma data válida!");
                continue;
            }, 
        };
        return data.expect("Data inválida!");
    }
}
fn definir_genero() -> Genero{
    println!("Defina o gênero do filme: ");
    println!("1 - Acao,\n2 - Animacao,\n3 - Comedia,\n4 - Drama,\n5 - Gospel,\n6 -Suspense,\nOutra entrada - Outros.");

    let mut genero = String::new();
    io::stdin()
        .read_line(&mut genero)
        .expect("Error reading this line!");

    match genero.trim(){
        "1" => Genero::Acao,
        "2" => Genero::Animacao,
        "3" => Genero::Comedia,
        "4" => Genero::Drama,
        "5" => Genero::Gospel,
        "6" => Genero::Suspense,
        _ => Genero::Outros,
    }
}