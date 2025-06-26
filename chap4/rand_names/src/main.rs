use std::io;

use rand::Rng;

fn main() {
    let mut nome = String::new();
    let prefixo = ["Elen", "Feri", "Ail", "Oni", "Ia", "Fa"];
    let corpo = ["eare", "li", "nado", "cas"];
    let sufixo = ["lu", "minu", "lua", "flor", "melo"];

    println!("Quantos nomes você precisa gerar?");

    let mut quantidade = String::new();

    io::stdin()
        .read_line(&mut quantidade)
        .expect("Failed read line");

    let quantidade: u32 = quantidade.trim().parse().expect("Error");

    for _i in 0..quantidade {
        create_name(&mut nome, &prefixo, &corpo, &sufixo);
        println!("{nome}");

        nome = String::new();
    }
}

fn create_name(nome: &mut String, prefixo: &[&str], corpo: &[&str], sufixo: &[&str]) {
    let new = rand::rng().random_range(0..=(prefixo.len() - 1));

    nome.push_str(prefixo[new]);

    let new = rand::rng().random_range(0..=(corpo.len() - 1));

    nome.push_str(corpo[new]);

    let new = rand::rng().random_range(0..=(sufixo.len() - 1));

    nome.push_str(sufixo[new]);
}
