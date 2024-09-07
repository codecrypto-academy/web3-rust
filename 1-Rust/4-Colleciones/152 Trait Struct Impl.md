# Traits, struct and impl

This example demonstrates the use of traits and structs to create a simple shape hierarchy.

# Traits en Rust

Los traits en Rust son una característica fundamental que permite definir comportamientos compartidos entre diferentes tipos. Se pueden considerar similares a las interfaces en otros lenguajes de programación, pero con algunas diferencias clave.

## Características principales de los traits

1. **Definición de comportamiento**: Los traits definen un conjunto de métodos que un tipo debe implementar.

2. **Abstracción**: Permiten escribir código genérico que puede trabajar con cualquier tipo que implemente el trait.

3. **Implementación por defecto**: Los traits pueden proporcionar implementaciones por defecto para algunos o todos sus métodos.

4. **Extensibilidad**: Se pueden implementar traits para tipos existentes, incluso si no son de tu propiedad.

5. **Restricciones genéricas**: Se utilizan para limitar los tipos genéricos a aquellos que implementan ciertos comportamientos.

## Ejemplo básico



```rust
use std::f64::consts::PI;

// Define a trait for shapes
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &str;
}

// Circle struct
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    fn name(&self) -> &str {
        "Circle"
    }
}

// Triangle struct
struct Triangle {
    base: f64,
    height: f64,
    side1: f64,
    side2: f64,
    side3: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn perimeter(&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }

    fn name(&self) -> &str {
        "Triangle"
    }
}

// Square struct
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }

    fn name(&self) -> &str {
        "Square"
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle {
        base: 4.0,
        height: 3.0,
        side1: 3.0,
        side2: 4.0,
        side3: 5.0,
    };
    let square = Square { side: 4.0 };

    // dyn permite trabajar con struct que implementen Shape
    // por eso podemos poner diferentes structs en el vector
    let shapes: Vec<&dyn Shape> = vec![&circle, &triangle, &square];

    for shape in shapes {
        println!("Shape: {}", shape.name());
        println!("Area: {:.2}", shape.area());
        println!("Perimeter: {:.2}", shape.perimeter());
        println!();
    }
}
