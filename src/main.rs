use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Procurando por: {}", query);
    println!("No arquivo: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Algo deu errado ao ler o arquivo");

    println!("Com o texto:\n{}", contents);
}

// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html