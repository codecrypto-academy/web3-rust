
# Objetivos de Empezando

## Tabla de Contenidos
1. [Instalación de Rust](#instalación-de-rust)
2. [Instalación de Visual Studio Code](#instalación-de-visual-studio-code)
3. [Extensiones de VS Code para Rust](#extensiones-de-vs-code-para-rust)
4. [Creación de Proyectos en Rust](#creación-de-proyectos-en-rust)
5. [Uso de Docker con VS Code para Rust](#uso-de-docker-con-vs-code-para-rust)
6. [Creación de Imágenes Docker para Proyectos Rust](#creación-de-imágenes-docker-para-proyectos-rust)

## Instalación de Rust

### Windows
1. Descarga e instala [rustup-init.exe](https://rustup.rs/)
2. Sigue las instrucciones del instalador

### macOS y Linux
Ejecuta en la terminal:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### WSL (Windows Subsystem for Linux)
1. Instala WSL siguiendo las [instrucciones oficiales](https://docs.microsoft.com/en-us/windows/wsl/install)
2. Abre una terminal de WSL y sigue las instrucciones para Linux

## Instalación de Visual Studio Code

1. Visita [code.visualstudio.com](https://code.visualstudio.com/)
2. Descarga e instala la versión correspondiente a tu sistema operativo

Para WSL: Instala la extensión "Remote - WSL" en VS Code

## Extensiones de VS Code para Rust

Instala las siguientes extensiones:
1. rust-analyzer
2. CodeLLDB
3. Better TOML
4. crates
5. Rust Test Explorer

## Para quien es Rust
Quien deberia de aprender Rust

## Proyectos significativos
Algunos proyectos con Activx, rocket, clap, etc.

## Ediciones de Rust
Relacion de la ediciones


## Creación de Proyectos en Rust

### Crear un Nuevo Proyecto Binario
```bash
cargo new mi_proyecto
cd mi_proyecto
```

### Crear una Nueva Librería
```bash
cargo new mi_libreria --lib
cd mi_libreria
```

### Crear un Workspace
1. Crea un directorio para el workspace:
   ```bash
   mkdir mi_workspace
   cd mi_workspace
   ```
2. Crea un archivo `Cargo.toml` con el siguiente contenido:
   ```toml
   [workspace]
   members = [
       "proyecto1",
       "proyecto2"
   ]
   ```
3. Crea los proyectos miembros:
   ```bash
   cargo new proyecto1
   cargo new proyecto2
   ```

## Uso de Docker con VS Code para Rust

1. Instala Docker en tu sistema
2. Instala la extensión "Remote - Containers" en VS Code
3. Crea un archivo `.devcontainer/devcontainer.json` en tu proyecto:

```json
{
    "name": "Rust",
    "image": "rust:latest",
    "customizations": {
        "vscode": {
            "extensions": [
                "rust-lang.rust-analyzer",
                "vadimcn.vscode-lldb",
                "serayuzgur.crates"
            ]
        }
    }
    // ... existing code ...
}
```

4. Abre el proyecto en un contenedor: Command Palette (F1) > "Remote-Containers: Reopen in Container"

## Creación de Imágenes Docker para Proyectos Rust

1. Crea un `Dockerfile` en la raíz de tu proyecto:

```Dockerfile
FROM rust:1.58 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/myapp/target/release/myapp /usr/local/bin/myapp
CMD ["myapp"]
```

2. Construye la imagen:
```bash
docker build -t myapp .
```

3. Ejecuta el contenedor:
```bash
docker run myapp
```

