# Ejercicio: Exportación de Datos de Clientes a CSV desde SQLite

## Objetivo
Desarrollar un programa en Rust que conecte a una base de datos SQLite, extraiga todos los datos de la tabla Customer y los exporte a un archivo CSV.

## Requisitos

1. Conexión a la Base de Datos:
   - El programa debe conectarse a una base de datos SQLite llamada "clientes.db".
   - Si la base de datos no existe, el programa debe manejar este error apropiadamente.

2. Consulta de Datos:
   - Ejecutar una consulta SQL para seleccionar todos los registros y columnas de la tabla Customer.
   - La estructura de la tabla Customer se asume como sigue:
     * CustomerID (Integer)
     * FirstName (Text)
     * LastName (Text)
     * Email (Text)
     * Phone (Text)

3. Procesamiento de Datos:
   - Almacenar los resultados de la consulta en una estructura de datos apropiada en Rust.

4. Generación del Archivo CSV:
   - Crear un archivo CSV llamado "clientes_export.csv".
   - Escribir los datos de los clientes en el archivo CSV.
   - La primera fila del CSV debe contener los nombres de las columnas.

5. Manejo de Errores:
   - Implementar un manejo de errores robusto para problemas como:
     * Fallo en la conexión a la base de datos.
     * Errores en la ejecución de la consulta SQL.
     * Problemas al escribir en el archivo CSV.

6. Optimización:
   - El programa debe ser capaz de manejar eficientemente grandes cantidades de registros.

## Estructura Sugerida del Programa

1. Función para establecer la conexión con la base de datos SQLite.
2. Función para ejecutar la consulta SQL y recuperar los datos.
3. Función para procesar los datos recuperados y convertirlos al formato adecuado para CSV.
4. Función para escribir los datos en un archivo CSV.
5. Función principal que orqueste todo el proceso y maneje los errores.

## Consideraciones Adicionales

- Usar la crate `rusqlite` para la conexión y consultas a SQLite.
- Utilizar la crate `csv` para la generación del archivo CSV.
- Implementar logging para registrar el progreso y cualquier error.

## Resultado Esperado

Al ejecutar el programa, debe:
1. Conectarse exitosamente a la base de datos.
2. Extraer todos los datos de la tabla Customer.
3. Generar un archivo CSV con los datos extraídos.
4. Manejar y reportar cualquier error que ocurra durante el proceso.
5. Mostrar un mensaje de éxito al completar la operación, indicando el número de registros exportados.

Este ejercicio combina varios aspectos importantes de la programación en Rust, incluyendo el manejo de bases de datos, procesamiento de datos, operaciones de archivo, y manejo de errores, proporcionando una buena práctica para desarrolladores que buscan mejorar sus habilidades en Rust y en el manejo de datos.
