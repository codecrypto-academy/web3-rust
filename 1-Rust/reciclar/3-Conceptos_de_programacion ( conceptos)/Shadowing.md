## Shadowing
En Rust, podemos declarar una nueva variable con el mismo nombre que una variable que hemos usado previamente.
Los Rustaceans dicen que la primera variable está eclipsada/ a la sombra (shadowed) por la segunda, lo que significa que esa segunda variable es lo que el compilador va a ver cuando uses la variable con ese nombre.

Efectivamente, la segunda variable eclipsa a la primera tomando cualquier uso del nombre de variable para sí mismo, hasta que esa variable es eclipsada de nuevo, o se termina su ámbito (scope).

Podemos eclipsar una variable usando el mismo nombre de variable y repitiendo el uso de la keyword `let`, lo veremos en el siguiente ejemplo:

 ----------------------------------------------------------------
### EJEMPLO: Asignacion inicial de valor a una variable y nueva asignación en un párrafo interior. 
El siguiente programa, lo primero vincula `x` al valor 5. Después crea una nueva variable `x` repitiendo `let x =` y tomando el valor original y sumándole 1, con lo cual, el valor de `x` pasa a ser 6.

Entonces, dentro del ámbito interior creado con las llaves {}, el tercer estamento `let` eclipsa también a  `x` y crea una nueva variable, multiplicando el valor previo por 2 y dándole a `x` un valor de 12. Cuando se termina su ámbito, la ocultación interna finaliza y `x` vuelve a tener un valor de 6. 
```
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```
Cuando ejecutamos el programa, obtendremos el siguiente resultado:
```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

Hacer Shadowing es diferente a marcar la variable con `mut` puesto que conseguiremos un error de compilación si accidentalmente tratamos de reasignar un valor a esta variable sin usar `let`.
Al usar `let` podemos ejecutar algunas transformaciones sobre un valor, pero seguimos teniendo la variable como inmutable una vez que esas transformaciones han concluido.

La otra diferencia entre `mut` y shadowing es que, efectivamente estamos creando una nueva variable cada vez que usamos `let` de nuevo, podemos cambiar el tipo del valor, pero podemos seguir usando el mismo nombre.

 ----------------------------------------------------------------
### EJEMPLO: Al hacer Shadowing, se crea una nueva variable.
Por ejemplo, podemos decir a nuestro programa que pida a un usuario que diga cuantos espacios quiere que haya entre un texto, introduciendo caractéres de espacio y usar esos espacios para obtener un número (de espacios):

```
	let spaces = "   ";
	let spaces = spaces.len();
```

En este caso, la primera variable `spaces` es de tipo string y la segunda variable `spaces` es de tipo numérico. De esta forma, el shadowing nos evita tener que usar nombres diferentes, como `spaces_str` y `spaces_num`. En vez de eso, podemos sencillamente reutilizar el mismo nombre `spaces`.

De cualquier modo, si tratamos de usar `mut` para ello, conseguiremos un error de compilación:

```
    let mut spaces = "   ";
    spaces = spaces.len();
```

El error nos dice que no está permitido mutar el tipo de una variable:
```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`
  |
help: try removing the method call
  |
3 -     spaces = spaces.len();
3 +     spaces = spaces;
  |

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` (bin "variables") due to 1 previous error
```
Y una vez que hemos explorado como trabajan las variables, veamos qué más tipos de datos podemos tener.

---------------------------------------------------------------
