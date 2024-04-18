## Shadowing
- Declaración de una variable que se ha usado previamente y su funcionamiento
- Los Rustaceans dicen que la primera variable está eclipsada/ a la sombra (shadowed) por la segunda

 ----------------------------------------------------------------

### EJEMPLO: Asignacion inicial de valor a una variable y nueva asignación en un párrafo interior. Al salir del bucle, el valor vuelve a ser el que tenía antes de entrar en el bucle.
- EXPLICACION: ¡¡¡PENDIENTE DE REDACTARLO!!!
- Comparacion con mut:
  
- Shadowing es diferente de marcar la variable con mut porque conseguiremos un error de compilación si accidentalmente tratamos de reasignar un valor a esta variable sin usar let.
- Al usar let podemos ejecutar algunas transformaciones sobre un valor, pero seguimos teniendo la variable como inmutable una vez que esas transformaciones han concluido.
- La otra diferencia entre mut y shadowing es que, efectivamente estamos creando una nueva variable cada vez que usamos let de nuevo, podemos cambiar el tipo del valor, pero podemos seguir usando el mismo nombre.

 ----------------------------------------------------------------
### EJEMPLO: Al hacer Shadowing, se crea una nueva variable.
	let spaces = "   ";
	let spaces = spaces.len();
- Podemos decir a nuestro programa que pida a un usuario que diga cuantos espacios quiere que haya entre un texto, introduciendo caractéres de espacio y usar esos espacios para obtener un número (de espacios).
- En este caso, la primera variable spaces es de tipo string y la segunda variable spaces es de tipo numérico. De esta forma, el shadowing nos evita tener que usar nombres diferentes, como spaces_str y spaces_num. En vez de eso, podemos sencillamente reutilizar el mismo nombre spaces. De cualquier modo, si tratamos de usar mut para ello, conseguiremos un error de compilación:
- ERROR: No está permitido mutar el tipo de una variable
		
	----------------------------------------------------------------
