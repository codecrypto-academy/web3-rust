# Ejemplos de Ownership en Rust

El ownership es uno de los conceptos más fundamentales y únicos de Rust. Aquí hay varios ejemplos que demuestran cómo funciona:

## 1. Transferencia de Propiedad Básica

```rust
fn main() {
    let s1 = String::from("hola");
    let s2 = s1;  // s1 se mueve a s2

    // println!("{}", s1);  // Esto causaría un error de compilación
    println!("{}", s2);  // Esto es válido
}
```

En este ejemplo, la propiedad de `s1` se transfiere a `s2`. Después de esta transferencia, `s1` ya no es válida.

## 2. Clonación para Evitar Mover

```rust
fn main() {
    let s1 = String::from("hola");
    let s2 = s1.clone();  // Crea una copia profunda de s1

    println!("s1 = {}, s2 = {}", s1, s2);  // Ambas son válidas
}
```

Aquí, `clone()` crea una nueva copia de los datos, permitiendo que tanto `s1` como `s2` sean válidas.

## 3. Ownership y Funciones

```rust
fn main() {
    let s = String::from("hola");
    tomar_propiedad(s);
    // println!("{}", s);  // Esto causaría un error

    let x = 5;
    hacer_copia(x);
    println!("{}", x);  // Esto es válido porque i32 implementa Copy
}

fn tomar_propiedad(una_string: String) {
    println!("{}", una_string);
}  // una_string sale del ámbito y se libera

fn hacer_copia(un_entero: i32) {
    println!("{}", un_entero);
}
```

Este ejemplo muestra cómo la propiedad se transfiere cuando se pasa un valor a una función.

## 4. Retorno de Propiedad

```rust
fn main() {
    let s1 = dar_propiedad();
    let s2 = String::from("hola");
    let s3 = tomar_y_dar(s2);
    println!("{}", s1);
    // println!("{}", s2);  // Esto causaría un error
    println!("{}", s3);
}

fn dar_propiedad() -> String {
    let una_string = String::from("tuya");
    una_string  // Se retorna la propiedad
}

fn tomar_y_dar(una_string: String) -> String {
    una_string  // Se retorna la propiedad
}
```

Aquí se muestra cómo la propiedad puede ser devuelta por una función.

## 5. Referencias y Préstamos

```rust
fn main() {
    let s1 = String::from("hola");
    let len = calcular_longitud(&s1);
    println!("La longitud de '{}' es {}.", s1, len);
}

fn calcular_longitud(s: &String) -> usize {
    s.len()
}
```

Este ejemplo utiliza una referencia para acceder al valor sin tomar la propiedad.

## 6. Referencias Mutables

```rust
fn main() {
    let mut s = String::from("hola");
    cambiar(&mut s);
    println!("{}", s);
}

fn cambiar(una_string: &mut String) {
    una_string.push_str(", mundo");
}
```

Aquí se muestra cómo usar referencias mutables para modificar un valor sin tomar la propiedad.

## 7. Slice y Ownership

```rust
fn main() {
    let s = String::from("hola mundo");
    let palabra = primera_palabra(&s);
    println!("La primera palabra es: {}", palabra);
}

fn primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

Este ejemplo demuestra cómo los slices permiten referenciar una parte de una colección sin tomar la propiedad.

## 8. Structs y Ownership

```rust
struct MiStruct {
    campo: String,
}

fn main() {
    let mi_struct = MiStruct {
        campo: String::from("hola"),
    };
    
    let MiStruct { campo } = mi_struct;
    // println!("{}", mi_struct.campo);  // Esto causaría un error
    println!("{}", campo);
}
```

Este ejemplo muestra cómo la propiedad funciona con estructuras.

Estos ejemplos ilustran los conceptos clave de ownership en Rust, incluyendo transferencia de propiedad, préstamo (borrowing), y cómo estos conceptos interactúan con diferentes tipos de datos y estructuras. El sistema de ownership de Rust es fundamental para garantizar la seguridad de la memoria sin necesidad de un recolector de basura.