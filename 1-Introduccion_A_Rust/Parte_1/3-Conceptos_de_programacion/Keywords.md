## Keywords (Palabras Clave): Palabras reservadas del lenguaje Rust
		NOTA: Resolver cómo se van a mostrar, si una matriz con varias columnas, o en la propia pantalla del Apéndice A.
- El lenguaje Rust posee un conjunto de palabras clave, las Keywords, que están reservadas para ser usadas exclusivamente por el lenguaje, al igual que ocurre con muchos otros lenguajes de programación. Ten en cuenta que no puedes usar esas palabras como nombres de variables o de funciones.
- Muchas de esas Keywords tienen significados especiales y vas a usarlas para hacer varias tareas en tus programas de Rust. Algunas de ellas no tienen actualmente ninguna funcionalidad asociada, pero también han sido reservadas por una funcionalidad que puede ser añadida a Rust en el futuro.

### Listado de Keywords:
as - perform primitive casting, disambiguate the specific trait containing an item, or rename items in use statements

async - return a Future instead of blocking the current thread

await - suspend execution until the result of a Future is ready

break - exit a loop immediately

const - define constant items or constant raw pointers

continue - continue to the next loop iteration

crate - in a module path, refers to the crate root

dyn - dynamic dispatch to a trait object

else - fallback for if and if let control flow constructs

enum - define an enumeration

extern - link an external function or variable

false - Boolean false literal

fn - define a function or the function pointer type

for - loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime

if - branch based on the result of a conditional expression

impl - implement inherent or trait functionality

in - part of for loop syntax

let - bind a variable

loop - loop unconditionally

match - match a value to patterns

mod - define a module

move - make a closure take ownership of all its captures

mut - denote mutability in references, raw pointers, or pattern bindings

pub - denote public visibility in struct fields, impl blocks, or modules

ref - bind by reference

return - return from function

Self - a type alias for the type we are defining or implementing

self - method subject or current module

static - global variable or lifetime lasting the entire program execution

struct - define a structure

super - parent module of the current module

trait - define a trait

true - Boolean true literal

type - define a type alias or associated type

union - define a union; is only a keyword when used in a union declaration

unsafe - denote unsafe code, functions, traits, or implementations

use - bring symbols into scope

where - denote clauses that constrain a type

while - loop conditionally based on the result of an expression

- Puedes encontrar una lista de las Keywords aquí:  Apéndice A

https://doc.rust-lang.org/stable/book/appendix-01-keywords.html

- Existen dos tipos de Keywords actualmente:

  Keywords actualmente en uso, a continuación de cada una encontrarás la descripción de su funcionalidad
			- (nota: faltan enlaces a mas información en cada una de ellas)

  Keywords reservadas para un futuro uso

### Listado de Keywords Reservadas:
abstract

become

box

do

final

macro

override

priv

try

typeof

unsized

virtual

yield

## PRACTICA: Keywords
- Tratar de usar una palabra reservada como variable, funcion...
- Analizar el error que da el compilador
- En la web (Play Rust), si pones una palabra reservada como una variable, aparece en negrita, por ejemplo as, y si le añades un 1 -> as1 ya se muestra sin negrita. El compilador detecta: as como una Keyword
