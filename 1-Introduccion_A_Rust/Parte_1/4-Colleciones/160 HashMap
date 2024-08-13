
# HashMap

## Ejemplos de Uso de `HashMap`

Primero, asegúrate de importar `HashMap`:

```rust
use std::collections::HashMap;
```

Ahora, veamos varios ejemplos:

1. Crear y usar un HashMap básico

```rust
fn main() {
    let mut mapa = HashMap::new();

    // Insertar elementos
    mapa.insert(String::from("azul"), 10);
    mapa.insert(String::from("rojo"), 50);

    // Acceder a un valor
    if let Some(valor) = mapa.get("azul") {
        println!("El valor de azul es: {}", valor);
    }

    // Iterar sobre el HashMap
    for (clave, valor) in &mapa {
        println!("{}: {}", clave, valor);
    }
}
```

2. Actualizar un valor en el HashMap

```rust
fn main() {
    let mut puntuaciones = HashMap::new();

    puntuaciones.insert(String::from("Azul"), 10);

    // Sobrescribir un valor
    puntuaciones.insert(String::from("Azul"), 25);

    // Actualizar un valor basado en el valor antiguo
    puntuaciones.entry(String::from("Amarillo")).or_insert(50);
    puntuaciones.entry(String::from("Azul")).and_modify(|e| *e += 10);

    println!("{:?}", puntuaciones);
}
```

3. Usar `entry` para insertar si la clave no existe

```rust
fn main() {
    let texto = "hola mundo maravilloso mundo";
    let mut conteo = HashMap::new();

    for palabra in texto.split_whitespace() {
        let contador = conteo.entry(palabra).or_insert(0);
        *contador += 1;
    }

    println!("{:?}", conteo);
}
```

4. Combinar dos HashMaps

```rust
fn main() {
    let mut mapa1 = HashMap::new();
    mapa1.insert(1, "a");
    mapa1.insert(2, "b");

    let mut mapa2 = HashMap::new();
    mapa2.insert(3, "c");
    mapa2.insert(4, "d");

    mapa1.extend(mapa2);

    println!("{:?}", mapa1);
}
```

5. Usar un HashMap con tipos personalizados

```rust
#[derive(Hash, Eq, PartialEq, Debug)]
struct Persona {
    id: u32,
    nombre: String,
}

fn main() {
    let mut empleados = HashMap::new();

    empleados.insert(
        Persona { id: 1, nombre: String::from("Alice") },
        String::from("Desarrollador")
    );
    empleados.insert(
        Persona { id: 2, nombre: String::from("Bob") },
        String::from("Diseñador")
    );

    for (persona, trabajo) in &empleados {
        println!("{:?} es {}", persona, trabajo);
    }
}
```

6. Eliminar elementos de un HashMap

```rust
fn main() {
    let mut frutas = HashMap::new();
    frutas.insert(String::from("Manzana"), 3);
    frutas.insert(String::from("Banana"), 2);
    frutas.insert(String::from("Naranja"), 5);

    // Eliminar una entrada
    frutas.remove("Banana");

    // Eliminar una entrada solo si tiene un valor específico
    frutas.remove_entry("Naranja");

    println!("{:?}", frutas);
}
```

7. Usar `HashMap` con `Option` y `Result`

```rust
fn main() {
    let mut configuracion = HashMap::new();
    configuracion.insert(String::from("color"), String::from("rojo"));
    configuracion.insert(String::from("tamaño"), String::from("grande"));

    let color = configuracion.get("color")
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| String::from("DESCONOCIDO"));

    println!("Color: {}", color);

    // Usando Result
    let resultado: Result<(), &str> = configuracion.get("forma")
        .map(|_| ())
        .ok_or("La forma no está especificada");

    match resultado {
        Ok(_) => println!("La forma está especificada"),
        Err(e) => println!("Error: {}", e),
    }
}
```

8. HashMap con valores por defecto

```rust
use std::collections::HashMap;

fn main() {
    let mut visitas = HashMap::new();

    println!("Visitas a /home: {}", visitas.entry("/home".to_string()).or_insert(0));
    println!("Visitas a /contact: {}", visitas.entry("/contact".to_string()).or_insert(0));

    *visitas.entry("/home".to_string()).or_insert(0) += 1;

    println!("Visitas actualizadas: {:?}", visitas);
}
```

Estos ejemplos cubren una amplia gama de usos comunes de `HashMap` en Rust. `HashMap` es una estructura de datos muy versátil y eficiente para muchos casos de uso, especialmente cuando necesitas acceso rápido a datos basados en una clave única. Recuerda que el orden de los elementos en un `HashMap` no está garantizado, si necesitas un orden específico, considera usar `BTreeMap` o mantener un vector separado con el orden deseado.