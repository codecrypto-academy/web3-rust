# Ejemplos de Uso de Serde. Serialización y Deserialización

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.1"
```

Ahora, veamos los ejemplos:

1. Crear JSON desde una estructura

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Persona {
    nombre: String,
    edad: u32,
}

fn main() {
    let persona = Persona {
        nombre: String::from("Alice"),
        edad: 30,
    };

    // Serializar a JSON
    let json = serde_json::to_string(&persona).unwrap();
    println!("JSON: {}", json);
}
```

2. Crear CSV desde una estructura

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Registro {
    id: u32,
    nombre: String,
    activo: bool,
}

fn main() {
    let registros = vec![
        Registro { id: 1, nombre: String::from("Alice"), activo: true },
        Registro { id: 2, nombre: String::from("Bob"), activo: false },
    ];

    let mut wtr = csv::Writer::from_writer(vec![]);
    for registro in registros {
        wtr.serialize(registro).unwrap();
    }

    let csv_data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    println!("CSV:\n{}", csv_data);
}
```

3. Convertir JSON a estructura (Deserialización)

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Persona {
    nombre: String,
    edad: u32,
}

fn main() {
    let json_str = r#"
    {
        "nombre": "Bob",
        "edad": 25
    }"#;

    let persona: Persona = serde_json::from_str(json_str).unwrap();
    println!("Persona deserializada: {:?}", persona);
}
```

4. Leer un fichero CSV

```rust
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Registro {
    id: u32,
    nombre: String,
    activo: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("datos.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Registro = result?;
        println!("{:?}", record);
    }

    Ok(())
}
```

5. Trabajar con estructuras anidadas en JSON

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Direccion {
    calle: String,
    ciudad: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Persona {
    nombre: String,
    edad: u32,
    direccion: Direccion,
}

fn main() {
    let persona = Persona {
        nombre: String::from("Charlie"),
        edad: 35,
        direccion: Direccion {
            calle: String::from("Calle Principal 123"),
            ciudad: String::from("Ciudad Ejemplo"),
        },
    };

    // Serializar a JSON
    let json = serde_json::to_string_pretty(&persona).unwrap();
    println!("JSON:\n{}", json);

    // Deserializar de JSON
    let persona_deserializada: Persona = serde_json::from_str(&json).unwrap();
    println!("Persona deserializada: {:?}", persona_deserializada);
}
```

6. Leer JSON desde un archivo

```rust
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Configuracion {
    nombre_app: String,
    version: String,
    max_usuarios: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("config.json")?;
    let reader = BufReader::new(file);

    let config: Configuracion = serde_json::from_reader(reader)?;
    println!("Configuración cargada: {:?}", config);

    Ok(())
}
```

7. Escribir CSV a un archivo

```rust
use serde::Serialize;
use std::error::Error;
use std::fs::File;

#[derive(Serialize)]
struct Empleado {
    id: u32,
    nombre: String,
    departamento: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let empleados = vec![
        Empleado { id: 1, nombre: String::from("Alice"), departamento: String::from("IT") },
        Empleado { id: 2, nombre: String::from("Bob"), departamento: String::from("HR") },
    ];

    let file = File::create("empleados.csv")?;
    let mut wtr = csv::Writer::from_writer(file);

    for empleado in empleados {
        wtr.serialize(empleado)?;
    }

    wtr.flush()?;
    println!("CSV escrito con éxito");

    Ok(())
}
```

Estos ejemplos cubren una variedad de escenarios comunes al trabajar con JSON y CSV en Rust utilizando Serde. Muestran cómo serializar y deserializar datos, trabajar con archivos, y manejar estructuras anidadas. Recuerda que Serde ofrece muchas más características y opciones para casos más complejos, pero estos ejemplos deberían proporcionarte una buena base para empezar.