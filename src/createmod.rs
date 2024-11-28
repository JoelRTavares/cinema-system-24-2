use crate::{Filme, Genero};
use std::io;
use crate::iocontroller::{append_filme_to_file, check_filme_nome};
use chrono::NaiveDate;


pub fn create_movie(){
    loop{
        let nome_f = definir_nome();
        let bilhetes = definir_bilhetes();      
        let data = definir_data();
        let gen = definir_genero();

        let filme = Filme{
            nome: nome_f,
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
        if opcao.trim() == "0"{
            break;
        }
        println!("Iniciando outro filme a ser criado...");
    }
}

fn definir_nome() -> String{
    loop{
        println!("Digite o nome do filme a incluir:");
	    let mut nome_f = String::new();
        io::stdin()
            .read_line(&mut nome_f)
            .expect("Error reading this line!");

        nome_f = nome_f.trim().to_string();

        match check_filme_nome(&nome_f, "filmes.bin"){
            true => {
                println!("Esse nome já existe no sistema! Por favor, escolha outro!");
                continue;
            },
            false =>{
                return nome_f;
            }
        }
    }
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
        println!("Digite a data no formato YYYY-MM-DD\n(Entre 2000-01-01 e 2025-12-31):");
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Error reading this line!");

        let data : Vec<_> = data.trim().split("-").collect();
        if data.len() != 3{
            println!("Por favor, insira uma data no formato YYYY-MM-DD!");
            continue;
        }

	    let ano = data[0].to_string();
        let ano: u32 = match ano.trim().parse() {
                Ok(num) if num >= 2000 && num <= 2025 => num,
                _ => {
                    println!("Por favor, insira um ano válido (Entre 2000 e 2025)!");
                    continue;
                },
        };

	    let mes = data[1].to_string();
        let mes: u32 = match mes.trim().parse() {
                Ok(num) if num >= 1 && num <= 12 => num, 
                _ => {
                    println!("Por favor, insira um mês válido (entre 1 e 12)!");
                    continue;
                },
        };

	    let dia = data[2].to_string();
        let dia: u32 = match dia.trim().parse() {
                Ok(num) if num >= 1 && num <= 31 => num,
                _ => {
                    println!("Por favor, insira um dia válido (entre 1 e 31)!");
                    continue;
                },
        };

        let data = NaiveDate::from_ymd_opt(ano as i32, mes, dia);
        match data { 
            Some(_) => {},
            None => {
                println!("Por favor, insira uma data válida!\nA data {ano}-{mes}-{dia} não existe!");
                continue;
            }, 
        };
        return data.expect("Data inválida!");
    }
}
fn definir_genero() -> Genero{
    println!("Defina o gênero do filme: ");
    println!("1 - Acao,\n2 - Animacao,\n3 - Comedia,\n4 - Drama,\n5 - Gospel,\n6 - Suspense,\nOutra entrada - Outros.");

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