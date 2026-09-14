use std::io;

fn main() {
    println!("Entre une phrase :");

    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("Merci de saisir une phrase !");

    println!("      ");
    println!("Caractères : {}", sentence.len() - 1);
    println!("Mots : {}", sentence.trim().split_whitespace().count());
}
