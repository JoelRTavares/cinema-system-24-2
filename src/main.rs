use chrono::NaiveDate;
use std::io;

mod iocontroller;
mod createmod;
mod readmod;
mod deletemod;
mod updatemod;

use self::createmod::{create_movie};
use self::readmod::{read_movies};
use self::deletemod::{delete_movie};
use self::updatemod::{update_movie};

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Genero{
    Acao,
    Animacao,
    Comedia,
    Drama,
    Gospel,
    Suspense,
    Outros,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Filme{
    nome: String,
    bilhetes_vendidos:u32,
    data_lancamento:NaiveDate,
    genero:Genero,
}
fn main() { 
    println!("Bem vindo ao Cinema System:");
    run();
}

fn run(){
    loop{
        println!("Escolha a operação que deseja realizar:");
        println!("-1 -> Encerrar execução\n\n1 -> Criar filme\n2 -> Atualizar filme\n3 -> Ler filme\n4 -> Deletar filme");
        let mut opt = String::new();

        io::stdin()
            .read_line(&mut opt)
            .expect("Erro na leitura dessa linha!");

        match opt.trim(){
            "-1" => {
                println!("Encerrando execução...\nObrigado pela preferência!");
                break;
            },
            "1" => create_movie(),
            "2" => update_movie(),
            "3" => read_movies(),
            "4" => delete_movie(),
            _ => println!("Comando não reconhecido"),
        }
    }
}