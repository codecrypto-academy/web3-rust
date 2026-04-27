# Genéricos

## Introducion

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


3. Enum Genérico

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

4. Genéricos con Restricciones de Trait

```rust
// Una función que suma todos los elementos de un vector
fn main() {
    fn suma<T: std::ops::Add<Output = T> + Copy + Default>(v: &Vec<T>) -> T {
        v.iter().fold(T::default(), |acc, x| acc + *x)
    }
    let numbers = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    println!("La suma de los números es: {}", suma(&numbers));
}
```

5. Uso de Where Clause

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

