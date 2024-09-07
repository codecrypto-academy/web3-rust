# Parseo de Entrada

```rust
use std::io::{self, Write};

fn main() {
    loop {
        // Preparar un String para almacenar la entrada del usuario
        let mut input = String::new();

        // Solicitar entrada al usuario
        print!("Por favor, ingrese un número entero: ");
        io::stdout().flush().unwrap(); // Asegurarse de que el mensaje se muestre inmediatamente

        // Leer la entrada del usuario
        io::stdin().read_line(&mut input)
            .expect("Error al leer la línea");

        // Intentar parsear la entrada a un entero
        match input.trim().parse::<i32>() {
            Ok(numero) => {
                println!("¡Número válido ingresado: {}!", numero);
                break; // Salir del loop si el parseo fue exitoso
            },
            Err(_) => {
                println!("Eso no es un número entero válido. Intente de nuevo.");
                // El loop continuará
            }
        }
    }

    println!("Programa finalizado.");
}
```

Explicación del código:

1. Usamos `use std::io::{self, Write};` para importar las funcionalidades de E/S necesarias.

2. El programa se ejecuta en un `loop` infinito que solo se romperá cuando se ingrese un número válido.

3. Dentro del loop:
   - Creamos un `String` vacío para almacenar la entrada del usuario.
   - Usamos `print!` para mostrar un mensaje solicitando la entrada.
   - `io::stdout().flush().unwrap();` asegura que el mensaje se muestre inmediatamente.
   - `io::stdin().read_line(&mut input)` lee la entrada del usuario.

4. Usamos `match` para manejar el resultado de `input.trim().parse::<i32>()`:
   - Si el parseo es exitoso (Ok), imprimimos el número y salimos del loop con `break`.
   - Si hay un error (Err), informamos al usuario y el loop continúa.

5. El programa termina cuando se ingresa un número válido y se sale del loop.

Este código demuestra:
- Lectura de entrada desde la consola.
- Manejo de errores al parsear strings a enteros.
- Uso de loops para repetir una operación hasta que se cumpla una condición.
- Uso de `match` para manejar los resultados de operaciones que pueden fallar.

Es un buen ejemplo de cómo Rust maneja la entrada del usuario y la conversión de tipos de manera segura y robusta.