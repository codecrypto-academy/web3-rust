# Ejemplos de Uso de Date Time en Rust
Primero, asegúrate de añadir `chrono` a tu `Cargo.toml`:

```toml
[dependencies]
chrono = "0.4"
```

Ahora, aquí están los ejemplos en formato Markdown:

# Ejemplos de Uso de Date, Time y Duration en Rust

## Importaciones Necesarias

```rust
use chrono::{NaiveDate, NaiveDateTime, Datelike, Weekday, Duration};
use chrono::prelude::*;
```

## Crear y Mostrar Fechas

```rust
fn main() {
    // Fecha actual
    let hoy = Local::now().date_naive();
    println!("Hoy es: {}", hoy);

    // Crear una fecha específica
    let fecha = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    println!("Fecha específica: {}", fecha);

    // Crear fecha y hora
    let fecha_hora = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
        NaiveTime::from_hms_opt(23, 59, 59).unwrap()
    );
    println!("Fecha y hora: {}", fecha_hora);
}
```

## Calcular Días Entre Dos Fechas

```rust
fn main() {
    let fecha1 = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let fecha2 = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();

    let duracion = fecha2.signed_duration_since(fecha1);
    println!("Días entre {} y {}: {}", fecha1, fecha2, duracion.num_days());
}
```

## Obtener el Día de la Semana

```rust
fn main() {
    let fecha = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let dia_semana = fecha.weekday();

    println!("El {} es {}", fecha, dia_semana);

    // Traducir el día de la semana al español
    let dia_espanol = match dia_semana {
        Weekday::Mon => "Lunes",
        Weekday::Tue => "Martes",
        Weekday::Wed => "Miércoles",
        Weekday::Thu => "Jueves",
        Weekday::Fri => "Viernes",
        Weekday::Sat => "Sábado",
        Weekday::Sun => "Domingo",
    };

    println!("En español: {}", dia_espanol);
}
```

## Añadir y Restar Tiempo

```rust
fn main() {
    let hoy = Local::now().date_naive();
    
    // Añadir días
    let futuro = hoy + Duration::days(30);
    println!("30 días en el futuro: {}", futuro);

    // Restar días
    let pasado = hoy - Duration::days(30);
    println!("30 días en el pasado: {}", pasado);

    // Añadir meses (nota: esto usa la característica "months" de chrono)
    let tres_meses_despues = hoy + chrono::Months::new(3);
    println!("3 meses después: {}", tres_meses_despues);
}
```

## Formateo Personalizado de Fechas

```rust
fn main() {
    let ahora = Local::now();
    
    // Formato personalizado
    println!("Formato personalizado: {}", ahora.format("%d/%m/%Y %H:%M:%S"));
    
    // Formato ISO 8601
    println!("Formato ISO 8601: {}", ahora.to_rfc3339());
}
```

## Parseo de Fechas desde Strings

```rust
fn main() {
    let fecha_str = "2023-12-31";
    let fecha = NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d")
        .expect("Error al parsear la fecha");
    println!("Fecha parseada: {}", fecha);

    let fecha_hora_str = "2023-12-31 23:59:59";
    let fecha_hora = NaiveDateTime::parse_from_str(fecha_hora_str, "%Y-%m-%d %H:%M:%S")
        .expect("Error al parsear la fecha y hora");
    println!("Fecha y hora parseadas: {}", fecha_hora);
}
```

## Trabajar con Zonas Horarias

```rust
use chrono::{DateTime, Utc, FixedOffset};

fn main() {
    let utc_time = Utc::now();
    println!("Tiempo UTC: {}", utc_time);

    // Convertir a una zona horaria específica (por ejemplo, UTC+2)
    let offset = FixedOffset::east_opt(2 * 3600).unwrap(); // 2 horas en segundos
    let local_time: DateTime<FixedOffset> = utc_time.with_timezone(&offset);
    println!("Tiempo local (UTC+2): {}", local_time);
}
```

Estos ejemplos cubren una amplia gama de operaciones comunes con fechas y tiempos en Rust usando la crate `chrono`. Incluyen la creación y manipulación de fechas, cálculo de diferencias entre fechas, determinación del día de la semana, formateo y parsing de fechas, y manejo básico de zonas horarias.

Recuerda que `chrono` ofrece muchas más funcionalidades, incluyendo soporte para diferentes calendarios y operaciones más avanzadas con zonas horarias. La documentación oficial de `chrono` es un excelente recurso para explorar más características.