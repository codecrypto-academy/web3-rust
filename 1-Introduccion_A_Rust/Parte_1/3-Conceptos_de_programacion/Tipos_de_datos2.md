## Tipos de Datos 2
### Tipos de Coma flotante: f32(Simple Precisión) y f64(Doble Precisión, por defecto actualmente), todos con signo
			- En SHDF: 
			- REPASAR EJEMPLOS
			- Constantes de utilidad en la librería std. específicos para cada tipo (f32 y f64), constantes matemáticas
			- Ver todos los valores en la documentación de la librería std

----------------------------------------------------------------			
### Operaciones con números: exigen que los operandos sean del mismo tipo
			- EN RUST BOOK: Suma, Resta, Multiplicación, División y Resto
			- En: APPENDIX B hay una lista que contiene todos los operadores que provee Rust

   - En SHDF: Operadores aritméticos (los mismos que en Rust Book)
			- REPASAR LA PRECEDENCIA DE LAS OPERACIONES
			- VER LOS EJEMPLOS
			- INCLUYE LA EXPONENCIACIÓN: Métodos pow(), powi(), powf(), 
			- INCLUYE LOS METODOS MATEMATICOS DE LOS TIPOS float, con una tabla (2.3)
			- Son métodos, no funciones

### CASTING (CONVERSIONES) ENTRE TIPOS:
				- NUMERICOS: CON as
				- STRINGS: CON parse
				- NO TODOS SON INTERCAMBIABLES
			- ALIAS DE TIPOS DE DATOS
			- TAMAÑO Y TIPO DE LAS VARIABLES

----------------------------------------------------------------
### El Tipo boleano: DOS VALORES, true y false
			- En SHDF: Operadores lógicos: AND, OR, NOT
	Operadores relacionales o de comparación:
			< > <= >= == !=
			- Habla de precedencia de los operadores, de las condiciones sobre los tipos de datos (Traits)
			- REPASAR EJEMPLOS
			
----------------------------------------------------------------
### El Tipo caracter:
Su tamaño es de 4 bytes y representan un valor escalar Unicode, el rango va de: U+0000 A U'D7FF Y DE `+E00 A U+10FFFF
Se definen usando comillas simples '', en vez de comillas dobles "", que se usan para definir cadenas de caracteres

----------------------------------------------------------------
			- En SHDF: 
### EJERCICIO
			- Pon ejemplos de códigos Unicode en distintos rangos
			- También muestra algunos métodos útiles para el tipo char, como preguntar si es numérico o alfabético, cambiar entre Mayúsculas o Minúsculas
			- Se pueden repasar todos los métodos en la documentación de char:
https://doc.rust-lang.org/stable/std/primitive.char.html

----------------------------------------------------------------
### Tipos de Datos Primitivos: ver todos los que hay en libro de SHDF (p. 16 y 17)
			- En el libro de Rust no hace mención, parece ser
Modo de uso:

			nombre_variable.nombre_metodo()
		
En la documentación de la Libería Estandard de Rust hay un listado con todos los Tipos Primitivos:

https://doc.rust-lang.org/stable/std/index.html#primitives

Además de los Tipos Primitivos, existen otros tipos de datos definidos por el usuario: las Estructuras (struct) y las enumeraciones (enum).
Hay otro tipo de datos: union que están pensados para mantener la compatibilidad con C.

  - Aqui también el libro de SHDF cita:
			- La POO, mediante Traits y Tipos de Datos Genéricos y 
			- Las Closures, que guardan una función anónima. Junto con los iteradores permiten a Rust usar el paradigma de la Programación Funcional.

----------------------------------------------------------------
