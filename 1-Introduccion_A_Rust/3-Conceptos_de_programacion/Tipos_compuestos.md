## Tipos Compuestos: agrupa múltiples valores en un único tipo.
Rust tiene dos tipos compuestos primitivos: Tupla y Array
	----------------------------------------------------------------
### Tipo Tupla: agrupa un cierto número de valores con variedad de tipos en un Tipo Compuesto. Tienen una longitud fija, una vez declaradas, no pueden cambiar su tamaño.
		- EJEMPLOS:
		- Definir una tupla y asignarle valores
		- Extraer y mostrar valores de una tupla mediante pattern matching
		- Extraer y mostrar valores de una tupla mediante desestrucuturación

----------------------------------------------------------------
### Tipo Array (Matriz) y cómo acceder a sus elementos
		- EJEMPLOS:
		- Definir una matriz y asignarle valores numéricos y string
		- Definir una variable con un tipo para sus elementos
		- Acceder a elementos de la matriz
		- Acceso inválido a elementos de una matriz (fuera de rango)
  
  ----------------------------------------------------------------
		- Ver también en SHDF (p. 17)
		- Cita: Vectores y HashMap (se ven en el capítulo 8 del Rust Book)
		- Revisa los tipos: bool, char, enteros y float
			- Esto creo que conviene incluirlo aquí:

### Operadores Aritméticos: añade la Exponenciación, los métodos matemáticos (no son funciones) para Float, las Constantes
		- Al final, explica las constantes y el mecanismo del casting (conversión) entre tipos, mediante parse, los Alias de los tipos de datos, el Tamaño y el Tipo de las Variables.
		
---------------------------------
			- VER LOS EJEMPLOS EN EL LIBRO SHDF, p.25:
### Potencia de un entero
			- Exponenciación: Todos los tipos enteros disponen de un método: pow()		
			- EJEMPLOS:
			- Potencia máxima y mínima de enteros y de floats	

### Métodos matemáticos de los tipos float
			- Tabla con algunos métodos
			- Código de ejemplo
			- Definir una función que usa métodos para float
			- Ejemplo de una closure

### Constantes
			- Ejemplo de definición de constantes

### Casting entre tipos:
			- Ejemplo de casting entre valores numéricos y booleanos

### Alias (mediante: type):
			- Ejemplo de alias

### Tamaño y tipo de las variables
			- Ejemplo de uso de funciones que devuelven el tamaño del tipo
			- Ejemplo de uso de funciones que devuelven el nombre del tipo <- SEGUIR DESDE P. 29
--------------------------------------------------------------------------------
