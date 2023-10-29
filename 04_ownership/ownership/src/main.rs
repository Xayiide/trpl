fn main() {
    let s = String::from("hello"); /* s entra en ámbito */
    
    takes_ownership(s); /* el valor de s se mueve dentro de la función */
                        /* y deja de ser válido aquí */

    let x = 5; /* x entra en ámbito */

    makes_copy(x); /* x se movería dentro de la función, pero i32 tiene
                      función Copy, por lo que se puede seguir usando x
                      después */
} /* x se sale de ámbito, luego s. Pero como el valor de s se ha movido, no sucede nada especial */

fn takes_ownership(some_string: String) { /* some_string entra en ámbito */
    println!("{}", some_string);
} /* some_string sale de ámbito y se llama a drop(). La memoria se libera */

fn makes_copy(some_integer: i32) { /* some_integer entra en ámbito */
    println!("{}", some_integer);
} /* some_integer sale de ámbito. No sucede nada en especial */
