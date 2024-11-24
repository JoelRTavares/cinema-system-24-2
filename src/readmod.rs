use crate::iocontroller::{load_from_file};

pub fn read_movies(){
	match load_from_file("filmes.bin"){
        Ok(loaded_filmes) => {
            for (index, filme) in loaded_filmes.iter().enumerate() {
                println!("{}: {:#?}", index, filme);
            }
                        
        }, 
        Err(e) => { 
             eprintln!("Erro ao carregar dados: {}", e); 
        }
    };
}