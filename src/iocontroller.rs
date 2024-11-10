use std::fs::File;
use std::io::{self, BufReader, BufWriter};


use crate::Filme;

pub fn save_to_file(filmes: &Vec<Filme>, path: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &filmes)?;
    Ok(())
}

pub fn load_from_file(path: &str) -> io::Result<Vec<Filme>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let filmes = serde_json::from_reader(reader)?;
    Ok(filmes)
}
