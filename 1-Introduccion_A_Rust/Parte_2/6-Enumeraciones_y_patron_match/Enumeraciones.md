## ENUMERACIONES Y PATTERN MATCHING

```
- Definiendo un Enum
- Option Enum y sus ventajas sobre los valores Nulos
- El match del Flujo de Control de Construct
- Patrones que enlazan con otros Valores
- Matching con Option<T>
- Los Matches son exhaustivos
- Patrones Catch-all y el _Placeholder
- Flujo de Control conciso, usando: if let
```

### Definiendo un Enum

Los enums en Rust se usan cuando queremos definir un tipo de dato que contiene un número fijo de posibles valores. 

Para definir un enum utilizamos la palabra clave `enum` seguido del nombre de la variable en `PascalCase` y dentro del bloque de código, los diferentes valores que puede tomar.

```
enum Dia {
    Lunes,
    Martes,
    Miercoles,
    Jueves,
    Viernes,
    Sabado,
    Domingo,
}
```

Los `enums` son similares a los `structures`, pero no hace falta asociar un tipo de dato dentro de los `enums`.

Para crear una instancia del `enum` debemos elegir una de las opciones utilizando la sintaxis de dos puntos repetidos `::`. 

```
let hoy = Dia::Jueves;
```

Una de las ventajas de usar `enums` en vez de `Srtings`, por ejemplo, es que se evitan errores ya que el dìa de la semana debe coincidir exactamente con los valores definidos o el compilador arrojará un error.

### El match del Flujo de Control de Construct

Vamos a ver un ejemplo más complejo. Imagínate que tienes una empresa de delivery y para calcular el coste de hacer una entrega se toman en cuenta dos variables, el tipo de vehículo y la distancia.

```
enum DeliveryType {
    Bicicleta,
    Moto,
    TransportePublico,
}

impl DeliveryType {
    fn calcular_envio(&self, km: f32) -> f32 {
        let coste = match self {
            DeliveryType::Bicicleta => 1.0 * km,
            DeliveryType::Moto => 1.5 * km,
            DeliveryType::TransportePublico => 0.8 * km
        };
        coste
    }
}

fn main() {
    let vehiculo = DeliveryType::Moto;
    let envio = vehiculo.calcular_envio(3.2);
    println!("El delivery te va a costar {} euros", envio)
    
}
```

Los `enums` pueden también tener algun dato asociado, y se puede definir usando paréntesis `()` y el tipo de dato o bien llaves `{}` y el nombre de la variable a guardar seguido de dos puntos `:` y el tipo de dato a guardar. Vamos a ajustar el ejemplo anterior para mostrar como puede hacerse.

```
enum DeliveryType {
    Bicicleta(f32),
    Moto(f32),
    TransportePublico { km: f32 },
}

impl DeliveryType {
    fn calcular_envio(&self) -> f32 {
        let coste = match self {
            DeliveryType::Bicicleta(km) => 1.0 * km,
            DeliveryType::Moto(km) => 1.5 * km,
            DeliveryType::TransportePublico { km } => 0.8 * km
        };
        coste
    }
}

fn main() {
    let vehiculo = DeliveryType::Moto(2.5);
    let envio = vehiculo.calcular_envio();
    println!("El delivery te va a costar {} euros", envio)
    
}
```

### Option Enum y sus ventajas sobre los valores Nulos

En algunos lenguajes de programación se puede usar el valor `null` para definir la ausencia de un valor, pero Rust no lo define. Sin embargo, en Rust existe el `Option enum` y está definido en la librería estandar.

```
pub enum Option<T> {
    /// No value.
    None,
    /// Some value of type `T`.
    Some(T),
}
```