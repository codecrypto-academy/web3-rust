# Operadores Lógicos y Relacionales en Rust

## Operadores Lógicos

Rust proporciona tres operadores lógicos principales:

| Operador | Descripción | Ejemplo |
|----------|-------------|---------|
| `&&`     | AND lógico  | `a && b` |
| `\|\|`   | OR lógico   | `a \|\| b` |
| `!`      | NOT lógico  | `!a` |

### Ejemplos:

```rust
let a = true;
let b = false;

println!("a && b = {}", a && b);  // false
println!("a || b = {}", a || b);  // true
println!("!a = {}", !a);          // false
```

### Cortocircuito:
- `&&` y `||` utilizan evaluación de cortocircuito.
- `a && b`: si `a` es `false`, `b` no se evalúa.
- `a || b`: si `a` es `true`, `b` no se evalúa.

## Operadores Relacionales

Rust ofrece seis operadores relacionales principales:

| Operador | Descripción | Ejemplo |
|----------|-------------|---------|
| `==`     | Igual a     | `a == b` |
| `!=`     | No igual a  | `a != b` |
| `<`      | Menor que   | `a < b` |
| `>`      | Mayor que   | `a > b` |
| `<=`     | Menor o igual que | `a <= b` |
| `>=`     | Mayor o igual que | `a >= b` |

### Ejemplos:

```rust
let x = 5;
let y = 10;

println!("x == y: {}", x == y);  // false
println!("x != y: {}", x != y);  // true
println!("x < y: {}", x < y);    // true
println!("x > y: {}", x > y);    // false
println!("x <= y: {}", x <= y);  // true
println!("x >= y: {}", x >= y);  // false
```

## Características Especiales

1. **Comparación de Punto Flotante:**
   - Cuidado con comparaciones de igualdad en punto flotante debido a imprecisiones.
   - Usa `f32::EPSILON` o `f64::EPSILON` para comparaciones aproximadas.

2. **Comparación de Estructuras y Enums:**
   - Deben implementar `PartialEq` para usar `==` y `!=`.
   - Para `<`, `>`, `<=`, `>=`, deben implementar `PartialOrd`.

3. **Operador `==` vs Método `eq()`:**
   - `==` llama internamente al método `eq()`.
   - `eq()` se puede sobrescribir para tipos personalizados.

4. **Comparación de Referencias:**
   - Las referencias se comparan por su valor, no por su dirección.

```rust
let a = 5;
let b = &a;
let c = &a;
println!("b == c: {}", b == c);  // true
```

5. **Operadores Bit a Bit:**
   - `&` (AND), `|` (OR), `^` (XOR), `!` (NOT)
   - `<<` (desplazamiento a la izquierda), `>>` (desplazamiento a la derecha)

```rust
let a = 0b1010;
let b = 0b1100;
println!("a & b: {:04b}", a & b);  // 1000
println!("a | b: {:04b}", a | b);  // 1110
println!("a ^ b: {:04b}", a ^ b);  // 0110
println!("!a: {:04b}", !a);        // ...11110101
```

## Buenas Prácticas

1. Usa paréntesis para clarificar el orden de las operaciones complejas.
2. Evita comparaciones de igualdad directa con números de punto flotante.
3. Implementa `PartialEq` y `PartialOrd` para tipos personalizados cuando sea necesario.
4. Utiliza métodos como `is_nan()` y `is_infinite()` para casos especiales de punto flotante.

Los operadores lógicos y relacionales en Rust proporcionan herramientas poderosas para el control de flujo y la toma de decisiones en programas. Su uso correcto es fundamental para escribir código Rust eficiente y libre de errores.