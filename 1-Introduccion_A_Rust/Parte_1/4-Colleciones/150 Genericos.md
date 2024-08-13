# Genéricos

## Ejemplos de Genéricos en Rust

1. Función Genérica Básica

```rust
// Una función genérica que puede trabajar con cualquier tipo que implemente Debug
fn imprimir<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}

fn main() {
    imprimir(5);
    imprimir("Hola");
    imprimir(vec![1, 2, 3]);
}
```

2. Función Genérica con Múltiples Tipos

```rust
// Una función que compara dos valores de cualquier tipo que sea comparable
fn max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("Máximo de 10 y 5: {}", max(10, 5));
    println!("Máximo de 3.14 y 2.71: {}", max(3.14, 2.71));
}
```

3. Estructura Genérica Simple

```rust
// Una estructura que puede contener dos valores de tipos diferentes
struct Par<T, U> {
    primero: T,
    segundo: U,
}

fn main() {
    let par_mixto = Par { primero: 1, segundo: "dos" };
    println!("Par: ({}, {})", par_mixto.primero, par_mixto.segundo);
}
```

4. Implementación de Métodos en Estructuras Genéricas

```rust
struct Contenedor<T> {
    contenido: T,
}

impl<T> Contenedor<T> {
    fn nuevo(contenido: T) -> Contenedor<T> {
        Contenedor { contenido }
    }

    fn obtener(&self) -> &T {
        &self.contenido
    }
}

fn main() {
    let contenedor = Contenedor::nuevo(42);
    println!("Contenido: {}", contenedor.obtener());
}
```

5. Enum Genérico

```rust
enum Resultado<T, E> {
    Ok(T),
    Err(E),
}

fn dividir(numerador: f64, denominador: f64) -> Resultado<f64, String> {
    if denominador == 0.0 {
        Resultado::Err(String::from("División por cero"))
    } else {
        Resultado::Ok(numerador / denominador)
    }
}

fn main() {
    match dividir(10.0, 2.0) {
        Resultado::Ok(valor) => println!("Resultado: {}", valor),
        Resultado::Err(error) => println!("Error: {}", error),
    }
}
```

6. Genéricos con Restricciones de Trait

```rust
// Una función que suma todos los elementos de un vector
fn suma_vector<T: std::ops::Add<Output = T> + Copy>(v: &Vec<T>) -> T {
    v.iter().fold(std::mem::zeroed(), |acc, &x| acc + x)
}

fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    println!("Suma: {}", suma_vector(&numeros));
}
```

7. Uso de Where Clause

```rust
fn procesar<T, U>(t: T, u: U) -> i32
where
    T: std::fmt::Debug,
    U: std::fmt::Display,
{
    println!("T: {:?}, U: {}", t, u);
    42
}

fn main() {
    procesar(vec![1, 2, 3], "Hola");
}
```

8. Genéricos en Traits

```rust
trait Convertible<T> {
    fn convertir(&self) -> T;
}

impl Convertible<f64> for i32 {
    fn convertir(&self) -> f64 {
        *self as f64
    }
}

fn main() {
    let numero = 42;
    println!("Convertido: {}", numero.convertir());
}
```

9. Genéricos con Lifetime

```rust
// Una función que devuelve la referencia más larga
fn referencia_mas_larga<'a, T>(x: &'a T, y: &'a T) -> &'a T {
    if std::mem::size_of_val(x) > std::mem::size_of_val(y) { x } else { y }
}

fn main() {
    let corta = String::from("xy");
    let larga = String::from("larga");
    println!("Referencia más larga: {}", referencia_mas_larga(&corta, &larga));
}
```

10. Genéricos en Closures

```rust
fn aplicar_y_sumar<T, F>(inicial: T, cantidad: usize, f: F) -> T
where
    T: std::ops::Add<Output = T> + Copy,
    F: Fn(T) -> T,
{
    (0..cantidad).fold(inicial, |acc, _| acc + f(inicial))
}

fn main() {
    let resultado = aplicar_y_sumar(1, 5, |x| x * 2);
    println!("Resultado: {}", resultado);
}
```

Estos ejemplos cubren una amplia gama de usos de genéricos en Rust, desde conceptos básicos hasta aplicaciones más avanzadas. Los genéricos permiten escribir código flexible y reutilizable, manteniendo la seguridad de tipos y el rendimiento que caracteriza a Rust.