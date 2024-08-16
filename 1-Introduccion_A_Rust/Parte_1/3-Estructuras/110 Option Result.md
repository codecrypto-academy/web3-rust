# Ejemplos de Option y Result en Rust

## Option

`Option` es un enum que representa la presencia o ausencia de un valor.

### Definición básica
```rust
enum Option<T> {
    Some(T),
    None,
}
```

### Ejemplos de uso de Option

1. **Valor opcional en una estructura**
```rust
struct Usuario {
    nombre: String,
    edad: Option<u8>,
}

let usuario = Usuario {
    nombre: String::from("Alice"),
    edad: Some(30),
};

match usuario.edad {
    Some(edad) => println!("La edad es {}", edad),
    None => println!("La edad no está especificada"),
}
```

2. **Método que puede fallar**
```rust
fn dividir(numerador: f64, denominador: f64) -> Option<f64> {
    if denominador == 0.0 {
        None
    } else {
        Some(numerador / denominador)
    }
}

let resultado = dividir(10.0, 2.0);
if let Some(valor) = resultado {
    println!("El resultado es {}", valor);
} else {
    println!("No se pudo realizar la división");
}
```

3. **Uso de map con Option**
```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = x.map(|n| n * 2);
println!("y = {:?}", y);  // Imprime: y = Some(10)
```

4. **Encadenamiento de métodos con Option**
```rust
let nombre: Option<String> = Some(String::from("Alice"));
let longitud = nombre.as_ref().map(|s| s.len());
println!("Longitud: {:?}", longitud);  // Imprime: Longitud: Some(5)
```

5. **Uso de unwrap_or**
```rust
let x: Option<i32> = None;
let y = x.unwrap_or(0);
println!("y = {}", y);  // Imprime: y = 0
```

## Result

`Result` es un enum que representa el éxito (`Ok`) o el fracaso (`Err`) de una operación.

### Definición básica
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Ejemplos de uso de Result

1. **Función que puede fallar**
```rust
use std::fs::File;
use std::io::Error;

fn abrir_archivo(ruta: &str) -> Result<File, Error> {
    File::open(ruta)
}

match abrir_archivo("archivo.txt") {
    Ok(archivo) => println!("Archivo abierto con éxito"),
    Err(error) => println!("Error al abrir el archivo: {:?}", error),
}
```

2. **Propagación de errores con ?**
```rust
use std::fs::File;
use std::io::{self, Read};

fn leer_nombre_usuario(ruta: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(ruta)?.read_to_string(&mut s)?;
    Ok(s.trim().to_string())
}

match leer_nombre_usuario("usuario.txt") {
    Ok(nombre) => println!("Nombre de usuario: {}", nombre),
    Err(e) => println!("Error: {}", e),
}
```

3. **Conversión entre Result y Option**
```rust
fn dividir(numerador: f64, denominador: f64) -> Result<f64, String> {
    if denominador == 0.0 {
        Err(String::from("División por cero"))
    } else {
        Ok(numerador / denominador)
    }
}

let resultado = dividir(10.0, 2.0).ok();
println!("Resultado: {:?}", resultado);  // Imprime: Resultado: Some(5.0)
```

4. **Uso de map_err para transformar el error**
```rust
fn parse_edad(input: &str) -> Result<u8, String> {
    input.parse::<u8>().map_err(|_| String::from("Edad inválida"))
}

let edad = parse_edad("30");
println!("Edad parseada: {:?}", edad);  // Imprime: Edad parseada: Ok(30)
```

5. **Combinación de múltiples Results**
```rust
fn operacion_compuesta() -> Result<i32, String> {
    let x = "5".parse::<i32>().map_err(|e| e.to_string())?;
    let y = "10".parse::<i32>().map_err(|e| e.to_string())?;
    Ok(x + y)
}

match operacion_compuesta() {
    Ok(resultado) => println!("El resultado es {}", resultado),
    Err(e) => println!("Error: {}", e),
}
```

Estos ejemplos muestran cómo `Option` y `Result` se utilizan en Rust para manejar casos de valores opcionales y operaciones que pueden fallar, respectivamente. Estas estructuras son fundamentales en Rust para escribir código robusto y manejar errores de manera elegante y segura.