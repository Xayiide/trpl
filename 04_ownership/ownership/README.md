# Ownership - Posesión
Es un conjunto de reglas para gestionar la memoria. Algunos lenguajes de programación usan recolección de basura, otros dejan que sea el programador el que maneje explícitamente la memoria. Rust lo que hace es gestionar la memoria mediante este sistema de posesión, que es un conjunto de reglas que el compilador comprueba para poder terminar la compilación correctamente. Ninguna regla de posesión ralentiza los programas mientras se ejecutan.

## Reglas
- Cada valor en Rust tiene un *dueño*.
- Sólo puede haber un dueño en un momento determinado.
- Cuando el dueño se sale de ámbito (*scope*), el valor se suelta.

## Variable Scope -- Ámbito de variables
El ámbito es el rango dentro de un programa para el cual un elemento es válido. Si tenemos la variable:
```rust
let s = "hello";
```
La variable *s* se refiere a un literal de tipo cadena, donde el valor de la cadena está hardcodeado dentro del texto de nuestro programa. La variable es válida desde el punto en el que se declara hasta el final del ámbito actual.
```rust
{ /* s no es válido aquí porque no está declarada */
    let s = "hello";
} /* aquí termina el ámbito y s deja de ser válida */
```

## Tipo string
Los tipos sencillos se ubican en la pila y es sencillo moverlos, copiarlos, mandar su valor, etc. Es más claro para explicar esto de la gestión de memoria si trabajamos con un tipo que se guarde en el *heap* para ver cómo hace Rust para saber cómo limpiarlos.

Los literales de tipo string son inmutables. No sirven para las situaciones en las que no sabemos su valor de antemano (por ejemplo, aquellas en las que se requiere que el usuario introduzca text). Para estas situaciones Rust tiene un tipo diferente: el tipo `String`. Este tipo gestiona datos guardados en el *heap* y por ello puede manejar texto de tamaños desconocidos en tiempo de compilación.
```rust
let s = String::from("hello"); /* crear String a partir de un literal cadena */
```
Este tipo de cadena sí puede cambiarse (es mutable):
```rust 
let mut s = String::from("Hello");
s.push_str(", world!");
println!("{}", s); /* imprime 'Hello, world!' */
```

## Memoria y asignación (allocation)
Con los literales de tipo cadena sabemos los contenidos en tiepo de compilación y el texto se hardcodea directamente en el ejecutable. Por eso son rápidos y eficientes. Son propiedades que derivan de su inmutabilidad. No podemos reservar y guardar un *blob* de memoria para cada pieza de texto cuyo tamaño es desconocido en compilación y que podría cambiar durante la ejecución. Con el tipo `String` podremos tener un texto mutable que puede crecer, y para ello necesitamos asignar memoria, lo que significa que:
- La memoria se pedirá al asignador de memoria en tiempo de ejecución.
- Necesitamos alguna forma de devolver esa memoria cuando hayamos terminado.

Rust soluciona esto sin recolector de basura y sin delegar toda al gestión al programador: la memoria se devuelve automáticamente cuando sale de ámbito. Cuando la variable sale de ámbito, Rust llama a una función especial para nosotros llamada `drop`. Es llamada automáticamente en las llaves de cierre.
> En C++ hay un patrón llamado *Resource Acquisition Is Initialization* que funciona parecido a cómo funciona la función *drop*.

### Move
Cuando se trata con valores almacenados en la pila, al hacer
```rust
let x = 5;
let y = x;
```
Se crean dos variables, `x` e `y` con el mismo valor: 5. Sin embargo, si lo hacemos con `String`s, que se almacenan en el *heap*:
```rust
let s1 = String::from("hello");
let s2 = s1;
```
Se nos presenta un problema. Cuando se llame a `drop`, tanto `s1` como `s2` apuntan a la misma memoria. Va a suceder un *double free*. Para evitarlo, Rust invalida el primer puntero, `s1`. Ya no se podrá usar, y dará un error al intentarlo. Esto es parecido a lo que hacen otro lenguajes como Python con las *deep copies* y las *shallow copies*. Es como un *shallow copy* pero además invalidando la primera variable. Rust **nunca** va a hacer automáticamente copias *deep*, por lo que cualquier copia, al ser *shallow*, es seguro que será también rápida y barata.

### Clone
Si queremos en realidad hacer una copia *deep*, podremos usar la función `clone()`:
```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

En la pila no hay diferencia entre una copia *deep* y una *shallow*. Se hacen copias de verdad, son dos variables diferentes que se almacenan en la pila y se liberan como en cualquier otro lenguaje.

## Funciones y posesión
Cuando se pasa un valor a una función, las mecánicas son similares. El valor se moverá o se copiará como con la asignación.
```rust
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
```
