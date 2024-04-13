## INSTALAR RUST:
### Introducción
Para instalar Rust, vamos a la [página oficial de Rust](https://www.rust-lang.org/).
Aqui van a conseguir casi toda la informacion necesaria sobre Rust, como documentación, pasos a seguir para la instalación, instaladores, enlaces a diferentes comunidades, eventos, entre otras cosas.

Para instalar Rust, lo primero que tenemos que hacer es [descargar](https://www.rust-lang.org/tools/install) el instalador. Selecciona el que sea compatible con tu sistema operativo. En mi caso voy a saleccionar la versión de `rustup` compatible con macOS. 

Corremos el siguiente comando desde la terminal y seguimos las instrucciones del asistente de configuración.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

info: downloading installer

Welcome to Rust!
```

Una vez completada la descarga, instalamos Rust siguiendo las instrucciones en la terminal.

```
Current installation options:

   default host triple: aarch64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>
```
Una de las ventajas de instalar Rust usando `rustup` es que podremos gestionar las versiones que queramos usar y se podrán hacer actualizaciones a través de esta herramienta sin necesidad de desinstalar versiones anteriores.

Para comprobar que Rust se ha instalado correctamente, podemos utilizar los siguientes comandos para verificar lo siguiente:

- Versión del toolchain manager: `rustup --version`.
- Versión del compilador: `rustc --version`.
- Versión de cargo `cargo version`.

`rustc` es el compilador de Rust y puede utilizarse para compilar el código en Rust directamente. Como en otros lenguajes de programación, Rust tiene su propio packet manager, `cargo`, que se encarga de instalar paquetes y mantener dependencias, similar a `npm` para Node o `pip` para Python.

Si esos tres comandos nos responden correctamente en la terminal, quiere decir que ya podemos usar Rust en nuestro equipo.

Si ya tienes una version de Rust y quieres actualizarla, puedes usar el siguiente comando para hacerlo.
```
rustup update
``` 
Si quieres eliminar Rust de tu equipo, puedes usar el siguiente comando.
```
rustup self uninstall
```

## Instalación en Windows

Se recomienda tener instalado un subsistema para Linux como [WSL2](https://learn.microsoft.com/en-us/windows/wsl/install) para que todos usemos los mismos comandos independientemente del sistema operativo.

## Usar Rust sin instalación

Existe un compilador web que se puede usar para escribir programas cortos o hacer pruebas cuando no se tiene un entorno de desarrollo instalado con Rust. En la página oficial de Rust pueden encontrar un [enlace](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021).