# Implementación de Structs en Rust sin Traits

En Rust, las estructuras (structs) son una forma fundamental de organizar y encapsular datos relacionados. Aunque los traits son una característica poderosa en Rust para definir comportamientos compartidos, es posible y a menudo útil implementar funcionalidad para structs sin depender de traits. Este ejemplo demuestra cómo hacerlo.

```rust
// Definición de la estructura
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

// Implementación de métodos para la estructura
impl Rectangulo {
    // Método constructor
    fn nuevo(ancho: u32, alto: u32) -> Rectangulo {
        Rectangulo { ancho, alto }
    }
    
    // Método para calcular el área
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }
}

// Uso de la estructura
fn main() {
    let rect = Rectangulo::nuevo(30, 50);
    println!("El área del rectángulo es: {}", rect.area());
}
```

Este código ilustra cómo definir una estructura `Rectangulo`, implementar métodos para ella, incluyendo un constructor y un método para calcular el área, y cómo utilizar esta estructura en el programa principal. Todo esto se logra sin la necesidad de definir o implementar traits.

## Explicación

1. **Definición de la estructura**: Se define `Rectangulo` con dos campos, `ancho` y `alto`.

2. **Implementación de métodos**:
   - `nuevo`: Un método constructor que crea una nueva instancia de `Rectangulo`.
   - `area`: Un método que calcula el área del rectángulo.

3. **Uso**: En la función `main`, se crea una instancia de `Rectangulo` y se calcula su área.

Este enfoque demuestra cómo las estructuras en Rust pueden ser poderosas y útiles por sí mismas, sin necesidad de traits adicionales.