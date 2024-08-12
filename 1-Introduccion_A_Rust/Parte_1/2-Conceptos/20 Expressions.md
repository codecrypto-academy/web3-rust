# Expresiones Numéricas en Rust

## Tipos de Números en Rust

Rust ofrece varios tipos de números para diferentes necesidades:

### Enteros

| Tipo   | Tamaño   | Rango                                                   |
|--------|----------|---------------------------------------------------------|
| `i8`   | 8 bits   | -128 a 127                                              |
| `u8`   | 8 bits   | 0 a 255                                                 |
| `i16`  | 16 bits  | -32,768 a 32,767                                        |
| `u16`  | 16 bits  | 0 a 65,535                                              |
| `i32`  | 32 bits  | -2,147,483,648 a 2,147,483,647                          |
| `u32`  | 32 bits  | 0 a 4,294,967,295                                       |
| `i64`  | 64 bits  | -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807 |
| `u64`  | 64 bits  | 0 a 18,446,744,073,709,551,615                          |
| `i128` | 128 bits | -170,141,183,460,469,231,731,687,303,715,884,105,728 a 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| `u128` | 128 bits | 0 a 340,282,366,920,938,463,463,374,607,431,768,211,455 |
| `isize`| arch     | Depende de la arquitectura (32 o 64 bits)               |
| `usize`| arch     | Depende de la arquitectura (32 o 64 bits)               |

### Punto Flotante

- `f32`: Precisión simple de 32 bits
- `f64`: Precisión doble de 64 bits (por defecto)

## Expresiones Numéricas Básicas

```rust
fn main() {
    // Enteros
    let entero: i32 = 42;
    let suma = 5 + 3;
    let resta = 10 - 5;
    let multiplicacion = 4 * 2;
    let division = 15 / 3;
    let modulo = 17 % 3;

    // Punto flotante
    let flotante: f64 = 3.14;
    let suma_float = 5.0 + 3.2;
    let division_float = 10.0 / 3.0;

    println!("Entero: {}", entero);
    println!("Suma: {}", suma);
    println!("Resta: {}", resta);
    println!("Multiplicación: {}", multiplicacion);
    println!("División: {}", division);
    println!("Módulo: {}", modulo);
    println!("Flotante: {}", flotante);
    println!("Suma float: {}", suma_float);
    println!("División float: {}", division_float);
}
```

## Inferencia de Tipos y Sufijos Numéricos

Rust puede inferir tipos, pero también se pueden especificar explícitamente:

```rust
let a = 5;      // i32 por defecto
let b = 5u8;    // u8
let c = 5_i32;  // i32
let d = 5.0;    // f64 por defecto
let e = 5.0f32; // f32
```

## Conversiones Numéricas

```rust
fn main() {
    let a: i32 = 5;
    let b: i64 = a as i64;
    let c: f64 = b as f64;

    println!("a: {}, b: {}, c: {}", a, b, c);

    // Cuidado con las conversiones que pueden perder datos
    let x: u8 = 300_i32 as u8;  // Resulta en 44 (300 % 256)
    println!("x: {}", x);
}
```

## Operaciones con Diferentes Tipos

Rust requiere tipos iguales para operaciones aritméticas:

```rust
fn main() {
    let a: i32 = 5;
    let b: i64 = 10;
    // let c = a + b;  // Esto daría un error
    let c = a + b as i32;  // Conversión explícita
    println!("c: {}", c);
}
```

## Desbordamiento y Métodos de Operación Seguros

```rust
fn main() {
    let a: u8 = 255;
    // let b = a + 1;  // Esto causaría pánico en modo debug

    // Métodos seguros
    let c = a.wrapping_add(1);  // Envuelve a 0
    let d = a.saturating_add(1);  // Se queda en 255
    let e = a.checked_add(1);  // Devuelve None

    println!("c: {}, d: {}, e: {:?}", c, d, e);
}
```

## Operaciones de Punto Flotante

```rust
fn main() {
    let x = 2.0;
    let y = 3.0;

    println!("Potencia: {}", x.powf(y));
    println!("Raíz cuadrada: {}", x.sqrt());
    println!("Exponencial: {}", x.exp());
    println!("Logaritmo: {}", x.ln());
    println!("Seno: {}", x.sin());
}
```

## Constantes Numéricas

```rust
const PI: f64 = 3.141592653589793;
const AVOGADRO: f64 = 6.022e23;
```

## Consideraciones Importantes

1. **Precisión**: Los tipos de punto flotante pueden tener problemas de precisión en ciertas operaciones.
2. **División por Cero**: La división por cero en enteros causa pánico, mientras que en punto flotante resulta en `inf` o `NaN`.
3. **NaN y Infinito**: Los tipos de punto flotante pueden representar NaN (Not a Number) e infinito.
4. **Rendimiento**: Los tipos más pequeños no siempre son más rápidos; a menudo `i32` y `i64` son óptimos.

Las expresiones numéricas en Rust ofrecen un alto grado de control y seguridad. La elección del tipo correcto y el manejo adecuado de las conversiones son cruciales para escribir código Rust eficiente y libre de errores.
