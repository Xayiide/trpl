# Tipos de datos
Rust es de tipado estático. El compilador necesita saber los tipos de una forma u otra. A veces los puede inferir, pero otras veces no.

## Tipos escalares
Un tipo escalar representa un único valor. Principalmente son 4:
 1. Enteros
 2. Flotantes
 3. Booleanos
 4. Caracteres

### Enteros
Con o sin signo. Se representan en memoria con complemento a dos.
Con signo el rango es (-2<sup>n-1</sup>) hasta (2<sup>n-1</sup>-1), incluido. *n* es el número de bits usados.

También hay dos tipos especiales, *isize* y *usize* que dependen de la arquitectura en la que se ejecute. La barra baja (\_) se puede usar para separar dígitos (bieeeen).


> Desbordamiento de enteros. Cuando se compila en modo debug, Rust incluye comprobaciones para los desbordamientos de enteros. Pero en modo release no se incluyen. En este caso lo que hace Rust en caso de desbordamiento es *two's complement wrapping*, es decir, los valores más grandes que el valor máximo dan la vuelta. En caso de un `u8`, el valor 256 se vuelve 0 y 257 se vuelve 1.

### Flotantes
Respetan el estándar IEEE-754 de coma flotante. `f32` y `f64`.


### Booleanos
Pues `true` y `false`.

### Caracteres
Pueden ser también símbolos y emojis. Igual que C en cuanto a las comillas (compillas simples para caracteres, dobles para cadenas). No confiarse. El concepto humano de caracter no es el mismo que el de Rust.




## Tipos compuestos
### Tuplas
Agrupaciones que pueden contener tipos diferentes. Longitud fija, no se pueden agrandar ni reducir su tamaño.

```rust
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; /* desestructurar la tupla */
    println!("y: {y}");

    let five_hundrer   = tup.0;
    let six_point_four = tup.1;
```

### Array
Agrupaciones del mismo tipo. Longitud fija también.
```rust
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5]; /* array de i32 de tamaño 5 */
    let c = [3; 5]; /* array de 5 posiciones inicializadas a 3 */
    let d = a[0];
```

