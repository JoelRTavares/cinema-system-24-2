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

        if let Err(e) = checar_nome(&nome_f) { 
            eprintln!("Erro ao criar o nome do filme:\n\t{}", e); 
            eprintln!("Erro ao criar o nome do filme:\n\t{}", e); 
            continue;
        }
        return nome_f;
    }
}
fn checar_nome(nome: &str) -> Result<&str, String>{
    match check_filme_nome(&nome, "filmes.bin"){
        true => {
            Err(String::from("Esse nome já existe no sistema! Por favor, escolha outro!"))
        },
        false =>{
            return Ok(nome);
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
        
        match checar_bilhetes(&bilhetes.trim()){
            Ok(num) => return num,
            Err(e) => {
                eprintln!("Erro ao definir os bilhetes vendidos:\n\t{}", e);
                continue;
            },
        }

    }
}
fn checar_bilhetes(bil: &str) -> Result<u32, String>{
    let _: u32 = match bil.parse() {
        Ok(num) => return Ok(num),
        Err(_) => return Err(String::from("Por favor, insira um valor numérico e não negativo!")),
    };
}

fn definir_data() ->NaiveDate{
    loop{
        println!("Digite a data no formato YYYY-MM-DD\n(Entre 2000-01-01 e 2025-12-31):");
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Error reading this line!");

        
        match checar_data(data.trim()){
            Ok(dat) => return dat,
            Err(e) => {
                eprintln!("Erro ao definir a data:\n\t{e}");
                continue;
            },
        };
    }
}

fn checar_data(data: &str) -> Result<NaiveDate, String>{
    let data : Vec<_> = data.split("-").collect();
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
        Some(_) => return Ok(data.expect("Data Inválida")),
        None => {
            return Err(format!("Por favor, insira uma data válida! A data {}-{}-{} não existe!", ano, mes, dia));
        }, 
    };
    
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
/*
fn checar_genero(gen: &str) -> Result<Genero, String>{

}
*/
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
        use crate::iocontroller::{load_from_file};

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
    //Como, na definição do genero, quaisquer outras entradas que não são entre 1 e 6 são consideradas para o gênero Outros,
    //não existe necessidade de tratamento e, assim, de testes para possíveis entradas inválidas.
}