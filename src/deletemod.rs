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
    if filmes.len() == 0{
        eprintln!("Não existem filmes para serem deletados!\nPor favor, crie algum filme para usar essa funcionalidade"); 
        return;
    }
    loop{
	    println!("Escolha o filme que gostaria de excluir (selecione um índice entre 0 e {}): ", filmes.len() - 1);
        for (index, filme) in filmes.iter().enumerate() {
            println!("{}: {:#?}", index, filme);
        }

	    let mut ind = String::new();
        io::stdin()
            .read_line(&mut ind)
            .expect("Error reading this line!");


        let ind = match checar_indice(&ind, filmes.len()) {
                Ok(num) => num,
                Err(e) => {
                    println!("Erro ao deletar o filme: {e}");
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
        println!("Iniciando a exclusão de outro filme...");
    }
}

fn checar_indice(ind: &str, size:usize) -> Result<usize, String>{
    let ind: usize = match ind.trim().parse() {
            Ok(num) if num < size => num,
            _ => {
                return Err(format!("Por favor, insira um índice válido (entre 0 e {})!", size - 1));
            },
    };
    Ok(ind)
}



#[cfg(test)]
mod tests{
    use super::*;

    //Testes de nome: 
    #[test]
    fn escolher_indice_valido() ->Result<(), String>{
        let indice = "0";
        let size = 10;
        match checar_indice(&indice, size) { 
            Ok(_) => Ok(()), 
            Err(e) => Err(e), 
        }
    }
    #[test]
    fn escolher_indice_negativo() ->Result<(), String>{
        let indice = "-1";
        let size = 10;
        match checar_indice(&indice, size) { 
            Ok(_) => Err(String::from("Não deveria ser possível aceitar um índice negativo!")), 
            Err(_) => Ok(()),  
        }
    }
    #[test]
    fn escolher_indice_acima_do_tamanho() ->Result<(), String>{
        let indice = "14";
        let size = 10;
        match checar_indice(&indice, size) { 
            Ok(_) => Err(String::from("Não deveria ser possível aceitar um índice acima do tamanho!")), 
            Err(_) => Ok(()),  
        }
    }
}