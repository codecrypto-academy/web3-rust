# Control de Flujo en Rust

## 1. if y Expresión if

### if Básico
```rust
let numero = 6;

if numero % 2 == 0 {
    println!("El número es par");
} else if numero % 3 == 0 {
    println!("El número es divisible por 3");
} else {
    println!("El número no es par ni divisible por 3");
}
```

### Expresión if
```rust
let condicion = true;
let numero = if condicion { 5 } else { 6 };

println!("El valor de número es: {}", numero);
```

## 2. loop y Expresión loop

### loop Básico
```rust
loop {
    println!("Esto se imprimirá indefinidamente");
    break; // Evita que el bucle sea realmente infinito
}
```

### Expresión loop
```rust
let mut contador = 0;
let resultado = loop {
    contador += 1;
    if contador == 10 {
        break contador * 2;
    }
};
println!("El resultado es {}", resultado);
```

## 3. while

```rust
let mut numero = 3;
while numero != 0 {
    println!("{}!", numero);
    numero -= 1;
}
println!("¡DESPEGUE!");
```

## 4. for y Expresión for

### for Básico
```rust
for numero in 1..4 {
    println!("{}!", numero);
}
```

### Iterando sobre una Colección
```rust
let a = [10, 20, 30, 40, 50];
for elemento in a.iter() {
    println!("el valor es: {}", elemento);
}
```

### Expresión for (usando collect)
```rust
let numeros = vec![1, 2, 3, 4, 5];
let cuadrados: Vec<i32> = (for num in numeros {
    num * num
}).collect();
println!("Cuadrados: {:?}", cuadrados);
```

## 5. match

```rust
let numero = 13;
match numero {
    1 => println!("Uno"),
    2 | 3 | 5 | 7 | 11 => println!("Es un número primo"),
    13..=19 => println!("Un adolescente"),
    _ => println!("Otro número"),
}
```

## 6. continue y break

### continue
```rust
for numero in 0..5 {
    if numero % 2 == 0 {
        continue;
    }
    println!("{}", numero);
}
```

### break
```rust
let mut contador = 0;
loop {
    println!("Contador: {}", contador);
    contador += 1;
    if contador == 5 {
        break;
    }
}
```

## 7. Etiquetas de Bucle

```rust
'exterior: loop {
    println!("Bucle exterior");
    'interior: loop {
        println!("Bucle interior");
        break 'exterior;
    }
}
```

## 8. if let

```rust
let algunos_valores = Some(5);
if let Some(x) = algunos_valores {
    println!("x = {}", x);
}
```

## 9. while let

```rust
let mut pila = Vec::new();
pila.push(1);
pila.push(2);
pila.push(3);

while let Some(valor_superior) = pila.pop() {
    println!("Valor superior = {}", valor_superior);
}
```

## 10. Expresiones de Retorno Temprano

```rust
fn buscar(objetivo: i32, nums: &[i32]) -> bool {
    for &num in nums {
        if num == objetivo {
            return true;
        }
    }
    false
}
```

## Características Avanzadas

### 1. match con Guardas
```rust
let numero = 4;
match numero {
    n if n < 0 => println!("Negativo"),
    n if n > 0 => println!("Positivo"),
    _ => println!("Cero"),
}
```

### 2. for con Patrones
```rust
let v = vec![(1, 'a'), (2, 'b'), (3, 'c')];
for (numero, letra) in v {
    println!("{}: {}", numero, letra);
}
```

### 3. break con Valor en for
```rust
let resultado = for i in 0..100 {
    if i * i > 400 {
        break i;
    }
};
println!("El primer número cuyo cuadrado es mayor que 400 es: {}", resultado);
```

### 4. loop con Condición de Salida
```rust
let mut contador = 0;
let resultado = loop {
    contador += 1;
    if contador * contador > 100 {
        break contador;
    }
};
println!("El resultado es: {}", resultado);
```

El control de flujo en Rust es rico y expresivo, permitiendo escribir código claro y conciso. Las características como expresiones `if`, `loop`, y `match` hacen que el código sea más legible y mantenible. Además, la seguridad de Rust se extiende a estas estructuras, ayudando a prevenir errores comunes y garantizando un comportamiento predecible.