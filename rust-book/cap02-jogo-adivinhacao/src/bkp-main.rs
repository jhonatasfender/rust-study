use std::io;

fn main() {
    println!("Adivinhe o número");

    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falta ao ler entrada");

    println!("Você disse: {}", palpite);
}
