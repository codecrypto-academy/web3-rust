# Ejercicio: Servidor Web con Rocket para API de Clientes

## Objetivo
Desarrollar un servidor web utilizando el framework Rocket en Rust que proporcione una API para acceder a los datos de clientes almacenados en una base de datos SQLite.

## Requisitos

1. Configuración del Proyecto:
   - Crear un nuevo proyecto Rust.
   - Agregar las dependencias necesarias en `Cargo.toml`:
     * rocket
     * rocket_contrib (para el soporte JSON)
     * rusqlite
     * serde (para la serialización/deserialización)

2. Estructura de la Base de Datos:
   - Utilizar una base de datos SQLite llamada "clientes.db".
   - La tabla `Customers` debe tener la siguiente estructura:
     * id (INTEGER PRIMARY KEY)
     * first_name (TEXT)
     * last_name (TEXT)
     * email (TEXT)
     * phone (TEXT)

3. Configuración de Rocket:
   - Implementar la configuración básica de Rocket.
   - Definir una estructura `Customer` que refleje la estructura de la tabla en la base de datos.

4. Implementación de Endpoints:

   a. GET /customers
      - Devuelve un JSON con todos los clientes en la base de datos.
      - Implementar paginación (opcional).

   b. GET /customer/<id>
      - Devuelve un JSON con los datos del cliente especificado por el ID.
      - Si el cliente no existe, devolver un error 404.

5. Conexión a la Base de Datos:
   - Implementar una función para establecer la conexión con la base de datos SQLite.
   - Utilizar `rocket::State` para compartir la conexión entre las rutas.

6. Manejo de Errores:
   - Implementar un manejo de errores robusto.
   - Devolver códigos de estado HTTP apropiados para diferentes situaciones de error.

7. Serialización/Deserialización:
   - Utilizar serde para serializar los datos de Customer a JSON.

8. Documentación:
   - Incluir comentarios explicativos en el código.
   - Proporcionar instrucciones básicas de uso en el README del proyecto.

## Estructura Sugerida del Código

```rust
#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Customer {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}

#[get("/customers")]
fn get_all_customers(/* parámetros */) -> Json<Vec<Customer>> {
    // Implementación
}

#[get("/customer/<id>")]
fn get_customer(id: i32, /* otros parámetros */) -> Option<Json<Customer>> {
    // Implementación
}

#[launch]
fn rocket() -> _ {
    // Configuración de Rocket y rutas
}

// Funciones auxiliares para la conexión a la base de datos y consultas
```

## Pruebas

1. Implementar pruebas unitarias para las funciones principales.
2. Incluir pruebas de integración para los endpoints.

## Consideraciones Adicionales

- Implementar un sistema de logging para registrar las solicitudes y errores.
- Considerar la implementación de un sistema de caché para mejorar el rendimiento.
- Agregar autenticación básica para los endpoints (como un desafío adicional).

## Resultado Esperado

Al ejecutar el servidor:
1. Debe iniciarse correctamente y estar listo para recibir solicitudes HTTP.
2. Los endpoints deben responder con los datos correctos en formato JSON.
3. Debe manejar adecuadamente los errores y casos extremos.
4. El servidor debe ser capaz de manejar múltiples conexiones concurrentes.

Este ejercicio proporciona una excelente oportunidad para practicar el desarrollo web en Rust, el manejo de bases de datos, y la creación de APIs RESTful. Combina varios aspectos importantes del desarrollo backend, incluyendo la gestión de rutas, serialización de datos, y interacción con bases de datos.