use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secreto = rand::thread_rng().gen_range(1..=100);
    println!("¡Adivina el número! ({secreto})");

    loop {
        let mut guess = String::new();

        println!("Input:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error al leer la línea");

        /* parse() lo convierte a u32 porque lo infiere del tipo de guess */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Elección: {guess}");

        /* Compara guess con secreto. Sabe que guess es u32, así que
         * infiere que secreto tb lo es aunque no se haya especificado
         * su tipo */
        match guess.cmp(&secreto) {
            Ordering::Less    => println!("Demasiado pequeño"),
            Ordering::Greater => println!("Demasiado grande"),
            Ordering::Equal   => {
                println!("Bieeeeeeen");
                break;
            }
        }
    }
}

