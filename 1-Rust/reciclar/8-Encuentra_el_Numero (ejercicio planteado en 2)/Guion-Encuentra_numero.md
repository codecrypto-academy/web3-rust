## 2º) PROGRAMANDO: ENCUENTRA EL NÚMERO
### 2-Programando-Encuentra_el_numero-Contenidos01.txt


2º) PROGRAMANDO: ENCUENTRA EL NÚMERO
- INTRODUCCION:
Vamos empezar a conocer Rust creando juntos un proyecto, donde vamos a practicar con los fundamentos del lenguaje.
	- NUEVOS CONCEPTOS: let, match, funciones, crates externos...

	- Vamos a implementar un problema deprogramación básico: Un juego para encontrar un número. Veamos cómo funciona:
	- El programa genera un nº entero aleatorio entre 1 y 100
	- Entonces, pregunta al jugador que introduzca el número que cree que es
	- El programa indicará si el nº introducido en mayor o menor que el nº secreto
	- Si le nº introducido es correcto, el juego imprime un mensaje de felicitación y termina

----------------------------------------------------
- CREAR UN NUEVO PROYECTO:
	- Nuevo proyecto con Cargo:
	Para crear un nuevo proyecto, ves al directorio de proyectos y crea un nuevo proyecto en Cargo:
$ cargo new guessing_game
$ cd guessing_game

	El 1º comando, cargo new toma el nombre del proyecto (guessing_game) como el 1º argumento. Después cambiamos al directorio del proyecto

	- Vamos a repasar el archivo generado: Cargo.toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
	# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
	[dependencies]???
	- Programa hello world, generado por defecto??? ¡PROBARLO!
	- Comprobamos el fichero creado: src/main.rs
fn main() {
    println!("Hello, world!");
}
	- Compilar y ejecutar, con: cargo run
	- VER EL RESULTADO DE LA EJECUCIÓN
	- El comando run es útil cuando tienes que hacer cambios rápidos en un programa, como vamos a haer en este proyecto, probando cada interacción antes de cambiar a la siguiente.
	- Abre de nuevo el archivo: src/main.rs. Vamos a escribir todo el código de nuestro programa en él.

- PROCESANDO UNA ENTRADA (GUESS)
	- La primera parte del programa va a pedir una entrada de usuario, va a procesar esa entrada y va a comprobar que la entrada está en el formato requerido
	- Para empezar, permitamos al jugador que introduzca su apuesta. Copia el código del listado 2-1 en el listado siguiente (listing 2-1):
	--------------------
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
	--------------------
	- Este código contiene mucha información. Vamos a hacer un análisis linea a linea.
	
	- Standard Library: std::io;
	- Para poder obtener datos de una entrada de usuario y poder imprimir el resultado en la salida, necesitamos añadir al programa la librería de entrada/salida io, que proviene de la Librería Estandard, que se conoce como std:
use std::io;

	- PRELUDE: Por defecto, Rust posee un conjunto de elementos definidos en la Librería Estandard que se importan en cada programa. Este conjunto se llama Prelude y puedes saber más sobre ello en la Documentación de la Librería Estandard:
https://doc.rust-lang.org/stable/std/prelude/index.html
	- EJERCICIO: REPASAR la Librería Estandard
	- Si un tipo que quieres usar no está en Prelude, tienes que traerlo explicitamente al programa, mediante la declaración: use.
	- Al usar la librería std::io tenemos a nuestra disposición un cierto número de características utilizables, incluída la capacidad de aceptar la entrada de datos de un usuario.

	- La función main es el punto de entrada al programa:  fn main() {}
	- La sintáxis de fn declara una nueva función. Los paréntesis indican que no hay ningún parámetro, y la llave { da comienzo al cuerpo de la función
	
	- println! Se trata de una Macro que imprime una cadena de caracteres en la pantalla
    println!("Guess the number!");
    println!("Please input your guess.");
	- Este código imprime un texto informativo que dice lo que hace el programa y solicita la entrada del usuario
----------------------------------------------------	
	- ALMACENANDO VALORES CON VARIABLES
	- Lo siguiente es crear una Variable para almacenar la entrada de usuario:
let mut guess = String::new();
	- Usamos la declaración let para crear una variable. Por ejemplo:
let apples = 5;
	- Esta línea crea una nueva variable, apples y la vincula con el valor 5
	- En Rust, las variables son inmutables por defecto, una vez que se le da un valor, el valor no cambia.
	- Para hacer que una variable pueda cambiar (mutar), añadiremos mut antes del nombre de la variable
let apples = 5; // immutable
let mut bananas = 5; // mutable
	- También vemos al final del código que se han incluído un comentario, precedido por: //
	- Ya sabemos que let mut guess añade una nueva variable mutable, llamada guess.
	- El signo = le dice a Rust que queremos vincular algo a la variable.
	- A la derecha del signo igual está el valor que guess está obligado a albergar, que es el resultado de llamar a la función String::new, que nos devuelve una nueva instancia de una String. String es un Tipo String, incluido en la Librería Estandard y que es un fragmento de texto, codificado en UTF-8 (Growable).
	- La sintaxis :: en la línea ::new indica que new es una función asociada del tipo String. Una función asociada es una función que está implementada en un tipo, en este caso String. Esta función new crea una nueva string vacía.
	- Nos encontraremos una función new en muchos tipos, porque es una manera corriente para una función que crea un nuevo valor de una clase en concreto.
	- Resumiendo, la linea: let mut guess = String::new(); crea una variable mutable que esta vinculada actualmente a una nueva instancia vacía de String

----------------------------------------------------
	- RECIBIENDO LA ENTRADA DEL USUARIO
	- Recuerda que, al usar: use std::io; en la primera línea del programa, habíamos incluido la funcionalidad de I/O desde la Librería Estandard.
	- Ahora, vamos a llamar a la función stdin, perteneciente al módulo io, que nos va a permitir manejar las entradas del usuario:
    io::stdin()
        .read_line(&mut guess)
	- Ten en cuenta que si no hubiéramos importado la librería io de esta manera al principio del programa, aún podríamos invocarla de esta manera: std::io::stdin
	- La función stdin devuelve una instancia de std::io::stdin, que es un tipo que representa un gestor (handle) de la entrada estandard de tu terminal.
	- La siguiente linea: .read_line(&mut guess) invoca el método read_line de la Entrada Estandard para conseguir los datos que introduce el usuario. El trabajo completo de read_line es tomar cualquier cosa que el usuario teclée en una entrada estándard y añadirlo a una string (sin sobreescribir su contenido). De esta forma, pasaremos esta string como un argumento. Este argumento necesita ser mutable, de este modo el método puede cambiar el contenido de la string.
	- El símbolo & indica que este argumento es una referencia (Reference), lo que te da un modo de mantener el acceso desde múltiples partes de tu código a un dato en concreto, sin necesidad de tener que copiar ese dato en la memoria múltiples veces.
	- Las References son una característica avanzada y una de las mayores ventajas que ofrece Rust es la forma fácil y segura en la que se pueden utilizar las referencias.
	- No necesitas saber mucho más detalles para poder acabar el programa. por ahora, todo lo que necesitas saber es que, al igual que las variables, las referencias también son inmutables por defecto.
	- Por tanto, necesitas escribir &mut guess, más que &guess para hacerlo mutable.

----------------------------------------------------
	- GESTIONANDO POTENCIALES FALLOS CON: Result
	- Seguiremos trabajando en esta línea de código. Vamos a comentar una tercera línea de texto, pero ten en cuenta que todavía forma parte de una única linea lógica de código. Lo siguiente es este método:
        .expect("Failed to read line");
	- Que podríamos haberlo escrito como:
io::stdin().read_line(&mut guess).expect("Failed to read line");
	- Pero una única linea de código larga es difícil de leer, así que lo mejor es dividirla. A menudo, lo más inteligente es introducir una nueva línea y otro espacio en blanco para romper las líneas largas, cuando invoques un método con la sintaxis de: .method_name()
	- Veamos lo que hace. Como dijimos antes, read_line pone cualquier cosa que el usuario introduzca en una string que pasaremos, pero además devuelve un valor Result. Result es una Enumeration, a menudo llamada enum, que es un tipo que puede estar en uno de muchos estados posibles. A cada estado posible lo llamamos variant.
	- El propósito de esos tipos Result es codificar información para el manejo de los errores.
	- Las variantes de Result son: Ok y Err. Ok significa que la operación ha sido exitosa y dentro de Ok está el valor generado correctamente.
	- La variante Err significa que la operación ha fallado y Err contiene las información sobre cómo o porqué la operación ha fallado.
	-Los valores del tipo Result, como los valores de cualquier tipo, tienen métodos definidos en ellos. Una instancia de Result tiene un método expect que tú puedes invocar. 
	- Si la instancia de Result es un valor Ok, expect tomará el valor de retorno que Ok está manteniendo y te devolverá ese valor, para que puedas usarlo. En este caso, el valor es el número de bytes en la entrada de usuario.
	- Si no invocaras expect, el programa compilará, pero obtendrás un aviso:
	----------------------------------------------------
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
	----------------------------------------------------
	- Rust te advierte de que no has usado el valor devuelto por Result desde read_line, y te indica que el programa no va a ser capaza de manejar un posible error.
	- La manera correcta de suprimir este aviso es escribiendo código de manejo de error.
	-En nuestro caso, lo que queremos es que el programa se rompa cuando ocurra un problema, así que podemos usar expect.
	
----------------------------------------------------
	- DEVOLVIENDO VALORES CON LOS PLACEHOLDERS: println!
	- Aparte de la llave del final } aún tenemos que comentar una línea más de código:
    println!("You guessed: {guess}");

	- Esta línea imprime la string que ahora contiene la entrada del usuario.
	- El conjunto de llaves {} es un Placeholder, piensa en que {} son como las pinzas de un cangrejo, que mantiene un valor en su sitio. Cuando queremos imprimir el valor de una variable, el nombre de la variable puede ir dentro de las llaves {}.
	- Cuando imprimimos el resultado de evaluar una expresión, colocaremos las dos llaves vacías {} dentro del la string de formato y mantendremos el formato de string con una lista de expresiones separadas por comas para imprimir en cada placeholder {} vacio en el mismo orden.
	- Imprimir una variable y el resultado de una expresión en una llamada a println! sería algo así:
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
	- Este código imprimirá: x = 5 y y + 2 = 12.
	
----------------------------------------------------
	- PROBANDO (TESTING) LA PRIMERA PARTE
	- Ejecutar el código con cargo run e introducir un número (guess)
	- Se imprime el número por pantalla

$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6


----------------------------------------------------
- GENERANDO UN NUMERO SERETO
	- Tenemos que generar un nº secreto, distinto cada vez que se ejecuta el juego
	- Usaremos el crate: rand

	----------------------------------------------------
	- USANDO UN CRATE PARA CONSEGUIR UNA MAYOR FUNCIONALIDAD
	- Tipos de crates:
		- binary crate: por ejemplo el código de nuestro programa
		- library crate: contiene código que va a ser usado en otros programas, pero que no puede ejecutarse por si solo
	- Cargo brilla en la gestión de los crates externos
	- Antes de poder usar rand dentro de nuestro código, tenemos que modificar el archivo: Cargo.toml e incluir el crate rand como una dependencia, debajo de [dependencies], con su correspondiente nº de versión
[dependencies]
rand = "0.8.5"

$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
  Downloaded libc v0.2.127
  Downloaded getrandom v0.2.7
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.16
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.3
   Compiling libc v0.2.127
   Compiling getrandom v0.2.7
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s

$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs


	- PENDIENTE: REPASARLO EN DETALLE
	----------------------------------------------------
	- ASEGURANDONOS DE BUILDS REPRODUCIBLES CON EL FICHERO Cargo.lock
		- REPASAR LAS DEPENDENCIAS Y LAS VERSIONES DE LOS CRATES
	----------------------------------------------------
	- ACTUALIZANDO UN CRATE PARA CONSEGUIR UNA NUEVA VERSION
	- Cargo update y como configurarlo 
	- DUDA: ¿Cuando actualizar de v0.8.5 a v0.9.0?
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6

[dependencies]
rand = "0.9.0"


----------------------------------------------------
- GENERANDO UN NUMERO ALEATORIO
	- Usando rand y su explicación
	- Generando la documentación de un crate con cargo
	- LISTADO 2-3:
	----------------------------------------------------
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

	----------------------------------------------------
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5



----------------------------------------------------
- COMPARANDO LA ENTRADA (GUESS) CON EL NÚMERO SECRETO
	- Usando la librería estandard cmp y su tipo Ordering, para comparaciones
	- Variantes: Less, Greater y Equal
	- match, arms y su explicación
	- Funcionamiento de match para un valor en concreto
	- ERROR: Type mismatch
	- Conversión de la string con el valor de entrada tecleado, a un valor numérico
	- Se hace usando el shadowing, manteniendo el mismo nombe de la variable: guess
	- Explicación del funcionamiento: quitar espacios en blanco, necesidad de añadir return y de quitarlo de la string
	- DUDA: ¿Por qué no usa un entero de menor tamaño de 32 bits?
	- ATENCIÓN: La comparación debe de ser entre números del mismo tipo: u32
	- Resultado de la comparación y su tratamiento
	- LISTADO 2-4:

	----------------------------------------------------
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}


	----------------------------------------------------
$ cargo build
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |                 |
   |                 arguments to this function are incorrect
   |
   = note: expected reference `&String`
              found reference `&{integer}`
note: associated function defined here
  --> /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/cmp.rs:783:8

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error

	----------------------------------------------------
	
	----------------------------------------------------
    // --snip--

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

	----------------------------------------------------

	----------------------------------------------------
let guess: u32 = guess.trim().parse().expect("Please type a number!");

	----------------------------------------------------

	----------------------------------------------------
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!



----------------------------------------------------
- PERMITIENDO MÚLTIPLES ENTRADAS (GUESS) MEDIANTE BUCLES
	- Usando loop
	- Forzar la salida de un bucle con ctrl-c y con una respuesta no numérica

	----------------------------------------------------
    // --snip--

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}

	----------------------------------------------------

	----------------------------------------------------
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

	- SALIENDO DEL BUCLE TRAS UN RESULTADO CORRECTO
	- añadiendo el comando break, para salir si se acierta el nº


	----------------------------------------------------
        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


----------------------------------------------------
	- GESTIONANDO ENTRADAS INVALIDAS
	- El programa puede detectar si el usuario ha introducido un valor no numérico
	- y te da la opción de volver a introducir otro valor de nuevo
	- PENDIENTE: REPASAR ESTA PARTE
	- El patrón Err(_) contiene _ que recoge cualquier valor (catchall)
	- DUDA: REPASAR EL FUNCIONAMIENTO DE ESE BUCLE, SI SALE DEL BUCLE CON UN VALOR CORRECTO Y SI VUELVE A PEDIR UNA NUEVA ENTRADA POR EL TECLADO, EN CASO DE ERROR
	- SI, PARECE QUE SI SE PONE UN VALOR NO NUMÉRICO, TE VUELVEA PEDIR UN NÚMERO

	----------------------------------------------------
        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // --snip--

	----------------------------------------------------

$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!



----------------------------------------------------	
	- PROGRAMA DEFINITIVO
	- Quitando la impresión del nº secreto
	- NOTA: El código se puede ejecutar en Play Rust
	- La ejecución aparece en la parte inferior y para poder mandar los números hay que hacerlo desde la última línea y pulsar el botón de SEND, o pulsar sobre RETURN
	- Si se pulsa ctrl-c, se sale de la ejecución

	----------------------------------------------------
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

	----------------------------------------------------
	
----------------------------------------------------
- REPASO:
	- NUEVOS CONCEPTOS: let, match, funciones, crates externos...
	- Capítulo 3: Conceptos comunes...
	- Capítulo 4: Ppropiedad
	- Capítulo 5: Structs
	- Capítulo 6: Funcionamiento de enum



