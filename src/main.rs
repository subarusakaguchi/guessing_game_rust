use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Chute um número de 0 a 100!");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Coloque aqui o seu valor: ");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Não consegui entender o seu chute!");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("O seu chute foi: {guess}");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("O número secreto é {secret_number}");
                println!("VITÓRIA!");
                break;
            }
        }
    }

}
