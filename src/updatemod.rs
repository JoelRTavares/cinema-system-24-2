use crate::iocontroller::{load_from_file, save_to_file, check_filme_nome};
use crate::{Filme, Genero};

use std::io;
use chrono::NaiveDate;

pub fn update_movie(){
	let mut filmes = match load_from_file("filmes.bin"){
        Ok(loaded_filmes) => loaded_filmes, 
        Err(e) => { 
             eprintln!("Erro ao carregar dados: {}", e); 
             return;
        }
    };
    if filmes.len() == 0{
        eprintln!("Não existem filmes para serem deletados!\nPor favor, crie algum filme para usar essa funcionalidade"); 
        return;
    }
    loop{
	    println!("Escolha o filme que gostaria de atualizar (selecione um índice entre 0 e {}): ", filmes.len() - 1);
        for (index, filme) in filmes.iter().enumerate() {
            println!("{}: {:#?}", index, filme);
        }

	    let mut ind = String::new();
        io::stdin()
            .read_line(&mut ind)
            .expect("Error reading this line!");
        let ind: usize = match ind.trim().parse() {
                Ok(num) if num < filmes.len() => num,
                _ => {
                    println!("Por favor, insira um índice válido (entre 0 e {})!", filmes.len() - 1);
                    continue;
                },
        };

        let mut filme = filmes[ind].clone();
        println!("Escolha quais atributos alterar:\n0 - Nome\n1 - Bilhetes Vendidos\n2 - Data de lançamento\n3 - Genero");
        let mut opt = String::new();

        io::stdin()
            .read_line(&mut opt)
            .expect("Erro na leitura dessa linha!");

        match opt.trim(){
            "0" => atualizar_nome(&mut filme),
            "1" => atualizar_bilhetes(&mut filme),
            "2" => atualizar_data(&mut filme),
            "3" => atualizar_genero(&mut filme),
            _ => println!("Campo não reconhecido!"),
        }

        println!("{:#?}", filme);
        filmes[ind] = filme;
        if let Err(e) = save_to_file(&filmes, "filmes.bin") { 
            eprintln!("Erro ao salvar dados: {}", e); 
            break;
        }

        println!("Filme Atualizado com sucesso!\nGostaria de atualizar outro filme?\n0 - Não\n1 - Sim");
        
        let mut opt = String::new();
        io::stdin()
            .read_line(&mut opt)
            .expect("Error reading this line!");
        if opt.trim() == "0" {
            break;
        }
        println!("Iniciando a atualização de outro filme...");
    }
}
fn atualizar_nome(f : &mut Filme){
    loop{
        println!("Escolha o novo nome do Filme: ");
        let mut nome_f = String::new();

        io::stdin()
            .read_line(&mut nome_f)
            .expect("Erro na leitura dessa linha!");
    
        nome_f = nome_f.trim().to_string();

        match check_filme_nome(&nome_f, "filmes.bin"){
            true => {
                println!("Esse nome já existe no sistema! Por favor, escolha outro!");
                continue;
            },
            false =>{
                f.nome = nome_f;
                break;
            }
        }        
    }
}

fn atualizar_bilhetes(f:&mut Filme){
    loop{
        println!("Digite a nova quantidade de bilhetes vendidos:");
	    let mut bilhetes = String::new();
        io::stdin()
            .read_line(&mut bilhetes)
            .expect("Error reading this line!");

        let _bilhetes: u32 = match bilhetes.trim().parse() {
                Ok(num) => {
                    f.bilhetes_vendidos = num;
                    break;
                },
                Err(_) => {
                    println!("Por favor, insira um valor numérico e não negativo!");
                    continue;
                },
        };
    }
}
fn atualizar_data(f: &mut Filme){
    loop{
        println!("Digite a nova data no formato YYYY-MM-DD\n(Entre 2000-01-01 e 2025-12-31):");
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
            Some(_dat) => {
                f.data_lancamento = data.expect("Data inválida!");
                break;
            },
            None => {
                println!("Por favor, insira uma data válida!");
                continue;
            }, 
        };
    }
}
fn atualizar_genero(f: &mut Filme){
    println!("Defina o gênero do filme: ");
    println!("1 - Acao,\n2 - Animacao,\n3 - Comedia,\n4 - Drama,\n5 - Gospel,\n6 - Suspense,\nOutra entrada - Outros.");

    let mut genero = String::new();
    io::stdin()
        .read_line(&mut genero)
        .expect("Error reading this line!");

    match genero.trim(){
        "1" => f.genero = Genero::Acao,
        "2" => f.genero = Genero::Animacao,
        "3" => f.genero = Genero::Comedia,
        "4" => f.genero = Genero::Drama,
        "5" => f.genero = Genero::Gospel,
        "6" => f.genero = Genero::Suspense,
        _ => f.genero = Genero::Outros,
    };
}

/*
#[cfg(test)]
mod tests{
    use super::*;

    //Testes de nome: 
    #[test]
    fn escolher_nome_valido() ->Result<(), String>{
        let nome = "😀😅🤠";
        match checar_nome(&nome) { 
            Ok(_) => Ok(()), 
            Err(e) => Err(e), 
        }
    }
}
*/