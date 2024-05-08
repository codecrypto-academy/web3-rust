## Tests Automatizados

Rust permite ejecutar tests de forma nativa. 

Para definir un test, usamos la palabra clave o atributo `#[test]` para indicarle al compilador que la siguiente función será un test a ejecutar. Este será ignorado en compilaciones normales pero inluido y llamado automáticamente cuando corremos `cargo test` desde cualquier directorio dentro del arbol del proyecto.

En la siguiente línea definimos la función como normalmente haríamos, usando la palabra clave `fn`, seguida de su nombre y paréntesis `()`.

```
#[test]
fn test_numero_es_par() { 
    let numero = 16;
    assert_eq!(numero % 2, 0);
    assert!((numero + 1) % 2 != 0);
}
```

```
----- Standard Output -----

running 1 test
test test_numero_es_par ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Si definimos varias funciones de test a lo largo del código, si corramos `cargo test`, vamos a ver como automáticamente se van a correr todas una detras de otra.