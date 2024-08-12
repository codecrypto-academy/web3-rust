# Ejemplos de Uso de String en Rust

## Creación y Manipulación Básica

```rust
fn main() {
    // Creación de String
    let mut s = String::from("Hola, mundo!");
    println!("Original: {}", s);

    // Añadir contenido
    s.push_str(" Bienvenido a Rust.");
    println!("Después de push_str: {}", s);

    // Reemplazar
    let s2 = s.replace("mundo", "Rust");
    println!("Después de replace: {}", s2);

    // Concatenación
    let s3 = s + " ¡Aprendamos más!";
    println!("Después de concatenar: {}", s3);
}
```

## Split y Trim

```rust
fn main() {
    let s = String::from("  Hola,mundo,Rust  ");

    // Split
    let partes: Vec<&str> = s.split(',').collect();
    println!("Partes: {:?}", partes);

    // Trim
    let trimmed = s.trim();
    println!("Trimmed: '{}'", trimmed);

    // Combinación de split y trim
    let palabras: Vec<&str> = s.split(',')
                               .map(|s| s.trim())
                               .collect();
    println!("Palabras trimmed: {:?}", palabras);
}
```

## Find y Subcadenas

```rust
fn main() {
    let s = String::from("Rust es genial");

    // Find
    if let Some(pos) = s.find("es") {
        println!("'es' encontrado en la posición: {}", pos);
    }

    // Subcadenas
    let sub = &s[5..7];
    println!("Subcadena: {}", sub);
}
```

## Formato con format!

```rust
fn main() {
    let nombre = "Alice";
    let edad = 30;

    let s = format!("Hola, me llamo {} y tengo {} años", nombre, edad);
    println!("{}", s);

    // Formato con especificadores
    let pi = 3.14159;
    let formatted = format!("Pi con 2 decimales: {:.2}", pi);
    println!("{}", formatted);
}
```

## Uso de Expresiones Regulares

Para usar expresiones regulares, necesitas añadir la dependencia `regex` a tu `Cargo.toml`:

```toml
[dependencies]
regex = "1.5"
```

Luego, puedes usar expresiones regulares así:

```rust
use regex::Regex;

fn main() {
    let texto = "El año es 2023 y el mes es 12";

    // Buscar un patrón
    let re = Regex::new(r"\d{4}").unwrap();
    match re.find(texto) {
        Some(coincidencia) => println!("Año encontrado: {}", coincidencia.as_str()),
        None => println!("No se encontró el año"),
    }

    // Reemplazar usando regex
    let re = Regex::new(r"\d+").unwrap();
    let resultado = re.replace_all(texto, "NÚMERO");
    println!("Texto con números reemplazados: {}", resultado);

    // Capturar grupos
    let re = Regex::new(r"(\d{4}).*?(\d{2})").unwrap();
    for cap in re.captures_iter(texto) {
        println!("Año: {}, Mes: {}", &cap[1], &cap[2]);
    }
}
```

## Conversión entre String y Otros Tipos

```rust
fn main() {
    // De número a String
    let num = 42;
    let num_str = num.to_string();
    println!("Número como String: {}", num_str);

    // De String a número
    let str_num = "3.14";
    let parsed_num: f64 = str_num.parse().unwrap();
    println!("String parseado a número: {}", parsed_num);

    // De &str a String
    let s: String = "Hola".into();
    println!("&str convertido a String: {}", s);
}
```

## Iteración sobre Caracteres

```rust
fn main() {
    let s = String::from("Rust🦀");

    // Iterar sobre caracteres
    for c in s.chars() {
        println!("Carácter: {}", c);
    }

    // Contar caracteres
    println!("Número de caracteres: {}", s.chars().count());

    // Contar bytes
    println!("Número de bytes: {}", s.len());
}
```

Estos ejemplos cubren una amplia gama de operaciones comunes con `String` en Rust, incluyendo manipulación básica, formateo, uso de expresiones regulares y conversiones. Recuerda que `String` en Rust es un tipo que posee sus datos y garantiza que siempre contenga UTF-8 válido.