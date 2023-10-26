# if
No hace falta usar paréntesis, y se pueden usar dentro de sentencias `let`.


# Bucles
Hay tres tipos: `loop`, `while` y `for`.

## loop
Repite el bucle hasta que se le diga explícitamente que pare. Hay dos palabras clave útiles aquí:
- `break`: Para el bucle.
- `continue`: sáltate lo que queda de esta iteración y comienza la siguiente.
Puede devolver un valor incluyéndolo en el `break`:
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("El resultado es {result}");
}
```

Se pueden utilizar etiquetas para los bucles, de forma que se puedan diferenciar. Se pueden utilizar junto con `break` y `continue` para especificar a qué bucle afectan.


## while
Funciona igual que el `while` normal y no tiene mucho que añadir.

## for
Funcionan igual que el resto y no tienen mucho que añadir. Mezcla un poco C y Python.

Tienen cosas como:
```rust
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("The value is {element}");
}

for number in (1..4).rev() {
    println!("{number}");
}
```
