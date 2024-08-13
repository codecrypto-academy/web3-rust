# Ejemplos de Struct y Vectores de Struct


Este ejemplo muestra:

1. Definición de estructuras simples y compuestas.
2. Creación de instancias de estructuras.
3. Acceso y modificación de campos de estructuras.
4. Creación y manipulación de vectores de estructuras.
5. Iteración sobre vectores de estructuras.
6. Filtrado de vectores basado en propiedades de las estructuras.
7. Uso de estructuras en cálculos.
8. Estructuras anidadas (una estructura como campo de otra).
9. Uso de enums para crear colecciones de diferentes tipos de estructuras.


```rust
// Definición de una estructura simple
struct Persona {
    nombre: String,
    edad: u32,
}

// Definición de una estructura con campos de diferentes tipos
struct Punto {
    x: f64,
    y: f64,
}

fn main() {
    // Crear una instancia de Persona
    let persona1 = Persona {
        nombre: String::from("Alice"),
        edad: 30,
    };

    // Acceder a los campos de la estructura
    println!("{} tiene {} años", persona1.nombre, persona1.edad);

    // Crear un vector de Personas
    let mut personas = vec![
        Persona {
            nombre: String::from("Bob"),
            edad: 25,
        },
        Persona {
            nombre: String::from("Charlie"),
            edad: 35,
        },
    ];

    // Añadir una nueva Persona al vector
    personas.push(Persona {
        nombre: String::from("David"),
        edad: 40,
    });

    // Iterar sobre el vector de Personas
    for persona in &personas {
        println!("{} tiene {} años", persona.nombre, persona.edad);
    }

    // Modificar una Persona en el vector
    if let Some(persona) = personas.get_mut(0) {
        persona.edad += 1;
    }

    // Filtrar el vector
    let mayores_de_30: Vec<&Persona> = personas.iter().filter(|p| p.edad > 30).collect();
    println!("Personas mayores de 30:");
    for persona in mayores_de_30 {
        println!("{}", persona.nombre);
    }

    // Usar la estructura Punto
    let punto = Punto { x: 3.0, y: 4.0 };
    let distancia = (punto.x.powi(2) + punto.y.powi(2)).sqrt();
    println!("Distancia desde el origen: {}", distancia);

    // Vector de Puntos
    let mut puntos = vec![
        Punto { x: 1.0, y: 2.0 },
        Punto { x: 3.0, y: 4.0 },
    ];

    // Añadir un nuevo Punto
    puntos.push(Punto { x: 5.0, y: 6.0 });

    // Calcular la suma de todas las coordenadas x
    let suma_x: f64 = puntos.iter().map(|p| p.x).sum();
    println!("Suma de todas las coordenadas x: {}", suma_x);

    // Encontrar el punto con la mayor coordenada y
    if let Some(punto_max_y) = puntos.iter().max_by(|a, b| a.y.partial_cmp(&b.y).unwrap()) {
        println!("Punto con mayor y: ({}, {})", punto_max_y.x, punto_max_y.y);
    }

    // Estructura con campo de otra estructura
    struct Rectangulo {
        esquina_sup_izq: Punto,
        ancho: f64,
        alto: f64,
    }

    let rectangulo = Rectangulo {
        esquina_sup_izq: Punto { x: 0.0, y: 0.0 },
        ancho: 5.0,
        alto: 3.0,
    };

    let area = rectangulo.ancho * rectangulo.alto;
    println!("Área del rectángulo: {}", area);

    // Vector de diferentes tipos de estructuras usando enum
    enum Forma {
        Circulo(f64),
        Rectangulo(Rectangulo),
    }

    let formas = vec![
        Forma::Circulo(2.0),
        Forma::Rectangulo(Rectangulo {
            esquina_sup_izq: Punto { x: 0.0, y: 0.0 },
            ancho: 4.0,
            alto: 3.0,
        }),
    ];

    for forma in &formas {
        match forma {
            Forma::Circulo(radio) => println!("Círculo con radio {}", radio),
            Forma::Rectangulo(rect) => println!("Rectángulo de {}x{}", rect.ancho, rect.alto),
        }
    }
}
```
