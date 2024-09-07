# Soluccion del ejercicio.

```rust
use std::io::{self, Write};
use rand::Rng;

fn main() {
    // Generar un número aleatorio entre 1 y 10
    let numero_secreto = rand::thread_rng().gen_range(1..=10);

    println!("¡Adivina el número entre 1 y 10!");

    loop {
        // Preparar un String para almacenar la entrada del usuario
        let mut input = String::new();

        // Solicitar entrada al usuario
        print!("Introduce tu número: ");
        io::stdout().flush().unwrap(); // Asegurarse de que el mensaje se muestre inmediatamente

        // Leer la entrada del usuario
        io::stdin().read_line(&mut input)
            .expect("Error al leer la línea");

        // Intentar parsear la entrada a un entero
        match input.trim().parse::<i32>() {
            Ok(numero) => {
                if numero < 1 || numero > 10 {
                    println!("Por favor, introduce un número entre 1 y 10.");
                    continue;
                }

                if numero < numero_secreto {
                    println!("Menor. Intenta de nuevo.");
                } else if numero > numero_secreto {
                    println!("Mayor. Intenta de nuevo.");
                } else {
                    println!("¡Ganaste! El número era {}.", numero_secreto);
                    break; // Salir del loop si el número es correcto
                }
            },
            Err(_) => {
                println!("Por favor, introduce un número válido.");
                // El loop continuará
            }
        }
    }

    println!("Juego terminado.");
}
```

Para ejecutar este programa, necesitarás añadir la dependencia `rand` a tu `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
```

Explicación del código:

1. Usamos `use std::io::{self, Write};` para las operaciones de E/S y `use rand::Rng;` para generar números aleatorios.

2. Generamos un número aleatorio entre 1 y 10 usando `rand::thread_rng().gen_range(1..=10)`.

3. El programa se ejecuta en un `loop` que continúa hasta que el usuario adivine el número correcto.

4. Dentro del loop:
   - Solicitamos al usuario que introduzca un número.
   - Leemos la entrada del usuario.
   - Parseamos la entrada a un entero.

5. Usamos `match` para manejar el resultado del parseo:
   - Si es un número válido, comparamos con el número secreto.
   - Si no es válido, informamos al usuario y continuamos el loop.

6. Comparamos el número introducido con el número secreto:
   - Si es menor, imprimimos "Menor".
   - Si es mayor, imprimimos "Mayor".
   - Si es igual, imprimimos "¡Ganaste!" y salimos del loop.

7. También verificamos que el número esté entre 1 y 10.

Este programa demuestra:
- Generación de números aleatorios en Rust.
- Manejo de entrada del usuario y parseo de strings a enteros.
- Uso de loops y control de flujo.
- Comparaciones y lógica condicional.

Es un buen ejemplo de un juego simple de adivinanza implementado en Rust, mostrando varias características importantes del lenguaje.