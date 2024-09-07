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

En algunos lenguajes de programación se puede usar el valor `null` para definir la ausencia de un valor, pero Rust no lo define. Sin embargo, en Rust existe el `Option enum` y está definido en la librería estandar por lo que no hace falta importar ninguna librería para usarlas.

```
pub enum Option<T> {
    /// No value.
    None,
    /// Some value of type `T`.
    Some(T),
}
```

`Some(T)` encapsula un valor genérico de tipo `T` y `None` representa la ausencia de un valor.

Cuando definimos el tipo de dato que puede tomar una variable, si quisiéramos que fuese opcional, tendríamos que definirlo usando la palabra clave `Option` seguido de los signos menor que `<` y mayor que `>`, encapsulando el tipo de la variable dentro de ambos.

```
struct Usuario {
    nombre: String,
    primer_apellido: String,
    segundo_apellido: Option<String>,
}

let usuario = Usuario {
    nombre: "Pedro".to_string(),
    primer_apellido: "Jimenez".to_string(),
    segundo_apellido: Some("Rivas".to_string()),
};

let usuario = Usuario {
    nombre: "John".to_string(),
    primer_apellido: "Smith".to_string(),
    segundo_apellido: None,
};
```

Es bastante común que un función retorne un valor opcional, dependiendo de los parámetros de entrada. En el ejemplo siguiente, si el usuario no existiese, la función en vez de retornar el valor 0, retornaría `None` para que pueda ser correctamente gestionado, diciendo que el usuario no existe por ejemplo.

```
#[derive(Debug)]
struct Usuario {
    iban: String,
    saldo: f32,
}

fn get_balance(banco: &Vec<Usuario>, iban: &String) -> Option<f32> {
    for usuario in banco {
        if usuario.iban == *iban {
            return Some(usuario.saldo)
        }
    }
    None
}

fn main() {
    let banco = vec![
        Usuario { 
            iban: "ES1234".to_string(),
            saldo: 1024.12,
        },
        Usuario { 
            iban: "ES5678".to_string(),
            saldo: 420.69,
        },     
    ];
    
    let balance = get_balance(&banco, &"ES1234".to_string());
    println!("La cuenta ES1234 tiene {} euros", balance.unwrap())
    
}


----- Standard Output -----

La cuenta ES1234 tiene 1024.12 euros
```

### Matching con Option<T>

Podemos usar el patrón `match` para decidir que hacer con un resultado que es opcional. Por ejemplo, si tenemos algún resultado hacer algo, y si no hay resultado hacer otra cosa. Usando el ejemplo anterior podríamos hacer lo siguiente para hacer el código más robusto en el caso de que la cuenta no exista.

```
#[derive(Debug)]
struct Usuario {
    iban: String,
    saldo: f32,
}

fn get_balance(banco: &Vec<Usuario>, iban: &String) -> Option<f32> {
    for usuario in banco {
        if usuario.iban == *iban {
            return Some(usuario.saldo)
        }
    }
    None
}

fn main() {
    let banco = vec![
        Usuario { 
            iban: "ES1234".to_string(),
            saldo: 1024.12,
        },
        Usuario { 
            iban: "ES5678".to_string(),
            saldo: 420.69,
        },     
    ];
    
    let iban = String::from("GB1337");
    let balance = get_balance(&banco, &iban.to_string());
    
    match balance {
        Some(balance) => println!("La cuenta {iban} tiene {balance} euros"),
        None => println!("La cuenta {iban} no existe")
    }
}

----- Standard Output -----

La cuenta GB1337 no existe
```

### Flujo de Control conciso, usando: if let

Si sólamente estamos interesados en el caso de que exista algún valor, descartando todos los demás posibles resultados, podemos usar la sintaxis plana.

Para ello, usamos la palabra clave `if let` seguida de `Some` y en paréntesis `()` el nombre de la variable que queremos consumir y la igualamos `=` a la variable que contiene el valor opcional.

```
if let Some(saldo) = balance {
    println!("La cuenta {iban} tiene {saldo} euros")
}
```

Se lee así, si hay algún valor dentro de la variable `balance`, lo llamaremos `saldo` e imprimiremos por pantalla ese valor. En caso de que `balance` sea `None` no haremos nada.

Dentro del bloque de código podemos imrpimir, asignar ese valor a otra variable, etc.

