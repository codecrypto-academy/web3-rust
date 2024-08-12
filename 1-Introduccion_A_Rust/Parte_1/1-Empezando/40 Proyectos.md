## HELLO CARGO!

### PROYECTO BINARIO Y DE LIBRERIA
```sh
cargo new my-app

cargo new --lib my-lib 
```
Con --lib creamos una libreria.
Si no podemos nada se crea un binario. Tambien podriamos poner --bin

Tambien podriamos usar el comando de cargo init

```Bash
cargo init my-app

cargo init --lib my-lib
```

Para construir el ejecutable o la libraria usamos
```Bash
cargo build
```

Pra ejecutar un binario usamos
```Bash
cargo run
```

Para ejecutar un binario, estando en el directorio

```Bash
cargo run 
```
### DEBUG O RELEASE

Para produccion se usa el modo release y para dev el modo debug. 
El modo release, como en otro lenguajes, genera ejecutable mas pequeños

```Bash
cargo build --release // para construir en modo release

cargo run --release // para ejecutar en modo release

```

## LIBRERIAS Y BINARIOS

### CREAMOS UNA LIBRERIA Y UN BINARIO
```Bash
cargo new --lib lib

cargo new --bin app // bin es opcional
```

Usamos el fichero Cargo.tml del binario para especificar que queremos usar la libreria

Usamos un path relativo.

```toml
[dependencies]
lib = {path = "../lib}
```

La libreria tiene el siguiente codigo con una funcion.



```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

y el main.rs del binario tiene
```rust
fn main() {
    println!("Hello, world!");
}
```
En el fichero anterior ponemos
```rust
use lib;
fn main() {
    // usamos la funcion add de la libreria lib
    println!("Hello, world! {}", lib::add(1,2) );
}

```

### USAMOS UNA LIBRERIA PUBLICADA EN GITHUB

```toml
[dependencies]
ring = { git = "https://github.com/briansmith/ring", branch = "main" }
```
En el fichero main.rs
```rs
// queremos usar la function digest
use ring::digest;
fn main() {
   let input = b"hello, world";
   // ejecutamos la function. El & significa 
   // que pasamos una referencia (se vera con detalle mas adelante)
   let digest = digest::digest(&digest::SHA256, input);
   println!("{:?}", digest);
}
```

## WORKSPACES
Es un tipo de proyectos donde tenemos multiples librerias y binarios. 
Creamos un directorio y metemos un fichero cargo.toml

```toml
[workspace] 
members = [ 
    "app", 
    "lib"
]
```
Creamos los proyectos 
```Bash
cargo init app
cargo init --lib lib
```
Ahora con
```
cargo run
cargo build 
```
se construye la libreria y el binario o se ejecuta el binario

![Ficheros del direcotio](image.png)

## DESARROLLO EN UN CONTENEDOR CON VSCODE

Con el comando siguiente creamos un container que tiene lo necesario para trabajar con vscode

```Bash
docker run --interactive --tty --restart=always rust
```

Ahora adjuntamos el container a visual studio code Ctrl+Sift+p (adjuntar al container que esta en ejecucion)

## CREACION DE UNA IMAGEN DOCKER A PARTIR DE UN PROYECTO.
En el proyecto tenemos que meter el siguiente Dockerfile


```dockerfile
FROM docker.io/rust:1-slim-bookworm AS build
## cargo package name: customize here or provide via --build-arg
ARG pkg=web01
WORKDIR /build
COPY . .
RUN --mount=type=cache,target=/build/target \
--mount=type=cache,target=/usr/local/cargo/registry \
--mount=type=cache,target=/usr/local/cargo/git \
set -eux; \
cargo build --release; \
objcopy --compress-debug-sections target/release/$pkg ./main

FROM docker.io/debian:bookworm-slim
WORKDIR /app
## copy the main binary
COPY --from=build /build/main ./
## copy runtime assets which may or may not exist
COPY --from=build /build/Rocket.tom[l] ./static
COPY --from=build /build/stati[c] ./static
COPY --from=build /build/template[s] ./templates
## ensure the container listens globally on port 8080
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
CMD ./main
```
En cuanto al fichero main.rs de nuestro binario, vamos a realizar un web server basico
```rs
// este servidor web tiene 2 rutas 
// las dos son get. Una devuelve Hello, world
// la otra /hello necesita un parametro llamado name
// y genera una cadena concatenando Hello con el parametro.
use rocket::{get, routes, launch};
#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
   format!("Hello, {}!", name)
}
#[launch]
// este es el main del web server donde a;adimos las rutas
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![hello])
}
```
En el fichero toml, metemos la depedencia de rocket, que es como express en nodejs
```toml
[package]
name = "web01"
version = "0.1.0"
edition = "2021"
[dependencies]
rocket = "0.5.1"
```
Tambien creamos un fichero llamado .dockerignore para que docker no use algunos ficheros y directorios.
```
target
```
Una vez tenemos esto creamos la imagen con
```Bash
docker build --build-arg pkg=web01 -t app  .
```
y con la imagen creada, creamos el container con
```Bash
docker run –p 8081:8080  app
```
Podremos hacer dese la terminal
```Bash
curl http://localhost:8081
curl http://localhost:8081/hello/jvh
```