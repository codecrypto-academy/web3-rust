# Guía Completa de Conceptos en Rust

## Tabla de Contenidos
1. [Tipos de Datos Básicos](#tipos-de-datos-básicos)
2. [Char y String](#char-y-string)
3. [Expresiones](#expresiones)
4. [Tipos de Datos Decimales](#tipos-de-datos-decimales)
5. [BigInt](#bigint)
6. [Operadores Lógicos y Relacionales](#operadores-lógicos-y-relacionales)
7. [Operadores Bitwise](#operadores-bitwise)
8. [Tratamiento del Overflow](#tratamiento-del-overflow)
9. [Control de Flujo](#control-de-flujo)
10. [Arrays, Vectores, Enums y Tuplas](#arrays-vectores-enums-y-tuplas)
11. [Option y Result](#option-y-result)
12. [Ownership en Rust](#ownership-en-rust)
13. [El Tipo String](#el-tipo-string)
14. [Uso de Chrono para Gestión de Date y Time](#uso-de-chrono-para-gestión-de-date-y-time)
15. [Genéricos](#genéricos)
16. [Vectores de Estructuras](#vectores-de-estructuras)
17. [HashMap](#hashmap)
18. [Parse String to Number](#parse-string-to-number)
19. [Serialización y Deserialización con Serde](#serialización-y-deserialización-con-serde)
20. [Iteradores](#iteradores)

## Tipos de Datos Básicos

Rust tiene varios tipos de datos primitivos:

- Enteros: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- Enteros sin signo: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- Punto flotante: `f32`, `f64`
- Booleano: `bool`
- Carácter: `char`

```rust
let entero: i32 = 42;
let flotante: f64 = 3.14;
let booleano: bool = true;
```

## Char y String

`char` representa un solo carácter Unicode, mientras que `String` es una cadena de texto UTF-8.

```rust
let caracter: char = 'a';
let cadena: String = String::from("Hola, mundo!");
```

## Expresiones

En Rust, casi todo es una expresión:

```rust
let x = {
    let y = 5;
    y + 1
};
```

## Tipos de Datos Decimales

Para cálculos precisos, se puede usar la crate `rust_decimal`:

```rust
use rust_decimal::Decimal;

let precio = Decimal::new(1025, 2); // 10.25
```

## BigInt

Para números enteros de tamaño arbitrario, se usa la crate `num-bigint`:

```rust
use num_bigint::BigInt;

let grande = BigInt::parse_bytes(b"123456789012345678901234567890", 10).unwrap();
```

## Operadores Lógicos y Relacionales

```rust
let a = 5;
let b = 10;

let y = a < b && b > 0;
let o = a > b || b < 20;
let no = !true;
```

## Operadores Bitwise

```rust
let a = 0b1010;
let b = 0b1100;

let and = a & b;
let or = a | b;
let xor = a ^ b;
let shift_left = a << 1;
let shift_right = b >> 1;
```

## Tratamiento del Overflow

Rust proporciona métodos para manejar el overflow:

```rust
let (result, overflowed) = 255u8.overflowing_add(1);
let saturated = 255u8.saturating_add(1);
let wrapped = 255u8.wrapping_add(1);
```

## Control de Flujo

```rust
// if-else
if x > 0 {
    println!("positivo");
} else {
    println!("no positivo");
}

// loop
loop {
    if condition {
        break;
    }
}

// while
while condition {
    // código
}

// for
for i in 0..10 {
    println!("{}", i);
}
```

## Arrays, Vectores, Enums y Tuplas

```rust
// Array
let arr: [i32; 5] = [1, 2, 3, 4, 5];

// Vector
let vec: Vec<i32> = vec![1, 2, 3, 4, 5];

// Enum
enum Color {
    Red,
    Green,
    Blue,
}

// Tupla
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

## Option y Result

```rust
// Option
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

// Result
fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("División por cero"))
    } else {
        Ok(a / b)
    }
}
```

## Ownership en Rust

```rust
let s1 = String::from("hola");
let s2 = s1; // s1 ya no es válida aquí
```

## El Tipo String

```rust
let mut s = String::from("hola");
s.push_str(", mundo!");
```

## Uso de Chrono para Gestión de Date y Time

```rust
use chrono::{DateTime, Utc};

let ahora: DateTime<Utc> = Utc::now();
```

## Genéricos

```rust
fn imprimir<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}
```

## Vectores de Estructuras

```rust
struct Persona {
    nombre: String,
    edad: u32,
}

let personas: Vec<Persona> = vec![
    Persona { nombre: String::from("Ana"), edad: 30 },
    Persona { nombre: String::from("Bob"), edad: 25 },
];
```

## HashMap

```rust
use std::collections::HashMap;

let mut mapa = HashMap::new();
mapa.insert(String::from("azul"), 10);
mapa.insert(String::from("rojo"), 50);
```

## Parse String to Number

```rust
let numero: i32 = "42".parse().unwrap();
```

## Serialización y Deserialización con Serde

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Persona {
    nombre: String,
    edad: u32,
}

let json = serde_json::to_string(&persona).unwrap();
let persona: Persona = serde_json::from_str(&json).unwrap();
```

## Iteradores

```rust
let numeros = vec![1, 2, 3, 4, 5];

// map
let cuadrados: Vec<i32> = numeros.iter().map(|&x| x * x).collect();

// filter
let pares: Vec<&i32> = numeros.iter().filter(|&&x| x % 2 == 0).collect();

// fold (reduce)
let suma: i32 = numeros.iter().fold(0, |acc, &x| acc + x);

// group_by
use itertools::Itertools;
let grupos = numeros.into_iter().group_by(|&x| x % 2 == 0);

// join
let joined: String = numeros.iter().map(|&x| x.to_string()).join(", ");

// Operaciones complejas
let resultado: Vec<i32> = numeros.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 2)
    .collect();
```

Este documento proporciona una visión general de los conceptos solicitados en Rust. Cada sección puede expandirse con más ejemplos y explicaciones detalladas según sea necesario.