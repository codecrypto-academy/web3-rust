# INSTALAR RUST

## Introducción

Para instalar Rust, vamos a la [página oficial de Rust](https://www.rust-lang.org/). Aquí encontrarás casi toda la información necesaria sobre Rust, incluyendo documentación, pasos de instalación, instaladores, enlaces a diferentes comunidades, eventos, entre otras cosas.

## Proceso de Instalación

1. [Descarga](https://www.rust-lang.org/tools/install) el instalador compatible con tu sistema operativo.
2. Para macOS y Linux, ejecuta el siguiente comando en la terminal:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Sigue las instrucciones del asistente de configuración.

### Opciones de Instalación
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

## Verificación de la Instalación

Comprueba la instalación con los siguientes comandos:
- Versión del toolchain manager: `rustup --version`
- Versión del compilador: `rustc --version`
- Versión de cargo: `cargo version`

## Componentes Principales

- `rustc`: Compilador de Rust
- `cargo`: Gestor de paquetes y dependencias de Rust

## Actualización y Desinstalación

- Para actualizar: `rustup update`
- Para desinstalar: `rustup self uninstall`

## Instalación en Windows

Se recomienda instalar [WSL2](https://learn.microsoft.com/en-us/windows/wsl/install) para usar los mismos comandos independientemente del sistema operativo.

## Uso de Rust sin Instalación

Puedes usar el [compilador web de Rust](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021) para programas cortos o pruebas sin necesidad de instalación.

## Ventajas de usar rustup

- Gestión de múltiples versiones de Rust
- Facilidad para actualizaciones sin necesidad de desinstalar versiones anteriores