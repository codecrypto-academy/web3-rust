## Estructuras

```
- Estructuras, definición e instanciación de Structs
- Usando el Campo Init Shorthand
- Creando Instancias desde Otras Instancias con la sintaxis struct Update
- Usando Structs de Tuplas sin Campos Nombrados, para crear Tipos Diferentes
- Structs Unit-Like sin ningún campo
- Propiedad de los datos de un struct
- EJEMPLO: Programa de ejemplo, usando Structs
- Refactorizado con Tuplas
- Refactorizado con Structs: Añadiendo más significado
- Añadiendo Funcionalidades útiles con Derived Traits
- Method Syntax
- Definiendo Métodos
- ¿Dónde está el operador ->?
- Métodos con más parámetros
- Funciones Asociadas
- Múltiples Bloques impl
```

### Definición e instanciación de estructuras.

Las estructuras nos permiten agrupar datos que están relacionados. Para definir una estructura usamos la palabra clave `struct` seguido del nombre usando la convecnción `PascalCase`. Luego usamos llaves `{}` para definir el bloque que va a contener la estructura.

El primer campo de la estructura es el nombre del atributo y se escribe usando la convención `snake_case`, seguido del tipo de dato en `PascalCase`.

```
struct Token {
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u128,
}
```

Cuando creamos una instancia de un token debemos inicializar todos los campos de la estructura.
```
let bitcoin = Token {
    name: String::from("Bitcoin"),
    symbol: "BTC".to_string(),
    decimals: 8,
    total_supply: 21_000_000,
};
```
### Creando Instancias desde Otras Instancias con la sintaxis struct Update

Para crear otra instancia de la misma estructura podemos definir únicamente los valores que deseamos cambiar y usar los demás valores previamente definidios en otra variable. Para ello usamos la palabra clave `..` seguido del nombre de la variable de la cual queremos copiar sus valores.

```
    let bitcoin_cash = Token {
    name: "Bitcoin Cash".to_string(),
    symbol: "BCH".to_string(),
    total_supply: 420_000_000_000,
    ..bitcoin
    };
```

Si quisiéramos crear una nueva instancia de `Token` reusando la instancia `bitcoin`, esto generaría un problema de propiedad con variables de tipo no primitivo, en este caso `String`, ya que en Rust `String` no implementa el trait `Copy`. Esto se puede resolver clonando la variable `bitcoin` después de añadir el trait `Clone` a la estructura para permitir clonarla.

```
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Token {
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u128,
}


fn main() {
    let bitcoin = Token {
        name: String::from("Bitcoin"),
        symbol: "BTC".to_string(),
        decimals: 8,
        total_supply: 21_000_000u128,
    };
    
    let bitcoin_cash = Token {
    name: "Bitcoin Cash".to_string(),
    // symbol: "BCH".to_string(),
    total_supply: 420_000_000_000,
    ..bitcoin.clone()
    };

    println!("BTC: {:?}\n", bitcoin);
    println!("BCH: {:?}", bitcoin_cash);

}

----- Standard Output -----

BTC: Token { name: "Bitcoin", symbol: "BTC", decimals: 8, total_supply: 21000000 }

BCH: Token { name: "Bitcoin Cash", symbol: "BTC", decimals: 8, total_supply: 420000000000 }
```

Para acceder a los campos de una estructura podemos usar `dot notation` después del nombre de la estructura.

```
let total_supply = bitcoin.total_supply;
```

También podemos usar la misma notación para mutar un campo de la estructura. Acuérdate que en Rust todas las variables son inmutables por defecto, así que debemos usar la palabra clave `mut` en su definición para poder mutarla.

```
bitcoin.total_supply = 42_000_000;
```


Para añadir funcionalidad a una estructura podemos definir bloques de implementación usando la palabra clave `impl` seguida del nombre de la estructura. Dentro del bloque de implementación podemos definir varias funciones que van a estar asociadas a ese tipo únicamente.

Podemos usar la palabra clave `self` que se refiere a la instancia de la estructura, en nuestro caso del `Token`

Para llamar el método usamos el nombre de la variable seguido de un punto `.` y luego el nombre de la función. En el ejemplo siguiente, podemos ver que no pasamos ningún argumento a la función ya que al usar `self` en la definición, la instancia se pasará automáticamente sin necesidad de pasarla en la llamada.

```
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Token {
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u128,
}

impl Token {
    fn get_total_supply(&self) -> u128 {
        self.total_supply
    }
}


fn main() {
    let bitcoin = Token {
        name: String::from("Bitcoin"),
        symbol: "BTC".to_string(),
        decimals: 8,
        total_supply: 21_000_000u128,
    };
    println!("\nThe total supply of Bitcoin is {}", bitcoin.get_total_supply());

}

----- Standard Output -----

The total supply of Bitcoin is 21000000
```

Para que una función pueda considerarse un método, se tienen que cumlir dos cosas:
1. Denemos pasar como primer parámetro a la función `&self`.
2. La función debe definirse dentro del bloque de implementación.

Podríamos pasar una referencia mutable de `&mut self` si quisiéramos que el método modifique la instancia. Por ejemplo, si quisiéramos aumentar la cantidad de decimales que usa Bitcoin haríamos lo siguiente.

```
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Token {
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u128,
}

impl Token {
    fn _get_total_supply(&self) -> u128 {
        self.total_supply
    }
    
    fn update_decimals(&mut self, decimals: u8) {
        self.decimals = decimals
    }
}


fn main() {
    let mut bitcoin = Token {
        name: String::from("Bitcoin"),
        symbol: "BTC".to_string(),
        decimals: 8,
        total_supply: 21_000_000u128,
    };
    bitcoin.update_decimals(18);
    println!("\nBitcoin now has {} decimals", bitcoin.decimals);

}

----- Standard Output -----

Bitcoin now has 18 decimals

```
