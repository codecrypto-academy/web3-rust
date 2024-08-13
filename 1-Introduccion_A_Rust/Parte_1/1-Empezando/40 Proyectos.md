# Rust: Cargo y Gestión de Proyectos

## 1. Hello Cargo!

### Creación de Proyectos

#### Proyecto Binario
```sh
cargo new my-app
# o
cargo new --bin my-app
```

#### Proyecto de Librería
```sh
cargo new --lib my-lib
```

#### Usando `cargo init`
```sh
cargo init my-app
cargo init --lib my-lib
```

### Construcción y Ejecución

#### Construir el proyecto
```sh
cargo build
```

#### Ejecutar un binario
```sh
cargo run
```

### Modos de Compilación

#### Debug (por defecto)
```sh
cargo build
cargo run
```

#### Release (para producción)
```sh
cargo build --release
cargo run --release
```

## 2. Librerías y Binarios

### Creación de una Librería y un Binario
```sh
cargo new --lib lib
cargo new app
```

### Configuración de Dependencias
En `app/Cargo.toml`:
```toml
[dependencies]
lib = { path = "../lib" }
```

### Ejemplo de Código

#### Librería (`lib/src/lib.rs`)
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

#### Binario (`app/src/main.rs`)
```rust
use lib;

fn main() {
    println!("Hello, world! {}", lib::add(1, 2));
}
```

### Uso de Librería desde GitHub
En `Cargo.toml`:
```toml
[dependencies]
ring = { git = "https://github.com/briansmith/ring", branch = "main" }
```

Ejemplo de uso:
```rust
use ring::digest;

fn main() {
   let input = b"hello, world";
   let digest = digest::digest(&digest::SHA256, input);
   println!("{:?}", digest);
}
```

## 3. Workspaces

### Estructura del Workspace
```toml
[workspace]
members = [
    "app",
    "lib"
]
```

### Creación de Proyectos en el Workspace
```sh
cargo init app
cargo init --lib lib
```

### Comandos en el Workspace
```sh
cargo build
cargo run
```

## 4. Desarrollo en Contenedor con VSCode

```sh
docker run --interactive --tty --restart=always rust
```

Luego, adjuntar el contenedor a VSCode (Ctrl+Shift+P, "Attach to Running Container").

## 5. Creación de Imagen Docker para un Proyecto Rust

### Dockerfile
```dockerfile
FROM docker.io/rust:1-slim-bookworm AS build
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
COPY --from=build /build/main ./
COPY --from=build /build/Rocket.tom[l] ./static
COPY --from=build /build/stati[c] ./static
COPY --from=build /build/template[s] ./templates
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
CMD ./main
```

### Ejemplo de Servidor Web (`main.rs`)
```rust
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
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![hello])
}
```

### Configuración de Cargo (`Cargo.toml`)
```toml
[package]
name = "web01"
version = "0.1.0"
edition = "2021"

[dependencies]
rocket = "0.5.1"
```

### .dockerignore
```
target
```

### Construcción y Ejecución de la Imagen Docker
```sh
docker build --build-arg pkg=web01 -t app .
docker run -p 8081:8080 app
```

### Prueba del Servidor
```sh
curl http://localhost:8081
curl http://localhost:8081/hello/jvh
```
