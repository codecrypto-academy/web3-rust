# Definición del Problema: Juego de Adivinanza Numérica

## Objetivo
Desarrollar un programa interactivo que implemente un juego de adivinanza donde el usuario intenta adivinar un número aleatorio generado por el programa.

## Especificaciones

1. Generación del Número Secreto:
   - El programa debe generar aleatoriamente un número entero.
   - El rango del número debe ser entre 1 y 10, ambos inclusive.

2. Interacción con el Usuario:
   - El programa debe solicitar al usuario que introduzca un número por consola.
   - Este proceso se repetirá hasta que el usuario adivine correctamente el número.

3. Validación de Entrada:
   - El programa debe verificar que la entrada del usuario sea un número entero válido.
   - Si la entrada no es válida, se debe informar al usuario y solicitar una nueva entrada.

4. Comparación y Retroalimentación:
   - Después de cada intento válido, el programa debe proporcionar retroalimentación:
     a. Si el número introducido es menor que el número secreto, mostrar "Menor".
     b. Si el número introducido es mayor que el número secreto, mostrar "Mayor".
     c. Si el número introducido es igual al número secreto, mostrar "Ganaste".

5. Finalización del Juego:
   - El juego debe terminar cuando el usuario adivine correctamente el número secreto.
   - Al finalizar, se debe mostrar un mensaje de victoria que incluya el número secreto.

6. Restricciones Adicionales:
   - El programa debe manejar entradas fuera del rango (1-10) adecuadamente, solicitando una nueva entrada.
   - Debe ser capaz de manejar entradas no numéricas sin fallar.

7. Interfaz de Usuario:
   - Las instrucciones y mensajes deben ser claros y fáciles de entender.
   - La interfaz debe ser por consola, sin requerir interfaz gráfica.

## Indicaciones
1. Repasar el parse
2. Instalar rand = "0.8.5" en las dependencias
## Flujo del Programa
1. Inicio del programa y generación del número secreto.
2. Solicitud de entrada al usuario.
3. Validación de la entrada.
4. Comparación con el número secreto y provisión de retroalimentación.
5. Repetición de los pasos 2-4 hasta que se adivine el número.
6. Mensaje de victoria y finalización del programa.

## Consideraciones de Diseño
- El programa debe ser robusto y capaz de manejar entradas inesperadas sin fallar.
- La lógica del juego debe ser clara y fácil de seguir.
- Se debe considerar la posibilidad de futuras expansiones, como permitir múltiples rondas o ajustar el rango de números.

Esta definición proporciona una base clara para implementar el juego de adivinanza, estableciendo los requisitos y el comportamiento esperado del programa.