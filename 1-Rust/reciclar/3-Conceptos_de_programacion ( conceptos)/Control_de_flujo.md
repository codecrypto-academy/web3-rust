# Control de Flujo
		- Expresiones if y else if
		- Uso de if en un estamento let
		- La instrucción match

		FALTA!
		- Repetición con Bucles: loop, while y for
		- Valores devueltos desde un bucle
		- Etiquetando bucles para quitar ambigüedades
		- Bucles condicionales con while
		- Explorar una Colección con for

## Condicionales `if` y `else`

En Rust, como en cualquier otro lenguaje de programación, existen los condicionales básicos `if` / `else` para para controlar el flujo. Vamos a explicarlo mejor con un ejemplo.

```
fn main() {
    let apuesta = 69;
    let numero_ganador = 69;
    if apuesta == numero_ganador  {
        println!("Ganaste!")
    } else {
        println!("Siga participando")
    }
}
```
En Rust no hace falta colocar la condición en paréntesis `()`, pero si hace falta colocar el bloque de cada condición dentro de corchetes `{}`. Tampoco es necesario escribir la condición `else` si no hay otras posibles opciones, el código compila si la eliminamos también.

Dentro de las expresiones `if` / `else` siempre tiene que haber un resultado de tipo `boolean`.

La condición en la expresión `if` siempre debe tener un resultado de tipo `boolean`, es decir, el valor debe ser cierto o falso.

Se pueden encadenar varias condiciones utilizando expresiones `else if` una detras de otra.

```
fn main() {
    let semaforo = "rojo";
    if semaforo == "rojo" {
        println!("Frena!");
    } else if semaforo == "amarillo" {
        println!("Precaución");
    } else {
        println!("Avanza!")
    }
}
```

No es necesario usar `else if` en la última condición cuando hay un número finito de posibilidades como en el caso de los colores de un semáforo.

Si queremos asignar un valor a una variable dependiendo de alguna condición, podemos hacerlo de la siguiente forma.

```
fn main() {
    let semaforo = "verde";
    let accion = if semaforo == "rojo" {
        "Frena!"
    } else if semaforo == "amarillo" {
        "Precaución"
    } else {
        "Avanza!"
    };
    println!("{}", accion)
}
```

Debido a que estamos asignando un valor a la variable `accion`, debemos retornar el mismo tipo de dato en todas las ramas. Adicionalmente, al final de la expresión debemos añadir un punto y coma `;` para señalar el fin de la declaración de la variable.

## Instrucción `match`

La instrucción `match` se utiliza para comparar un valor con una serie de elementos y ejecutar un bloque de código basado en la primera ocurrencia. Podemos usar el ejemplo anterior y re-escribirlo usando `match`. 

```
fn main() {
    let semaforo = "verde";
    let accion = match semaforo {
        "rojo" => "Frena!",
        "amarillo" => "Precaución",
        _ => "Avanza!",
    };
    println!("{}", accion)
}
```

Para declararlo, escribimos la palabra clave `match` seguida de una variable o valor. Dentro de los corchetes `{}` y del lado izquierdo colocamos todos los posibles valores que puede tomar la variable (brazos), y del lado derecho el código a ejecutar en caso de que el patrón coincida.

Es importante tener en cuenta que tenemos que cubrir todos los posibles valores que puede tomar la variable o el compilador arrojará un error. Para ello, en el ejemplo anterior utilizamos el `brazo por defecto`, que cubre todas las demás opciones sin necesidad de escribirlas en caso de que ningun otro brazo haya sido igualado.

## Blucles

### `loop`
El bucle más simple en Rust se define con la palabra clave `loop` y lo que hace es ejecutar el código dentro de los corchetes `{}` hasta que explícitamente usemos la palabra clave `break`.

```
fn main() {
    let mut num = 0;
    loop {
        println!("numero dentro del loop: {}", num);
        num += 1;
        if num == 7 {
            break;
        }
    }
            println!("numero afuera del loop: {}", num);
}

----- Standard Output -----

numero dentro del loop: 0
numero dentro del loop: 1
numero dentro del loop: 2
numero dentro del loop: 3
numero dentro del loop: 4
numero dentro del loop: 5
numero dentro del loop: 6
numero afuera del loop: 7
```

Si tenemos varios bucles encadenados, la palabra clave `break` terminará el bucle más interno. Se pueden etiquetar los bucles con un nombre para explícitamente romperlos y evitar ambiguedades.

```
fn main() {
    'exterior: loop {
        println!("dentro del bucle exterior");
        'interior: loop {
            println!("dentro del bucle exterior");
            break 'exterior;
        }
    }
}

----- Standard Output -----

dentro del bucle exterior
dentro del bucle exterior
```

Los bucles pueden devolver un valor y se éste a su vez se puede asignar a variables.

### `for`
Otro tipo de bucles son los `for loops`. Se usan para iterar sobre colecciones como son vectores o matrices. 

```
fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    for numero in numeros {
        println!("{numero}")
    }
}

----- Standard Output -----

1
2
3
4
5
```

### `while`

Los bucles `while` ejecutarán el código entre llaves `{}` siempre y cuando la condición a evaluar sea verdad. Para definirlos usamos la palabra clave `while` seguida de la condición a evaluar.

```
fn main() {
    let mut edad = 15;
    while edad < 18 {
        println!("Con {} años eres menor de edad", edad);
        edad += 1;
    }
    println!("Tienes {} años", edad)
}

----- Standard Output -----

Con 15 años eres menor de edad
Con 16 años eres menor de edad
Con 17 años eres menor de edad
Tienes 18 años
```

Cualquiera de los tres tipos de bucle mencionados se puede terminar usando la plabra clave `break` dentro de el bloque de ejecución.