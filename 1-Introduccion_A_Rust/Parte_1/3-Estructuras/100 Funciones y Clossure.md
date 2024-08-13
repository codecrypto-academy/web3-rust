# Funciones y Closures en Rust

## Funciones

Las funciones son bloques de código reutilizables que realizan una tarea específica.

### Definición Básica
```rust
fn saludar(nombre: &str) {
    println!("Hola, {}!", nombre);
}

fn main() {
    saludar("Alice");
}
```

### Funciones con Retorno
```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b  // La última expresión se retorna implícitamente
}

fn main() {
    let resultado = sumar(5, 3);
    println!("La suma es: {}", resultado);
}
```

### Retorno Anticipado
```rust
fn es_par(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    false
}
```

### Funciones como Parámetros
```rust
fn aplicar_funcion<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn main() {
    let resultado = aplicar_funcion(|x| x * x, 5);
    println!("El resultado es: {}", resultado);
}
```

## Closures

Los closures son funciones anónimas que pueden capturar su entorno.

### Sintaxis Básica
```rust
let suma = |a, b| a + b;
println!("3 + 4 = {}", suma(3, 4));
```

### Captura de Variables
```rust
let x = 4;
let igual_a_x = |z| z == x;
println!("{}", igual_a_x(4));
```

### Tipos de Captura
- `Fn`: Captura por referencia
- `FnMut`: Captura por referencia mutable
- `FnOnce`: Captura por valor

```rust
fn consumir<F: FnOnce()>(f: F) {
    f();
}

let x = String::from("hola");
consumir(|| println!("{}", x));
```

### Closures como Parámetros
```rust
fn ejecutar<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let saludo = || println!("Hola, mundo!");
    ejecutar(saludo);
}
```

### Closures que Mutan su Entorno
```rust
let mut contador = 0;
let mut incrementar = || {
    contador += 1;
    println!("Contador: {}", contador);
};

incrementar();
incrementar();
```

### Retornando Closures
```rust
fn crear_incrementador(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

let incrementa_por_5 = crear_incrementador(5);
println!("10 + 5 = {}", incrementa_por_5(10));
```

## Características Avanzadas y Consejos

1. **Funciones Genéricas**:
   ```rust
   fn imprimir<T: std::fmt::Display>(valor: T) {
       println!("El valor es: {}", valor);
   }
   ```

2. **Métodos asociados**:
   ```rust
   struct Rectangulo {
       ancho: u32,
       alto: u32,
   }

   impl Rectangulo {
       fn area(&self) -> u32 {
           self.ancho * self.alto
       }
   }
   ```

3. **Funciones de orden superior**:
   ```rust
   fn map<F, T, U>(vector: Vec<T>, f: F) -> Vec<U>
   where
       F: Fn(T) -> U,
   {
       vector.into_iter().map(f).collect()
   }
   ```

4. **Closures y move**:
   ```rust
   let x = vec![1, 2, 3];
   let igual_a_x = move |z| z == x;
   // x ya no es accesible aquí
   ```

5. **Funciones variádicas con macros**:
   ```rust
   macro_rules! print_todos {
       ($($arg:expr),*) => {
           $(
               println!("{}", $arg);
           )*
       };
   }

   print_todos!(1, "dos", 3.0);
   ```

6. **Closures en iteradores**:
   ```rust
   let numeros = vec![1, 2, 3, 4, 5];
   let pares: Vec<_> = numeros.into_iter().filter(|&x| x % 2 == 0).collect();
   ```

7. **Funciones unsafe**:
   ```rust
   unsafe fn peligrosa() {
       // Código que requiere garantías del programador
   }
   ```

Las funciones y closures en Rust proporcionan poderosas herramientas para estructurar y abstraer código. Las funciones ofrecen una forma de organizar código en bloques reutilizables, mientras que los closures permiten crear funciones anónimas que pueden capturar su entorno. Juntos, estos conceptos permiten escribir código Rust expresivo, modular y eficiente.

La capacidad de Rust para tratar las funciones como ciudadanos de primera clase (high order functions), combinada con su sistema de tipos fuerte y sus garantías de seguridad, hace que trabajar con funciones y closures sea seguro y flexible. Esto es particularmente útil en programación funcional y en situaciones donde se necesita pasar comportamiento como argumento.