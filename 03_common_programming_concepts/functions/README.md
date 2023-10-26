# Parámetros y argumentos
Podemos definir una función para que tenga parámetros. Los valores concretoscon los que se llama la función se conocen como *argumentos*.

```rust
fn function(x: i32) {
    /* x es un parámetro */
}

function(28); /* 28 es un argumento */
```

# Sentencias y expresiones
Es importante entender esta distinción porque Rust es un lenguaje basado en expresiones.
- Sentencia: instrucción que realiza una acción pero no devuelve ningún valor.
- Expresión: Evalúan y llegan a un valor resultado.

`let y = 6;` es una sentencia. Como no devuelve un valor, no puedes asignar una sentencia `let` a otra variable: `let x = (let y = 6);` es incorrecto. En C o en Ruby, una asignación de valor sí devuelven el valor de la asignación (`x = y = 6`). Las definiciones de funciones también son sentencias.  

Las expresiones evalúan y llegan a un valor. Prácticamente el resto del código de Rust son expresiones. Una operación matemática es una expresión, Las expresiones pueden ser parte de las sentencias, el 6 en `let y = 6;` es una expresión que evalúa al valor 6. Llamar a una función o macro es una expresión. Un nuevo bloque de ámbito es una expresión:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 /* sin punto y coma */
    }
    println!("El valor de y es {y}");
}

```

El valor del bloque en el que se declara `x` sí que devuelve el valor 4, que es enlazado con la variable y. el `x + 1` no tiene punto y coma al final: las expresiones no incluyen el punto y coma al final. Si añades un punto y coma al final de una expresión, la conviertes en una sentencia y no devolverá un valor.


# Funciones que devuelven valores
No se les pone nombre pero sí el tipo.

Una función que devuelve valor puede ser:
```rust
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = five();
    println!("El valor de x es {x}"); /* 5 */
    let x = plus_one(x);
    println!("El valor de x es {x}"); /* 6 */
}
```

Pero si por el contrario ponemos un punto y coma al final de la función:
```rust
fn plus_one(x: i32) -> i32 {
    x + 1;
}
```
Entonces dará error de compilación porque no devuelve ningún valor.
