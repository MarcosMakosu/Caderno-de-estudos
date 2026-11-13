use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let agora = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let numero_secreto = (agora.subsec_nanos() % 100 + 1) as u32;

    println!("Adivinhe o número entre 1 e 100!");

    loop {
        println!("Digite seu chute:");

        let mut chute = String::new();
        io::stdin().read_line(&mut chute).expect("erro ao ler");
        let chute: u32 = chute.trim().parse().expect("digite um número");

        if chute < numero_secreto {
            println!("Muito baixo!");
        } else if chute > numero_secreto {
            println!("Muito alto!");
        } else {
            println!("Acertou! O número era {}", numero_secreto);
            break;
        }
    }
}