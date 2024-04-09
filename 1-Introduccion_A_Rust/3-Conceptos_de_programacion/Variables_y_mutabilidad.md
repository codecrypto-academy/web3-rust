## Variables y mutabilidad
### Introducción:

En Rust, las variables son inmutables por defecto, pero Rust te ofrece la posibilidad de hacerlas mutables

----------------------------------------------------------------
## 1º) EJEMPLO: PROYECTO: variables (con Cargo)
ASIGNACION DE DOS VALORES A LA MISMA VARIABLE -> ERROR

error[E0384]: cannot assign twice to immutable variable `x`

  - COMENTARIOS AL ERROR OBTENIDO: Errores de compilación
    
		- Se comentan los errores recibidos
    
		- El compilador da la siguiente sugerencia:
help: consider making this binding mutable: `mut x`

----------------------------------------------------------------
## 2º) EJEMPLO: ASIGNACION DE DOS VALORES A LA MISMA VARIABLE, PERO CON: mut -> FUNCIONA CORRECTAMENTE	
		
		- NOTA: ¿En qué casos puede tener sentido hacer uso de la mutabilidad en las variables?
