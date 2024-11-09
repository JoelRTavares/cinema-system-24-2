use chrono::NaiveDate;

#[derive(Debug)]
enum Genero{
    Acao,
    Animacao,
    Comedia,
    Drama,
    Gospel,
    Suspense,
}

#[derive(Debug)]
struct Filme{
    nome: String,
    bilhetes_vendidos:u32,
    data_lancamento:NaiveDate,
    genero:Genero,
}
fn main() {

    let movie = Filme{
        nome:String::from("Filme muito engracado!"),
        bilhetes_vendidos: 1200,
        data_lancamento: NaiveDate::from_ymd_opt(2024, 10, 24).expect("Data invalida!!!"),
        genero: Genero::Comedia,
    };
    println!("Filme definido: {:#?}", movie);
}
