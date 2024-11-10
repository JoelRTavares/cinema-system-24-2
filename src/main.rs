use chrono::NaiveDate;

mod iocontroller;
use self::iocontroller::{save_to_file, load_from_file};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Genero{
    Acao,
    Animacao,
    Comedia,
    Drama,
    Gospel,
    Suspense,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filme{
    nome: String,
    bilhetes_vendidos:u32,
    data_lancamento:NaiveDate,
    genero:Genero,
}
fn main() {

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

    if let Err(e) = save_to_file(&filmes, "filmes.json") { 
        eprintln!("Erro ao salvar dados: {}", e); 
    } // Carregar dados do arquivo
    
    match load_from_file("filmes.json") { 
        Ok(loaded_filmes) => { 
            println!("Dados carregados: {:#?}", loaded_filmes); 
            filmes = loaded_filmes; 
            } 
         Err(e) => eprintln!("Erro ao carregar dados: {}", e), 
    } // Exibir dados
    println!("Filme definido: {:#?}", filmes);
}
