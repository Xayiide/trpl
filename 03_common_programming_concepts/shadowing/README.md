# Shadowing
Declarar una variable con el mismo nombre que otra anterior. Se dice que la primera variable está `shadowed` por la segunda, lo que significa que la segunda será la que vea el compilador cuando utilices el nombre de esa variable.

Para shadowear una variable usamos su nombre y repitiendo el uso de la palabra `let`.

Shadowear es diferente que marcar una variable como `mut` porque dará error de compilación si la reasignamos sin usar `let`. Usando `let` podemos efectuar unas pocas transformaciones de valor pero mantener la variable como inmutable.

La otra diferencia es que shadoweando podemos cambiar el tipo pero manteniendo el nombre.
