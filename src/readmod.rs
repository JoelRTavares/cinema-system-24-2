use crate::iocontroller::{load_from_file};

pub fn read_movies(){
	match load_from_file("filmes.bin"){
        Ok(loaded_filmes) => {
            println!("{:#?}", loaded_filmes);            
        }, 
        Err(e) => { 
             eprintln!("Erro ao carregar dados: {}", e); 
        }
    };
}