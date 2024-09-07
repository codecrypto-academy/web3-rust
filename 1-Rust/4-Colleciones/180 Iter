
# Iteradores y Métodos de Iteración

## Ejemplos de Uso de `iter()` y `iter_mut()`

Primero, definamos una estructura simple para nuestros ejemplos:

```rust
#[derive(Debug)]
struct Persona {
    nombre: String,
    edad: u32,
}
```

Ahora, veamos ejemplos de `iter()` e `iter_mut()`:

1. Usando `iter()` para lectura

```rust
fn main() {
    let personas = vec![
        Persona { nombre: String::from("Alice"), edad: 30 },
        Persona { nombre: String::from("Bob"), edad: 25 },
        Persona { nombre: String::from("Charlie"), edad: 35 },
    ];

    // Usar iter() para leer los datos
    for persona in personas.iter() {
        println!("{} tiene {} años", persona.nombre, persona.edad);
    }
}
```

En este ejemplo, `iter()` se usa para iterar sobre el vector sin modificar sus elementos.

2. Usando `iter_mut()` para modificación

```rust
fn main() {
    let mut personas = vec![
        Persona { nombre: String::from("Alice"), edad: 30 },
        Persona { nombre: String::from("Bob"), edad: 25 },
        Persona { nombre: String::from("Charlie"), edad: 35 },
    ];

    // Usar iter_mut() para modificar los datos
    for persona in personas.iter_mut() {
        persona.edad += 1;
    }

    // Imprimir los resultados
    for persona in personas.iter() {
        println!("{} ahora tiene {} años", persona.nombre, persona.edad);
    }
}
```

Aquí, `iter_mut()` permite modificar cada elemento del vector.

3. Ejemplo combinado: filtrar con `iter()` y modificar con `iter_mut()`

```rust
fn main() {
    let mut personas = vec![
        Persona { nombre: String::from("Alice"), edad: 30 },
        Persona { nombre: String::from("Bob"), edad: 25 },
        Persona { nombre: String::from("Charlie"), edad: 35 },
    ];

    // Usar iter() para encontrar personas mayores de 30
    let mayores_de_30: Vec<&String> = personas.iter()
        .filter(|p| p.edad > 30)
        .map(|p| &p.nombre)
        .collect();

    println!("Personas mayores de 30: {:?}", mayores_de_30);

    // Usar iter_mut() para modificar edades
    for persona in personas.iter_mut() {
        if persona.edad < 30 {
            persona.edad += 5;
        }
    }

    // Imprimir resultados
    for persona in personas.iter() {
        println!("{} ahora tiene {} años", persona.nombre, persona.edad);
    }
}
```

Este ejemplo muestra cómo usar `iter()` para operaciones de lectura y filtrado, y `iter_mut()` para modificaciones.

Cuándo usar cada uno:

- Use `iter()` cuando:
  1. Solo necesita leer los elementos del vector.
  2. No necesita modificar los elementos.
  3. Quiere realizar operaciones como filtrado, mapeo o reducción sin cambiar los datos originales.

- Use `iter_mut()` cuando:
  1. Necesita modificar los elementos del vector.
  2. Quiere realizar cambios in-situ en los datos existentes.
  3. Necesita acceso mutable a los elementos para operaciones más complejas.

Consideraciones adicionales:

- `iter()` proporciona referencias inmutables (&T) a los elementos.
- `iter_mut()` proporciona referencias mutables (&mut T) a los elementos.
- Si necesita consumir el vector (es decir, mover sus elementos), puede usar `into_iter()`.

Ejemplo de `into_iter()`:

```rust
fn main() {
    let personas = vec![
        Persona { nombre: String::from("Alice"), edad: 30 },
        Persona { nombre: String::from("Bob"), edad: 25 },
    ];

    // Usar into_iter() para consumir el vector
    for persona in personas.into_iter() {
        println!("{} tiene {} años", persona.nombre, persona.edad);
    }

    // personas ya no es utilizable aquí, ha sido consumido
}
```

En resumen, la elección entre `iter()`, `iter_mut()`, e `into_iter()` depende de si necesitas leer, modificar o consumir los elementos del vector, respectivamente. Usar el método correcto no solo hace que tu código sea más claro, sino que también aprovecha las características de seguridad y rendimiento de Rust.