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

## COMPRENDIENDO LA PROPIEDAD

### Qué es la propiedad

La propiedad en Rust es un sistema que gestiona la memoria de forma segura sin necesidad de un recolector de basura.

Ejemplo:
```rust
let s1 = String::from("hola");
let s2 = s1; // s1 ya no es válida aquí
// println!("{}", s1); // Esto causaría un error de compilación
```

#### · Stack and Heap (Pila y Montón)

- Stack: Almacena datos de tamaño fijo conocido en tiempo de compilación.
- Heap: Para datos de tamaño variable o desconocido en tiempo de compilación.

Ejemplo:
```rust
let x = 5; // Almacenado en el stack
let s = String::from("hola"); // El puntero está en el stack, los datos en el heap
```

#### Reglas de propiedad

1. Cada valor tiene un propietario.
2. Solo puede haber un propietario a la vez.
3. Cuando el propietario sale del ámbito, el valor se descarta.

#### Variable Scope (Ámbito de las variables)

El ámbito es el rango donde una variable es válida.

Ejemplo:
```rust
{
    let s = String::from("hola"); // s es válida desde este punto
    // hacer algo con s
} // Este ámbito ha terminado, y s ya no es válida
```

#### Tipo String: ejemplo de tipo de datos complejo

`String` es un tipo que almacena texto de longitud variable en el heap.

Ejemplo:
```rust
let mut s = String::from("hola");
s.push_str(", mundo"); // s ahora contiene "hola, mundo"
```

#### Interacción de Variables y Datos con Move

Cuando se asigna un valor de un tipo que no implementa `Copy`, la propiedad se mueve.

Ejemplo:
```rust
let s1 = String::from("hola");
let s2 = s1; // s1 se mueve a s2
// println!("{}", s1); // Esto causaría un error
```

#### Interacción de Variables y Datos con Clone

`Clone` crea una copia profunda de los datos.

Ejemplo:
```rust
let s1 = String::from("hola");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2); // Ambas son válidas
```

#### Stack-Only Data: Copy

Los tipos que implementan `Copy` se copian automáticamente en lugar de moverse.

Ejemplo:
```rust
let x = 5;
let y = x; // x sigue siendo válida porque i32 implementa Copy
println!("x = {}, y = {}", x, y);
```

#### La Propiedad y las Funciones

Las funciones pueden tomar posesión de los valores pasados como argumentos.

Ejemplo:
```rust
fn tomar_propiedad(s: String) {
    println!("{}", s);
} // s se descarta aquí

let s = String::from("hola");
tomar_propiedad(s); // s se mueve a la función
// println!("{}", s); // Esto causaría un error
```

#### Valores devueltos y Scope (Ámbito)

Las funciones pueden transferir la propiedad de los valores que retornan.

Ejemplo:
```rust
fn crear_string() -> String {
    String::from("hola")
}

let s = crear_string(); // s toma posesión del valor retornado
```

### Memoria y asignación (Allocation)

Rust maneja automáticamente la asignación y liberación de memoria.

Ejemplo:
```rust
{
    let s = String::from("hola"); // Memoria asignada aquí
    // usar s
} // Memoria liberada automáticamente aquí
```

### Referencia y préstamo

Las referencias permiten usar valores sin tomar posesión.

Ejemplo:
```rust
fn longitud(s: &String) -> usize {
    s.len()
}

let s = String::from("hola");
let len = longitud(&s); // Préstamo de s
println!("La longitud de '{}' es {}.", s, len);
```

#### Referencias Mutables

Permiten modificar un valor prestado.

Ejemplo:
```rust
fn agregar_mundo(s: &mut String) {
    s.push_str(", mundo");
}

let mut s = String::from("hola");
agregar_mundo(&mut s);
println!("{}", s); // Imprime "hola, mundo"
```

####    Referencias Colgadas (Dangling References)

Rust previene las referencias colgadas en tiempo de compilación.

Ejemplo (que no compilará):
```rust
fn dangle() -> &String { // Error: esta función intenta devolver una referencia a un valor que será liberado
    let s = String::from("hola");
    &s
} // s se libera aquí, por lo que la referencia sería inválida
```

#### Las Reglas de las Referencias

1. En cualquier momento, puedes tener una referencia mutable O cualquier número de referencias inmutables.
2. Las referencias deben ser siempre válidas.

### El tipo Slice

Los slices son referencias a una secuencia contigua de elementos en una colección.

#### · String Slices: referencias a partes de la cadena de caracteres

Ejemplo:
```rust
let s = String::from("hola mundo");
let hola = &s[0..4];
let mundo = &s[5..10];
println!("{} {}", hola, mundo);
```

#### String Literals as Slices

Los literales de cadena son slices.

Ejemplo:
```rust
let s: &str = "Hola, mundo!";
```

#### String Slices as Parameters

Usar slices como parámetros permite mayor flexibilidad.

Ejemplo:
```rust
fn primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

let mi_string = String::from("hola mundo");
let palabra = primera_palabra(&mi_string[..]);
```

#### Otros Slices

Los slices pueden ser de otros tipos además de String.

Ejemplo:
```rust
let numeros = [1, 2, 3, 4, 5];
let slice = &numeros[1..3];
assert_eq!(slice, &[2, 3]);
```

Estos ejemplos ilustran cómo Rust maneja la memoria y los datos de manera segura y eficiente a través de su sistema de propiedad y préstamo.