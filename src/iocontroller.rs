use std::fs::File;
use std::io::{self, BufReader, BufWriter};

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
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let filmes = bincode::deserialize_from(reader)?;
    Ok(filmes)
}


