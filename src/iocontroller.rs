use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter, ErrorKind};

use crate::Filme;

use thiserror::Error;

#[derive(Error, Debug)] 
pub enum BinError { 
    #[error("IO error")] 
    Io(#[from] std::io::Error), 
    #[error("Bincode error")] 
    Bincode(#[from] Box<bincode::ErrorKind>), 
}


pub fn save_to_file(filmes: &Vec<Filme>, path: &str) -> Result<(), BinError> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    bincode::serialize_into(writer, &filmes)?;
    Ok(())
}

pub fn load_from_file(path: &str) -> Result<Vec<Filme>, BinError> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(f) => f,
                Err(e) => panic!("Erro ao criar o arquivo: {e:?}"),
            },
            other_error => {
                panic!("Erro ao abrir o arquivo: {other_error:?}");
            }
        }
    };
    let reader = BufReader::new(file);
    let filmes = match bincode::deserialize_from(reader){
        Ok(fil) => fil,
        Err(error) => match *error { 
            bincode::ErrorKind::Io(ref io_error) if io_error.kind() == ErrorKind::UnexpectedEof => Vec::new(),
            _ => return Err(error.into()), 
        },
    };
    Ok(filmes)
}

pub fn append_filme_to_file(filme: &Filme, path: &str) -> Result<(), BinError> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;
    let writer = BufWriter::new(file);
    bincode::serialize_into(writer, &filme)?;
    Ok(())
}

pub fn check_filme_nome(nome_filme: &str, path: &str) -> bool{//Checar se nome selecionado ja existe no arquivo. true = ja existe; false = nao existe
    let filmes = match load_from_file(path){
        Ok(loaded_filmes) => loaded_filmes, 
        Err(e) => { 
             eprintln!("Erro ao carregar dados: {}", e); 
             return false; 
        }
    };
    for i in filmes{
        if i.nome == nome_filme {
            return true;
        }
    }
    false
}