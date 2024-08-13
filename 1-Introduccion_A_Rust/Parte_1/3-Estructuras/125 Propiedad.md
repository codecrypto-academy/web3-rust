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