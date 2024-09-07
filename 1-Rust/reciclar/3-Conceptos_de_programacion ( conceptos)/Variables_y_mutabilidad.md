## Variables y mutabilidad
### Introducción:
En Rust, las variables son inmutables por defecto. Es una de las muchas ayudas que te ofrece Rust cuando estás escribiendo código de una manera en la cual te beneficias de la seguridad y la fácil concurrencia que te ofrece este lenguaje.

De cualquier modo, también tienes la opción de hacer que tus variables sean mutables. Exploremos cómo y por qué Rust te alienta a favor de la inmutabilidad y por qué algunas veces puedes querer optar por que sean inmutables.

Cuando una variable es inmutable, una vez que un valor está asociado a un nombre, no puedes cambiar ese valor.

Veamos el siguiente ejemplo:

## 1º) EJEMPLO: PROYECTO: variables
ASIGNACION DE DOS VALORES A LA MISMA VARIABLE -> ERROR
```
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
Al ejecutar el código recibiremos el siguiente: Error de compilación

No puedes asignar dos veces un valor a la variable inmutable 'x'

error[E0384]: cannot assign twice to immutable variable `x`

```
 Compiling playground v0.0.1 (/playground)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `playground` (bin "playground") due to 1 previous error
```
Este ejemplo nos muestra también de qué manera el compilador nos ayuda a encontrar errores en tus programas.
Los errores de compilación pueden resultar frustrantes, pero lo único que significan es que tu programa no es seguro todavía haciendo lo que quieres hacer de este modo. No quiere decir que no seas un buen programador. Incluso los Rustaceans más experimentados todavía pueden conseguir errores de compilación.

Has obtenido el mensaje de error: cannot assign twice to immutable variable 'x' porque has intentado asignar un segundo valor a la variable inmutable `x`.		

El compilador nos da la siguiente sugerencia:
```
help: consider making this binding mutable: `mut x`
```
Es importante que obtengamos errores en tiempo de compilación cuando intentemos cambiar un valor que está designado como inmutable, puesto que esta situación nos puede llevar a obtener errores.

Si una parte de nuestro código opera asumiendo que un valor nunca va a cambiar y en otra parte del código cambiamos este valor, es posible que la primera parte del código no esté haciendo aquello para lo que está diseñado.

La causa de este tipo de errores puede ser difícil de trazar, especialmente cuando la segunda parte del código cambia el valor, sólo algunas veces. El compilador de Rust nos garantiza que cuando definimos que un valor no va a cambiar, el valor realmente no va a cambiar, con lo cual no vamos a tener que trazarlo nosotros mismos para comprobarlo. De este modo, tu código es más fácil de comprender
		
Pero la mutabilidad puede ser muy útil y puede hacer que el código sea más conveniente. A pesar de que las variables son inmutables por defecto, puedes hacerlas mutables añadiendo `mut` delante del nombre de la variable. Y al añadir `mut`, también comunicamos a los futuros lectores de nuestro código que otras partes del código pueden cambiar los valores de esta variable.

----------------------------------------------------------------
## 2º) EJEMPLO: ASIGNACION DE DOS VALORES A LA MISMA VARIABLE
## USANDO mut -> FUNCIONA CORRECTAMENTE
```
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
Al ejecutar el programa, obtendremos lo siguiente:

RESULTADO:
```
 Compiling playground v0.0.1 (/playground)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/playground`
	Standard Output
The value of x is: 5
The value of x is: 6
```
Cuando usamos `mut`, nos está permitido cambiar el valor almacenado en x desde 5 a 6.

Por último, la decisión de usar o no la mutabilidad dependerá de nosotros y de lo que pensemos que sea más claro en cada situación en particular.
