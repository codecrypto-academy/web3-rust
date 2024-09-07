# Tipos de Pruebas en Rust

## 1. Pruebas Unitarias

**Descripción**: Prueban funciones individuales o pequeñas unidades de código.

**Características**:
- Se escriben en el mismo archivo que el código que prueban.
- Usan el atributo `#[test]`.
- Generalmente se colocan en un módulo `tests` con `#[cfg(test)]`.

**Ejemplo**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma() {
        assert_eq!(suma(2, 2), 4);
    }
}
```

## 2. Pruebas de Integración

**Descripción**: Prueban cómo diferentes partes del código trabajan juntas.

**Características**:
- Se escriben en archivos separados dentro del directorio `tests/`.
- Pueden acceder a funciones y tipos públicos de la crate.
- Se ejecutan como si fueran una crate externa usando tu biblioteca.

**Ejemplo**:
```rust
// En tests/integration_test.rs
use mi_crate;

#[test]
fn test_funcionalidad_integrada() {
    assert!(mi_crate::funcion_publica());
}
```

## 3. Pruebas de Documentación

**Descripción**: Aseguran que los ejemplos en la documentación del código funcionen correctamente.

**Características**:
- Se escriben en los comentarios de documentación (`///` o `//!`).
- Se ejecutan con `cargo test`.

**Ejemplo**:
```rust
/// ```
/// # use mi_crate::suma;
/// assert_eq!(suma(2, 2), 4);
/// ```
pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}
```

## 4. Pruebas de Benchmark (Nightly Rust)

**Descripción**: Miden el rendimiento del código.

**Características**:
- Requieren Rust nightly.
- Usan el atributo `#[bench]`.

**Ejemplo**:
```rust
#![feature(test)]
extern crate test;

#[bench]
fn bench_suma(b: &mut test::Bencher) {
    b.iter(|| suma(2, 2));
}
```

## 5. Pruebas de Propiedades

**Descripción**: Generan casos de prueba automáticamente para verificar propiedades del código.

**Características**:
- Utilizan crates como `proptest` o `quickcheck`.
- Prueban el código con una amplia gama de entradas generadas.

**Ejemplo** (usando `proptest`):
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_suma_conmutativa(a in 0..100, b in 0..100) {
        prop_assert_eq!(suma(a, b), suma(b, a));
    }
}
```

## Cobertura de Código

La cobertura de código mide qué partes del código se ejecutan durante las pruebas.

### Herramientas:
1. **Tarpaulin**: Popular para proyectos Rust en sistemas Unix-like.
   ```
   cargo install cargo-tarpaulin
   cargo tarpaulin
   ```

2. **grcov**: Herramienta de Mozilla que funciona en múltiples plataformas.
   ```
   cargo install grcov
   # Requiere configuración adicional
   ```

3. **kcov**: Otra opción para sistemas Unix-like.

### Características de la Cobertura:
- Mide el porcentaje de líneas de código ejecutadas.
- Identifica ramas y condiciones no probadas.
- Ayuda a encontrar código muerto o no utilizado.

### Buenas Prácticas:
- Apuntar a una alta cobertura, pero no necesariamente 100%.
- Usar la cobertura como guía, no como objetivo final.
- Combinar con análisis manual para asegurar pruebas significativas.

## Consejos Generales para Pruebas en Rust

1. **Organización**: Usa `#[cfg(test)]` para código específico de pruebas.
2. **Aserciones**: Aprovecha macros como `assert!`, `assert_eq!`, `assert_ne!`.
3. **Mocking**: Utiliza crates como `mockall` para crear objetos simulados.
4. **Pruebas Paralelas**: Rust ejecuta pruebas en paralelo por defecto; usa `#[test]` con cuidado en pruebas que no son thread-safe.
5. **Pruebas Condicionales**: Usa `#[cfg(feature = "...")]` para pruebas que dependen de características específicas.

Este resumen abarca los principales tipos de pruebas en Rust, ofreciendo una visión general de cómo puedes asegurar la calidad y el rendimiento de tu código Rust.