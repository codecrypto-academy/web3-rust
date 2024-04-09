## Tipos de Datos:
Cada valor en Rust posee un cierto tipo de dato, que le dice a Rust que clase de datos se están especificando, de este modo sabe como trabajar con esos datos.

Rust: Se trata de un Lenguaje de Tipado Estático, es decir que Rust DEBE conocer el tipo de todas las variables en el momento de compilar el programa. Normalmente, el compilador puede inferir qué tipo queremos usar, basándose en el valor y cómo lo usamos. 

En aquellos casos en los que son posibles muchos valores, como en la conversión de tipo string a tipo número, usando parse, deberemos añadir una anotación de tipo, por ejemplo:
### EJEMPLO DE NOTACION DE TIPO:
		
	let guess: u32 = "42".parse().expect("Not a number!");	
  - Tipos Escalares: un valor simple

### PRACTICA: Especificar un tipo concreto, cuando puede haber multiples posibilidades

	fn main() {
      let guess: u32 = "42".parse().expect("Not a number!");
      println!("{guess}!");
    }

### NOTA: ATENCION AL NUMERO DE ESPACIOS EN LOS FRAGMENTOS DE CODIGO
  - Cambiar "42" por "hola" ¿Qué error va a dar?

----------------------------------------------------------------
### Tipos Escalares:
Un Tipo Escalar representa un único valor. 

Al igual que en muchos otros lenguajes de programación, en Rust existen cuatro tipos primarios: Enteros, coma flotante, buleanos y caracteres

----------------------------------------------------------------
### Tipos Enteros:
Son números sin componente fraccionario.

En el caso del tipo u32, esta declaración de tipos indica que el valor que está asociado con este tipo debe de ser un entero sin signo (por eso u32 empieza por u, en vez de por i, como en i32) y que ocupa 32 bits de espacio.

Con signo (comienzan por una: i) y sin signo (comienzan por una: u)
   
Según su tamaño, hay seis tipos, de 8, 16, 32, 64 y 128 bit y uno más, que depende de la arquitectura del procesador: isize y usize (Ver la tabla 3-1)

Se pueden usar cualquiera de las combinaciones posibles de la tabla (12).

### EJERCICIO: repasar los detalles de la explicación ¡¡¡PERO LO MISMO SE PUEDE DECIR EN EL VIDEO TAMBIÉN!!!
			- Los tamaños de los tipos isize y usize dependen de las características de la arquitectura del ordenador que estemos usando, 64 bits o 32 bits.

### Literales enteros en Rust:
Pueden escribirse literales enteros en cualquiera de las maners mostradas en la Tabla 3-2.
Ten en cuenta que a los literales numéricos que pueden tener múltiples tipos numéricos, les está permitido un sufijo de tipo, como 57u8, para designar su tipo.
Los literales numéricos también pueden hacer uso del guión bajo "_" como separador visual, para hacer que el número sea más fácil de leer:
			1_000 tendrá el mismo valor que 1000
Literales enteros en Rust (Ver la tabla 3-2)

Maxint...

Hay funciones estandard que te dan los valores máximos de cada tipo:
				
Maneras de escribirlos:

----------------------------------------------------------------
### Desbordamiento de enteros: y como gestionarlo
			- Repasar formas de tratar el desbordamiento
			
----------------------------------------------------------------
