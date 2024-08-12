# Tuplas, Arrays, vec! y Enums en Rust

## 1. Tuplas

Las tuplas son colecciones de valores de diferentes tipos agrupados en una sola unidad.

### Definición y Uso Básico
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Desestructuración
let (x, y, z) = tup;
println!("y = {}", y);

// Acceso por índice
let quinientos = tup.0;
let uno = tup.2;
```

### Tupla Unitaria
```rust
let unitaria = (42,);
```

### Tuplas en Funciones
```rust
fn coordenadas() -> (f64, f64) {
    (6.3, 15.0)
}

let (x, y) = coordenadas();
```

## 2. Arrays

Los arrays son colecciones de elementos del mismo tipo con una longitud fija conocida en tiempo de compilación.

### Definición y Uso Básico
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Array con valores repetidos
let a = [3; 5];  // Equivalente a [3, 3, 3, 3, 3]

// Acceso a elementos
let primero = a[0];
let segundo = a[1];
```

### Slices
```rust
let slice = &a[1..3];  // Referencia a una parte del array
```

### Arrays en Funciones
```rust
fn suma_array(arr: &[i32; 5]) -> i32 {
    arr.iter().sum()
}
```

## 3. vec!

`vec!` es un macro para crear vectores, que son arrays dinámicos que pueden crecer o encogerse en tiempo de ejecución.

### Creación Básica
```rust
let v = vec![1, 2, 3, 4, 5];

// Vec vacío
let mut v: Vec<i32> = Vec::new();

// Vec con capacidad inicial
let v: Vec<i32> = Vec::with_capacity(10);
```

### Operaciones Comunes
```rust
let mut v = vec![1, 2, 3];

// Añadir elementos
v.push(4);

// Acceder a elementos
let tercero: &i32 = &v[2];

// Iterar
for i in &v {
    println!("{}", i);
}

// Eliminar el último elemento
let ultimo = v.pop();

// Longitud
println!("Longitud: {}", v.len());

// Capacidad
println!("Capacidad: {}", v.capacity());
```

### Vec con Diferentes Tipos
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let fila = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("azul")),
    SpreadsheetCell::Float(10.12),
];
```

## 4. Enums

Los enums permiten definir un tipo enumerando sus posibles variantes.

### Definición Básica
```rust
enum DiaSemana {
    Lunes,
    Martes,
    Miercoles,
    Jueves,
    Viernes,
    Sabado,
    Domingo,
}

let dia = DiaSemana::Miercoles;
```

### Enums con Datos Asociados
```rust
enum Mensaje {
    Salir,
    Mover { x: i32, y: i32 },
    Escribir(String),
    CambiarColor(i32, i32, i32),
}

let m1 = Mensaje::Mover { x: 10, y: 20 };
let m2 = Mensaje::Escribir(String::from("hola"));
```


### Option Enum
```rust
let algunos_numeros = Some(5);
let ninguna_cadena: Option<String> = None;
```

### Uso de match con Enums
```rust
enum Moneda {
    Centavo,
    Cinco,
    Diez,
    Veinticinco,
}

fn valor_en_centavos(moneda: Moneda) -> u8 {
    match moneda {
        Moneda::Centavo => 1,
        Moneda::Cinco => 5,
        Moneda::Diez => 10,
        Moneda::Veinticinco => 25,
    }
}
```

## Características Avanzadas y Consejos

1. **Tuplas Estructuradas**: Útiles para retornar múltiples valores de una función.

2. **Arrays Multidimensionales**:
   ```rust
   let matriz: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
   ```

3. **Vec y Ownership**:
   ```rust
   let v = vec![1, 2, 3];
   let primero = &v[0];  // Referencia inmutable
   v.push(4);  // Error: no se puede mutar mientras hay una referencia inmutable
   ```

4. **Enums para Máquinas de Estado**: Excelentes para modelar estados y transiciones.

5. **Pattern Matching con Enums**:
   ```rust
   match mensaje {
       Mensaje::Salir => println!("Saliendo..."),
       Mensaje::Mover { x, y } => println!("Moviendo a ({}, {})", x, y),
       Mensaje::Escribir(texto) => println!("Texto: {}", texto),
       Mensaje::CambiarColor(r, g, b) => println!("Color: RGB({},{},{})", r, g, b),
   }
   ```

6. **Uso de `if let` con Enums**:
   ```rust
   if let Some(x) = opcional {
       println!("El valor es: {}", x);
   }
   ```

Estas estructuras de datos son fundamentales en Rust y proporcionan formas poderosas y seguras de organizar y manipular datos. Las tuplas ofrecen flexibilidad para agrupar datos heterogéneos, los arrays proporcionan eficiencia para colecciones homogéneas de tamaño fijo, `vec!` ofrece flexibilidad para colecciones dinámicas, y los enums permiten modelar datos que pueden ser uno de varios tipos o estados diferentes. Juntos, estos tipos de datos forman una base sólida para la programación robusta y expresiva en Rust.