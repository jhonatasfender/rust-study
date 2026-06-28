use std::io;
use rand::RngExt;

fn main() {
    println!("Advinhe o número!");

    let numero_secreto = rand::rng().random_range(1..101);

    println!("O número secreto é: {}", numero_secreto);

    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    println!("Você disse: {}", palpite);
}
