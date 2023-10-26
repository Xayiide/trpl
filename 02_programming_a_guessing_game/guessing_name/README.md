# Versiones
En `Cargo.toml` se especifica una versión de una librería de la que se depende. Aunque esa librería saque versiones nuevas, en el fichero Cargo.lock se especificará la versión usada en la primera compilación para asegurar compilaciones idénticas y reproducibles.

El comando `cargo update` ignora el fichero `Cargo.lock` y se encarga de actualizar las dependencias a la última versión que siga cumpliendo las especificaciones de tu `Cargo.toml`, cambiando lo escrito en `Cargo.lock`.


# Docs
Cada crate tiene documentación. Se puede acceder con  
`cargo doc --open`  
Se abrirá en el navegador.

