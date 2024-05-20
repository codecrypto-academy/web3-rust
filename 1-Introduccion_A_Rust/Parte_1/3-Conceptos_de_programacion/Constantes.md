## Constantes
Las Constantes son valores que están asignados a un nombre y no está permitido cambiarlos.
Veamos sus diferencias:

### Declaración de Constantes:
Una constante se declara usando la Keyword: const, en vez de: let
Su tipo debe de ser definido
Pueden declararse constantes en cualquier ámbito, incluso globalmente, lo que permite que puedan usarse en cualquier parte del código que las pueda necesitar
Las constantes sólo pueden asignarse a una expresión constante, no a ningún resultado de un valor que sólo se pueda calcular en tiempo de ejecución.
```
Convencion para nombrar constantes: Todo en Mayúsculas con guión bajo:
THREE_HOURS_IN_SECONDS
```
##### No está permitido usar mut con las constantes: Las constantes son SIEMPRE INMUTABLES
----------------------------------------------------------------
### EJEMPLO: Asignación de valor a una constante, mediante expresión numérica
### EJERCICIO: Poner 3 horas en segundos
```
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
El código incluido en un programa
```
fn main() {
// Podemos asignar un valor a una constante mediante una expresión que permita que se entienda mejor
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);
}
```
----------------------------------------------------------------
### Ventajas del uso de Constantes
Nombrar valores Harcoded usados a lo largo de todo el programa como constantes es muy conveniente para el significado de ese valor, de cara a futuros mantenedores del código.
También ayuda el tener todos los valores Harcoded en un único punto, si en algún momento es necesario cambiar esos valores.

----------------------------------------------------------------
### Ejemplos de constantes:
Máximo número de puntos que un jugador de un juego puede conseguir, o la velocidad de la luz.

----------------------------------------------------------------
- VER TAMBIEN: CONSTANT EVALUATION, EN RUST REFERENCE:

https://doc.rust-lang.org/stable/reference/const_eval.html
