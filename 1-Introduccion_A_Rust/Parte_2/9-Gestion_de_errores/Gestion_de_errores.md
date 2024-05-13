## Gestión de errores

Funciones que hacen cualquier cosa que pueda fallar, como parsear algun dato, deberían retornar un tipo `Result` que puede ser 
cualquiera de las siguientes viariantes:

A value written `Ok(v)`, indicating that the parse succeeded and `v` is the value produced
A value written `Err(e)`, indicating that the parse failed and `e` is an error value explaining why

A diferencia de la mayoria de los lenguajes modernos de programación, Rust no tiene excepciones, es decir, todos los errores se deben manejar bien sea usando un `Result` o `panic`.