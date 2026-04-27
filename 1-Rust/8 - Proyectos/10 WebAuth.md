# WebAuth

Sistema front - back (rust) en el que se abordan los siguientes temas:

1. Autenticación de usuarios:
   - Signup: Registro de nuevos usuarios
     - Validación de campos (email, contraseña, etc.)
     - Almacenamiento seguro de contraseñas (hashing)
   - Signin: Inicio de sesión de usuarios existentes
   - Logout: Cierre de sesión

2. Gestión de perfiles de usuario:
   - Visualización de información del perfil
   - Edición de datos del perfil
   - Cambio de contraseña

3. Seguridad:
   - Validación de correo electrónico
     - Envío de correo de confirmación
     - Verificación del enlace de confirmación
   - Implementación de políticas de contraseñas seguras
     - Longitud mínima
     - Combinación de caracteres (mayúsculas, minúsculas, números, símbolos)
   - Autenticación basada en JWT (JSON Web Tokens)
     - Generación y validación de tokens
     - Manejo de expiración de tokens
   - Sistema de roles y permisos
     - Definición de roles (ej. usuario, administrador)
     - Control de acceso basado en roles

4. Integración de OAuth 2.0:
   - Implementación de flujos de autorización
   - Soporte para proveedores populares (Google, Facebook, GitHub)

5. Frontend:
   - Desarrollo de interfaz de usuario con un framework moderno (ej. Yew, Seed)
   - Formularios de registro e inicio de sesión
   - Página de perfil de usuario
   - Manejo de estados de autenticación en el cliente

6. Backend:
   - API RESTful con Actix-web o Rocket
   - Middleware de autenticación
   - Manejo de sesiones
   - Integración con base de datos (ej. PostgreSQL con Diesel ORM)

7. Seguridad adicional:
   - Implementación de HTTPS
   - Protección contra ataques comunes (CSRF, XSS, inyección SQL)
   - Rate limiting para prevenir ataques de fuerza bruta

8. Pruebas:
   - Pruebas unitarias para lógica de autenticación
   - Pruebas de integración para flujos de usuario
   - Pruebas de seguridad
