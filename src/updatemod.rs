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
            _ => {
                println!("Campo não reconhecido!");
                continue;
            },
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
        if let Err(e) = checar_nome(&nome_f){
            println!("Erro ao atualizar o nome do filme:\n\t{e}");
            continue;
        };
        f.nome = nome_f;
        break;        
    }
}
fn checar_nome(nome: &str) -> Result<(), String>{
    match check_filme_nome(&nome, "filmes.bin"){
        true => Err(String::from("Esse nome já existe no sistema! Por favor, escolha outro!")),
        false => Ok(()),
    }  
}

fn atualizar_bilhetes(f:&mut Filme){
    loop{
        println!("Digite a nova quantidade de bilhetes vendidos:");
	    let mut bilhetes = String::new();
        io::stdin()
            .read_line(&mut bilhetes)
            .expect("Error reading this line!");
        match checar_bilhetes(&bilhetes.trim()){
            Ok(num) => f.bilhetes_vendidos = num,
            Err(e) => {
                eprintln!("Erro na atualização dos bilhetes:\n\t{}", e);
                continue;
            },
        };
        break;
    }
}
fn checar_bilhetes(bil: &str)-> Result<u32, String>{
    let _: u32 = match bil.parse() {
        Ok(num) => return Ok(num),
        Err(_) => return Err(String::from("Por favor, insira um valor numérico e não negativo!")),
    };
}

fn atualizar_data(f: &mut Filme){
    loop{
        println!("Digite a nova data no formato YYYY-MM-DD\n(Entre 2000-01-01 e 2025-12-31):");
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Error reading this line!");

        
        match checar_data(data.trim()) { 
            Ok(dat) => f.data_lancamento = dat,
            Err(e) => {
                eprintln!("Erro ao atualizar a data de lançamento:\n\t{}", e);
                continue;
            }, 
        }
        break;
    }
}
fn checar_data(data: &str) -> Result<NaiveDate,String>{
    let data : Vec<_> = data.trim().split("-").collect();
    if data.len() != 3{
        return Err(String::from("Por favor, insira uma data no formato YYYY-MM-DD!"));
    }

	let ano = data[0].to_string();
    let ano: u32 = match ano.trim().parse() {
        Ok(num) if num >= 2000 && num <= 2025 => num,
        _ => {
            return Err(String::from("Por favor, insira um ano válido (Entre 2000 e 2025)!"));
        },
    };

	let mes = data[1].to_string();
    let mes: u32 = match mes.trim().parse() {
        Ok(num) if num >= 1 && num <= 12 => num, 
        _ => {
            return Err(String::from("Por favor, insira um mês válido (entre 1 e 12)!"));
        },
    };

	let dia = data[2].to_string();
    let dia: u32 = match dia.trim().parse() {
        Ok(num) if num >= 1 && num <= 31 => num,
        _ => {
            return Err(String::from("Por favor, insira um dia válido (entre 1 e 31)!"));
        },
    };

    let data = NaiveDate::from_ymd_opt(ano as i32, mes, dia);
    match data { 
        Some(dat) => Ok(dat),
        None => Err(format!("Por favor, insira uma data válida! A data {}-{}-{} não existe!", ano, mes, dia)), 
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
    #[test]
    fn escolher_nome_invalido() -> Result<(), String>{
        let arquivo = match load_from_file("filmes.bin"){
            Ok(loaded_filmes) => loaded_filmes, 
            Err(e) => { 
                 return Err(format!("Erro ao carregar dados: {}", e)); 
            }
        };
        if arquivo.len() == 0{
            return Err(String::from("Por favor, insira um filme antes de executar esse teste!"));
        }

        let nome = &arquivo[0].nome;//Escolhendo o nome do primeiro filme disponivel no arquivo

        match checar_nome(&nome) { 
            Ok(_) => Err(String::from("Não deveria ser possível aceitar um nome de filme já existente!")), 
            Err(e) =>
            {
                if e.contains("Esse nome já existe no sistema!"){
                    Ok(())
                }else{
                    Err(e)
                }
            }, 
        }
    }

    //Testes de bilhete:
    #[test]
    fn definir_bilhetes_validos() -> Result<(), String>{
        let bil = "1000";
        match checar_bilhetes(&bil) { 
            Ok(_) => Ok(()), 
            Err(e) => Err(e), 
        }
    }
    #[test]
    fn definir_bilhetes_nao_numericos() -> Result<(), String>{
        let bil = "Foram 300 bilhetes!";
        match checar_bilhetes(&bil) { 
            Ok(_) => Err(String::from("Não deveria ser possível aceitar um argumento não numérico!")), 
            Err(e) =>
            {
                if e.contains(" numérico e não negativo!"){
                    Ok(())
                }else{
                    Err(e)
                }
            },
        }
    }
    #[test]
    fn definir_bilhetes_negativos() -> Result<(), String>{
        let bil = "-300";
        match checar_bilhetes(&bil) { 
            Ok(_) => Err(String::from("Não deveria ser possível aceitar um argumento não numérico!")), 
            Err(e) =>
            {
                if e.contains(" numérico e não negativo!"){
                    Ok(())
                }else{
                    Err(e)
                }
            },
        }
    }

    //Testes de data
    #[test]
    fn definir_data_correta() -> Result<(), String>{
        let data = "2020-01-01";
        match checar_data(&data) { 
            Ok(_) => Ok(()), 
            Err(e) => Err(e), 
        }
    }
    #[test]
    fn definir_data_formato_incorreto() -> Result<(), String>{
        let data = "2020/01/01";
        match checar_data(&data) { 
            Ok(_) => Err(String::from("Não deveria ser possível aceitar essa data com esse formato!")), 
            Err(e) =>
            {
                if e.contains("formato YYYY-MM-DD!"){
                    Ok(())
                }else{
                    Err(e)
                }
            },
        }
    }
    #[test]
    fn definir_data_ano_incorreto() -> Result<(), String>{
        let data = "1952-01-01";
        match checar_data(&data) { 
            Ok(_) => Err(String::from("Não deveria ser possível aceitar um ano fora do intervalo [2000-2025]!")), 
            Err(e) =>
            {
                if e.contains("ano"){
                    Ok(())
                }else{
                    Err(e)
                }
            },
        }
    }
    #[test]
    fn definir_data_mes_incorreto() -> Result<(), String>{
        let data = "2018-16-01";
        match checar_data(&data) { 
            Ok(_) => Err(String::from("Não deveria ser possível aceitar um mês fora do intervalo [1-12]!")), 
            Err(e) =>
            {
                if e.contains("mês"){
                    Ok(())
                }else{
                    Err(e)
                }
            },
        }
    }
    #[test]
    fn definir_data_dia_incorreto() -> Result<(), String>{
        let data = "2022-01-40";
        match checar_data(&data) { 
            Ok(_) => Err(String::from("Não deveria ser possível aceitar um dia fora do intervalo [1-31]!")), 
            Err(e) =>
            {
                if e.contains("dia"){
                    Ok(())
                }else{
                    Err(e)
                }
            },
        }
    }
    #[test]
    fn definir_data_nao_existente() -> Result<(), String>{
        let data = "2022-02-31";
        match checar_data(&data) { 
            Ok(_) => Err(String::from("Essa data não existe! Logo, não deveria ser aceita!")), 
            Err(e) =>
            {
                if e.contains("não existe!"){
                    Ok(())
                }else{
                    Err(e)
                }
            },
        }
    }
}
