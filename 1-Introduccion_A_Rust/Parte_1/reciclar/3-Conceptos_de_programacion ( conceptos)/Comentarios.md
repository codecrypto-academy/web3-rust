## Cómo poner comentarios

Yo soy partidario de que colocar comentarios en el código siempre mejora su calidad. No siempre podremos escribir código de manera auto explicativa, sobretodo cuando estamos colaborando con otras personas o trabajamos con código abierto. Además hay programadores que no están acostumbrados a realizar algún tipo de operaciones y siempre es bueno explicar el porqué de ellas.

El compilador de Rust ignora los comentarios que hacemos usando cualquiera de las siguientes palabras clave:
```
// para escribir comentarios en Rust
// se colocan dos barras al principio de la línea
// y el compilador ignorará estas palabras

/*
si queremos escribir en bloque
sin necesidad de colocar dos barras
al principio de cada línea
lo podemos hacer así
*/
```
Luego, si queremos que Rust genere documentación sobre nuestro código podemos usar tres barras para hacer los comentarios. Así cuando definamos una función que se usa en otra parte del código, podremos pasar el ratón por encima y podremos leer qué hace dicha función, mejorando la experiencia del programador.

```
/// esta función calcula el punto medio de dos puntos
/// entrada:  coord_1 = (x1, y1); coord_2 = (x2, y2)
/// salida: punto_medio = (x, y)
fn punto_medio(coord_1: (u8, u8), coord_2: (u8, u8)) -> (u8, u8) {
    ((coord_1.0 + coord_2.0) / 2, (coord_1.1 + coord_2.1) / 2)
}
```