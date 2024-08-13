# rust_decimal en Rust

## Propósito Principal

`rust_decimal` es una crate de Rust diseñada para manejar números decimales con precisión fija. Se utiliza principalmente para:

1. **Cálculos financieros**
2. **Operaciones contables**
3. **Cualquier escenario que requiera aritmética decimal precisa**

## Características Clave

- **Precisión fija**: Mantiene una precisión constante en los cálculos.
- **Sin errores de redondeo**: Evita los problemas típicos de los números de punto flotante.
- **Rendimiento**: Optimizado para operaciones rápidas.
- **Compatibilidad**: Implementa traits estándar de Rust para facilitar su uso.

## Uso Básico

Para usar `rust_decimal`, añade esto a tu `Cargo.toml`:

```toml
[dependencies]
rust_decimal = "1.26"
```

Ejemplo simple:

```rust
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn main() {
    let price = Decimal::new(1000, 2); // 10.00
    let quantity = dec!(3.5);
    let total = price * quantity;
    
    println!("Total: {}", total); // Imprime: Total: 35.00
}
```

## Ventajas

1. **Precisión**: Ideal para cálculos que requieren exactitud decimal.
2. **Confiabilidad**: Evita errores sutiles en cálculos financieros.
3. **Facilidad de uso**: API intuitiva y familiar.
4. **Interoperabilidad**: Buena integración con tipos estándar de Rust.

## Casos de Uso Comunes

1. **Sistemas de facturación**
2. **Cálculos de impuestos**
3. **Aplicaciones bancarias**
4. **Contabilidad y auditoría**
5. **Cálculos científicos que requieren precisión decimal**

## Funcionalidades Avanzadas

- **Redondeo controlado**: Múltiples opciones de redondeo disponibles.
- **Serialización**: Soporte para serde.
- **Operaciones matemáticas**: Incluye funciones trigonométricas y exponenciales.
- **Conversión de tipos**: Fácil conversión desde y hacia otros tipos numéricos.

## Comparación con Alternativas

| Característica | rust_decimal | f64 | BigDecimal |
|----------------|--------------|-----|------------|
| Precisión      | Fija         | Limitada | Arbitraria |
| Rendimiento    | Alto         | Muy Alto | Moderado |
| Uso de Memoria | Moderado     | Bajo | Alto |
| Exactitud      | Alta         | Baja | Muy Alta |

## Consideraciones

1. **Tamaño**: Ocupa más espacio que los tipos de punto flotante estándar.
2. **Rango limitado**: A diferencia de `BigDecimal`, tiene un rango máximo fijo.
3. **Curva de aprendizaje**: Puede requerir ajustes en el pensamiento para desarrolladores acostumbrados a punto flotante.

## Conclusión

`rust_decimal` es una herramienta esencial para cualquier desarrollador de Rust que trabaje en aplicaciones financieras o que requieran cálculos decimales precisos. Ofrece un equilibrio entre la exactitud de los cálculos y el rendimiento, haciéndolo ideal para una amplia gama de aplicaciones en el mundo real.

## Calculo del importe de una factura:

```rust
let bruto = 453 * 2350;
let iva: i32 = (bruto as f32 * 21_f32 / 100_f32).round() as i32; 
let importe = bruto + iva as i32;
println!("El bruto es: {}", bruto);
println!("El iva es: {}",iva);
println!("El importe es: {}", importe);
use rust_decimal::prelude::*;
let bruto = Decimal::new(453, 0) * Decimal::new(2350, 0);
let iva = (bruto * Decimal::new(21, 0) / Decimal::new(100,0)).round();
let importe = bruto + iva;
println!("El bruto es: {}", bruto);
println!("El iva es: {} ",iva);
println!("El total es: {}", importe.round());
```