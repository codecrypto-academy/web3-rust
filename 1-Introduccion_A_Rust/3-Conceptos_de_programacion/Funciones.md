## Funciones
		- Funciones
		- Parámetros
		- Sentencias y expresiones
		- Devolución de valores en funciones
		- La Recursividad en Rust

Las funciones en Rust se definen usando la palabra `fn` seguido del nombre de la función. Los paréntesis `()` pueden contener los parámetros de entrada de la función, si hubiese alguno. 

Debe especificarse el nombre de los parámetros de entrada seguido de su tipo, usando `:`. Para añadir más de un parámetro se pueden separar con una coma `,`.

```
fn imprimir(nombre: &str) {
	println!("Mi nombre es {}.", nombre);
}

fn main() {
    imprimir("Nahem");
}

----- Standard Output -----

Mi nombre es Nahem.
```

Después de los paréntesis `()`, se puede indicar con una flecha `->` el tipo de dato que retorna la función, si hubiese, o se puede dejar en blanco si no retorna nada. Entre llaves `{}` se encuentra el bloque de código que va a ejecutarse cuando se llama la función.

En una función, la ultima línea se utiliza para retornar un valor siempre y cuando no se coloque un punto y coma `;` al final de la línea. A esto se le conoce como una expresión. Las declaraciones suelen llevar un punto y coma `;` al final y no retornan ningún valor.

Si quisiéramos retornar algún valor en algún lugar que no sea la última línea, se utiliza la palabra `return` seguida de la expresión que deseamos retornar. En este caso sí tenemos que terminar la línea con un punto y coma `;`.

```
fn promedio(a: f32, b: f32) -> f32 {
    (a + b) / 2.0
}


fn main() {
    let avg = promedio(3.0, 6.0);
    println!("promedio: {}", avg);
}

----- Standard Output -----

promedio: 4.5
```

La convención para definir el nombre de las funciones en Rust es utilizar `snake_case`. Esto significa que todas las letras de la(s) palabra(s) se escriben en minúscula, y se separan con un `_` cuando hay más de una.

Como en otros lenguajes de programación, el resultado de una función se puede guardar en una variable si la declaramos al momento de llamarla.

```
let avg = promedio(3.14, 15.92);
```

Si una función retorna varios resultados, por ejemplo, una tupla, podemos desestruxcturarlos y asignarlos a variables individuales de la siguiente forma.

```
fn operaciones(a: u8, b: u8) -> (u8, u8) {
    let suma = a + b;
    let multiplicacion = a * b;
    (suma, multiplicacion)

}


fn main() {
    let (suma, multiplicacion) = operaciones(5, 4);
    println!("suma: {} | multiplicación: {}", suma, multiplicacion);
    
}

----- Standard Output -----

suma: 9 | multiplicación: 20

```

