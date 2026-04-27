# Genéricos 2 parte

## Ejemplos de Genéricos en Rust



1. Estructura Genérica Simple

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

2. Implementación de Métodos en Estructuras Genéricas

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


3. Genéricos en Traits

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

4. Genéricos con Lifetime

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

5. Genéricos en Closures

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
