
# Guía de Instalación de Visual Studio Code y Extensiones para Rust

## Instalación de Visual Studio Code

1. **Descarga**
   - Visita la [página oficial de Visual Studio Code](https://code.visualstudio.com/).
   - Haz clic en el botón de descarga para tu sistema operativo (Windows, macOS, o Linux).

2. **Instalación en Windows**
   - Ejecuta el instalador descargado.
   - Sigue las instrucciones del asistente de instalación.
   - Asegúrate de marcar la opción "Agregar a PATH" durante la instalación.

3. **Instalación en macOS**
   - Abre el archivo `.zip` descargado.
   - Arrastra Visual Studio Code a la carpeta de Aplicaciones.

4. **Instalación en Linux**
   - Debian/Ubuntu:
     ```
     sudo apt update
     sudo apt install software-properties-common apt-transport-https wget
     wget -q https://packages.microsoft.com/keys/microsoft.asc -O- | sudo apt-key add -
     sudo add-apt-repository "deb [arch=amd64] https://packages.microsoft.com/repos/vscode stable main"
     sudo apt update
     sudo apt install code
     ```
   - Fedora/RHEL:
     ```
     sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc
     sudo sh -c 'echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" > /etc/yum.repos.d/vscode.repo'
     sudo dnf check-update
     sudo dnf install code
     ```

5. **Verificación de la Instalación**
   - Abre Visual Studio Code.
   - Verifica que se inicie correctamente y no haya errores.

## Extensiones Útiles para Proyectos Rust

Para instalar extensiones, abre VS Code, ve a la barra lateral izquierda y haz clic en el icono de extensiones (cuatro cuadrados).

1. **rust-analyzer**
   - Proporciona análisis de código en tiempo real, autocompletado, y más.
   - Búscala en el marketplace de extensiones y haz clic en "Instalar".

2. **CodeLLDB**
   - Depurador para Rust y otros lenguajes.
   - Esencial para la depuración de programas Rust.

3. **Better TOML**
   - Mejora el soporte para archivos TOML, utilizados en `Cargo.toml`.

4. **crates**
   - Ayuda a gestionar dependencias en proyectos Rust.

5. **Rust Test Explorer**
   - Interfaz visual para ejecutar y depurar pruebas de Rust.

6. **Rust Syntax**
   - Mejora el resaltado de sintaxis para código Rust.


## Configuración Adicional

1. **Configurar rust-analyzer**
   - Abre la configuración de VS Code (File > Preferences > Settings).
   - Busca "rust-analyzer" y ajusta las opciones según tus preferencias.

2. **Configurar el Formato Automático**
   - En la configuración, busca "Rust format".
   - Activa "Format On Save" para formatear automáticamente tu código Rust.



