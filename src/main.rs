use chrono::NaiveDate;
use std::io;

mod iocontroller;
mod createmod;

use self::createmod::{create_movie};
use self::iocontroller::{save_to_file, load_from_file};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Genero{
    Acao,
    Animacao,
    Comedia,
    Drama,
    Gospel,
    Suspense,
    Outros,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filme{
    nome: String,
    bilhetes_vendidos:u32,
    data_lancamento:NaiveDate,
    genero:Genero,
}
fn main() { 
/*
    let filme = Filme{
        nome:String::from("Filme muito engracado!"),
        bilhetes_vendidos: 1200,
        data_lancamento: NaiveDate::from_ymd_opt(2024, 10, 24).expect("Data invalida!!!"),
        genero: Genero::Comedia,
    };
    let filme2 = Filme{
        nome:String::from("Filme muito dramatico...!"),
        bilhetes_vendidos: 400,
        data_lancamento: NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data invalida!!!"),
        genero: Genero::Drama,
    };

    let mut filmes = vec![filme]; // Salvar dados no arquivo 
    filmes.push(filme2);
    
    if let Err(e) = save_to_file(&filmes, "filmes.bin") { 
        eprintln!("Erro ao salvar dados: {}", e); 
    } // Carregar dados do arquivo
    
    match load_from_file("filmes.bin") { 
        Ok(loaded_filmes) => { 
            println!("Dados carregados: {:#?}", loaded_filmes); 
            filmes = loaded_filmes; 
            } 
         Err(e) => eprintln!("Erro ao carregar dados: {}", e),
    } 
    */
    println!("Bem vindo ao Cinema System:");
    loop{
        println!("Escolha a operação que deseja realizar:");
        println!("-1 -> Encerrar execução\n\n1 -> Criar filme\n2 -> Atualizar filme\n3 -> Ler filme\n4 -> Deletar filme");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Erro na leitura dessa linha!");

        match guess.trim(){
            "-1" => {
                println!("Encerrando execução...\nObrigado pela preferência!");
                break;
            },
            "1" => create_movie(),
            "2" => println!("Atualizar filme"),
            "3" => println!("Ler filme"),
            "4" => println!("Deletar filme"),
            _ => println!("Comando não reconhecido"),
        }
    }
}
