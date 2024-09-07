
# Guía Completa  Rust Parte1

## Tabla de Contenidos
1. [Tipos de Datos Escalares](#tipos-de-datos-escalares)
2. [Char y String](#char-y-string)
3. [Expresiones](#expresiones)
4. [Tipos de Datos Decimales](#tipos-de-datos-decimales)
5. [BigInt](#bigint)
6. [Operadores Lógicos y Relacionales](#operadores-lógicos-y-relacionales)
7. [Operadores Bitwise](#operadores-bitwise)
8. [Tratamiento del Overflow](#tratamiento-del-overflow)
9. [Control de Flujo](#control-de-flujo)
10. [Arrays, Vectores, Enums y Tuplas](#arrays-vectores-enums-y-tuplas)
11. [Slices](#slices)
12. [Funciones y Closures](#funciones-y-closures)
13. [Option y Result](#option-y-result)
14. [Ownership en Rust](#ownership-en-rust)
15. [El Tipo String](#el-tipo-string)
16. [Uso de Chrono para Gestión de Date y Time](#uso-de-chrono-para-gestión-de-date-y-time)
17. [Genéricos](#genéricos)
18. [Vectores de Estructuras](#vectores-de-estructuras)
19. [HashMap](#hashmap)
20. [Parse String to Number or Dates](#parse-string-to-number)
21. [Serialización y Deserialización con Serde](#serialización-y-deserialización-con-serde)
22. [Iteradores](#iteradores)
23. [Itertools](#itertools)
24. [Operaciones con Iteradores](#operaciones-con-iteradores)

Aquí tienes una breve explicación con un snippet para cada tema:

1. Tipos de Datos Escalares
```rust
let entero: i32 = 42;
let flotante: f64 = 3.14;
let booleano: bool = true;
```
Rust tiene tipos de datos básicos como enteros, flotantes y booleanos.

2. Char y String
```rust
let caracter: char = 'a';
let cadena: String = String::from("Hola");
```
`char` es un solo carácter Unicode, `String` es una cadena de texto.

3. Expresiones
```rust
let x = {
    let y = 5;
    y + 1
};
```
En Rust, los bloques de código pueden ser expresiones que devuelven un valor.

4. Tipos de Datos Decimales
```rust
use rust_decimal::Decimal;
let precio = Decimal::new(1025, 2); // 10.25
```
Para cálculos precisos, se usa la crate `rust_decimal`.

5. BigInt
```rust
use num_bigint::BigInt;
let grande = BigInt::parse_bytes(b"123456789012345678901234567890", 10).unwrap();
```
Para números enteros de tamaño arbitrario, se usa `num_bigint`.

6. Operadores Lógicos y Relacionales
```rust
let a = 5;
let b = 10;
let y = a < b && b > 0;
```
Rust soporta operadores lógicos estándar como `&&`, `||`, y `!`.

7. Operadores Bitwise
```rust
let a = 0b1010;
let b = 0b1100;
let and = a & b;
```
Operaciones a nivel de bits como AND (`&`), OR (`|`), XOR (`^`), etc.

8. Tratamiento del Overflow
```rust
let (result, overflowed) = 255u8.overflowing_add(1);
```
Rust proporciona métodos para manejar el desbordamiento de forma segura.

9. Control de Flujo
```rust
if x > 0 {
    println!("positivo");
} else {
    println!("no positivo");
}
```
Estructuras de control como `if`, `else`, `loop`, `while`, y `for`.

10. Arrays, Vectores, Enums y Tuplas
```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
enum Color { Red, Green, Blue }
let tup: (i32, f64, u8) = (500, 6.4, 1);
```
Diferentes estructuras de datos en Rust.

11. Slices
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```
Slices son referencias a una secuencia contigua de elementos en una colección.

12. Funciones y Closures
```rust
fn suma(a: i32, b: i32) -> i32 { a + b }
let closure = |x: i32| x + 1;
```
Definición de funciones y closures (funciones anónimas).

13. Option y Result
```rust
let x: Option<i32> = Some(5);
fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err(String::from("División por cero")) } else { Ok(a / b) }
}
```
Tipos para manejar valores opcionales y resultados que pueden fallar.

14. Ownership en Rust
```rust
let s1 = String::from("hola");
let s2 = s1; // s1 ya no es válida aquí
```
Sistema de propiedad de Rust para gestionar la memoria de forma segura.

15. El Tipo String
```rust
let mut s = String::from("hola");
s.push_str(", mundo!");
```
`String` es un tipo para texto mutable y de tamaño variable.

16. Uso de Chrono para Gestión de Date y Time
```rust
use chrono::{DateTime, Utc};
let ahora: DateTime<Utc> = Utc::now();
```
La crate `chrono` se usa para manejar fechas y tiempos.

17. Genéricos
```rust
fn imprimir<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}
```
Los genéricos permiten escribir código que funciona con múltiples tipos.

18. Vectores de Estructuras
```rust
struct Persona { nombre: String, edad: u32 }
let personas: Vec<Persona> = vec![
    Persona { nombre: String::from("Ana"), edad: 30 },
    Persona { nombre: String::from("Bob"), edad: 25 },
];
```
Vectores pueden contener estructuras personalizadas.

19. HashMap
```rust
use std::collections::HashMap;
let mut mapa = HashMap::new();
mapa.insert(String::from("azul"), 10);
```
HashMaps son colecciones de pares clave-valor.

20. Parse String to Number
```rust
let numero: i32 = "42".parse().unwrap();
```
Conversión de strings a números.

21. Serialización y Deserialización con Serde
```rust
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Persona { nombre: String, edad: u32 }
```
Serde facilita la conversión entre estructuras de datos y formatos como JSON.

22. Iteradores
```rust
let numeros = vec![1, 2, 3, 4, 5];
let cuadrados: Vec<i32> = numeros.iter().map(|&x| x * x).collect();
```
Los iteradores permiten procesar secuencias de elementos de forma eficiente.

23. Itertools
```rust
use itertools::Itertools;
let grupos = vec![1, 2, 3, 4, 5].into_iter().group_by(|&x| x % 2 == 0);
```
La crate `itertools` proporciona funciones adicionales para trabajar con iteradores.

24. Operaciones con Iteradores
```rust
let resultado: Vec<i32> = vec![1, 2, 3, 4, 5].iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 2)
    .collect();
```
Los iteradores pueden encadenarse para realizar operaciones complejas de forma declarativa.