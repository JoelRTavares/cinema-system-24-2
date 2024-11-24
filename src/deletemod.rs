use crate::iocontroller::{load_from_file, save_to_file};
use std::io;

pub fn delete_movie(){

    let mut filmes = match load_from_file("filmes.bin"){
        Ok(loaded_filmes) => loaded_filmes, 
        Err(e) => { 
             eprintln!("Erro ao carregar dados: {}", e); 
             return;
        }
    };
    loop{
	    println!("Escolha o filme que gostaria de excluir (selecione um índice entre 0 e {}): ", filmes.len() - 1);
        for (index, filme) in filmes.iter().enumerate() {
            println!("{}: {:#?}", index, filme);
        }

	    let mut ind = String::new();
        io::stdin()
            .read_line(&mut ind)
            .expect("Error reading this line!");
        let ind: usize = match ind.trim().parse() {
                Ok(num) if num >= 0 && num < filmes.len() => num,
                _ => {
                    println!("Por favor, insira um índice válido (entre 0 e {})!", filmes.len() - 1);
                    continue;
                },
        };
        filmes.remove(ind);

        if let Err(e) = save_to_file(&filmes, "filmes.bin") { 
            eprintln!("Erro ao salvar dados: {}", e); 
            break;
        }
        println!("Filme deletado com sucesso!\nGostaria de excluir outro filme?\n0 - Não\n1 - Sim");
        
        let mut opt = String::new();
        io::stdin()
            .read_line(&mut opt)
            .expect("Error reading this line!");
        if opt.trim() == "0" {
            break;
        }
        println!("Iniciando a exclusão de outro filme...")
    }
}