# Slices en Rust

Los slices son una característica fundamental en Rust que permiten referenciar una secuencia contigua de elementos en una colección sin tomar posesión de ellos. Son útiles para trabajar con porciones de arrays, vectores o cadenas de texto.

## Características principales

- Los slices son referencias inmutables por defecto.
- Se representan con el tipo `&[T]` para slices de tipo T.
- Para cadenas de texto, se usa el tipo `&str`.
- No tienen propiedad de los datos que referencian.

## Sintaxis básica

```rust
let slice = &colección[inicio..fin];
```

Donde:
- `inicio` es el índice donde comienza el slice (inclusive).
- `fin` es el índice donde termina el slice (exclusive).

## Ejemplos de uso

### Slices de arrays

```rust
fn main() {
    let números = [1, 2, 3, 4, 5];
    
    let slice = &números[1..4];
    
    println!("Slice: {:?}", slice); // Imprime: Slice: [2, 3, 4]
}
```

### Slices de vectores

```rust
fn main() {
    let vec = vec![10, 20, 30, 40, 50];
    
    let slice = &vec[2..];
    
    println!("Slice: {:?}", slice); // Imprime: Slice: [30, 40, 50]
}
```

### Slices de strings

```rust
fn main() {
    let s = String::from("Hola, mundo!");
    
    let hola = &s[0..4];
    let mundo = &s[6..11];
    
    println!("{} {}", hola, mundo); // Imprime: Hola mundo
}
```

## Slices como parámetros de función

Los slices son útiles como parámetros de función cuando no necesitas poseer los datos:

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

fn main() {
    let mi_string = String::from("Hola mundo");
    let palabra = primera_palabra(&mi_string);
    println!("La primera palabra es: {}", palabra); // Imprime: La primera palabra es: Hola
}
```

## Slices mutables

También puedes crear slices mutables:

```rust
fn main() {
    let mut números = [1, 2, 3, 4, 5];
    
    let slice = &mut números[1..4];
    
    slice[0] = 20;
    
    println!("Números modificados: {:?}", números); // Imprime: Números modificados: [1, 20, 3, 4, 5]
}
```

## Consideraciones importantes

1. Los slices deben ser válidos durante toda su vida útil.
2. No puedes tener slices mutables e inmutables de la misma colección simultáneamente.
3. Los índices de los slices se comprueban en tiempo de ejecución para evitar accesos fuera de límites.

Los slices son una herramienta poderosa en Rust que permite trabajar eficientemente con partes de colecciones sin comprometer la seguridad y sin la necesidad de copiar datos innecesariamente.