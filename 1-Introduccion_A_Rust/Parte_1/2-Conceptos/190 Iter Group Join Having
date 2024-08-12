
# Operaciones con iteradores

## Group by y Count, Join, Having

Primero, definamos nuestras estructuras:

```rust
#[derive(Debug, Clone)]
struct Cliente {
    id: u32,
    nombre: String,
    ciudad: String,
}

#[derive(Debug, Clone)]
struct Pedido {
    id: u32,
    cliente_id: u32,
    producto: String,
    cantidad: u32,
}
```

Ahora, veamos algunos ejemplos de operaciones similares a SQL usando iteradores:

1. "JOIN" - Combinar clientes con sus pedidos

```rust
fn main() {
    let clientes = vec![
        Cliente { id: 1, nombre: "Alice".to_string(), ciudad: "Nueva York".to_string() },
        Cliente { id: 2, nombre: "Bob".to_string(), ciudad: "Los Ángeles".to_string() },
        Cliente { id: 3, nombre: "Charlie".to_string(), ciudad: "Chicago".to_string() },
    ];

    let pedidos = vec![
        Pedido { id: 101, cliente_id: 1, producto: "Laptop".to_string(), cantidad: 1 },
        Pedido { id: 102, cliente_id: 1, producto: "Mouse".to_string(), cantidad: 2 },
        Pedido { id: 103, cliente_id: 2, producto: "Teclado".to_string(), cantidad: 1 },
        Pedido { id: 104, cliente_id: 3, producto: "Monitor".to_string(), cantidad: 2 },
    ];

    // Simular un JOIN
    for cliente in &clientes {
        let pedidos_cliente: Vec<_> = pedidos.iter()
            .filter(|p| p.cliente_id == cliente.id)
            .collect();

        println!("Cliente: {}, Pedidos: {:?}", cliente.nombre, pedidos_cliente);
    }
}
```

2. "GROUP BY" y "COUNT" - Contar pedidos por cliente

```rust
use std::collections::HashMap;

fn main() {
    // ... (usar las mismas definiciones de clientes y pedidos del ejemplo anterior)

    let conteo_pedidos: HashMap<_, _> = pedidos.iter()
        .fold(HashMap::new(), |mut acc, pedido| {
            *acc.entry(pedido.cliente_id).or_insert(0) += 1;
            acc
        });

    for cliente in &clientes {
        let num_pedidos = conteo_pedidos.get(&cliente.id).unwrap_or(&0);
        println!("Cliente: {}, Número de pedidos: {}", cliente.nombre, num_pedidos);
    }
}
```

3. "LEFT JOIN" - Mostrar todos los clientes, incluso los que no tienen pedidos

```rust
fn main() {
    // ... (usar las mismas definiciones de clientes y pedidos)

    for cliente in &clientes {
        let pedidos_cliente: Vec<_> = pedidos.iter()
            .filter(|p| p.cliente_id == cliente.id)
            .collect();

        if pedidos_cliente.is_empty() {
            println!("Cliente: {}, Sin pedidos", cliente.nombre);
        } else {
            println!("Cliente: {}, Pedidos: {:?}", cliente.nombre, pedidos_cliente);
        }
    }
}
```

4. "HAVING" - Clientes con más de un pedido

```rust
use std::collections::HashMap;

fn main() {
    // ... (usar las mismas definiciones de clientes y pedidos)

    let conteo_pedidos: HashMap<_, _> = pedidos.iter()
        .fold(HashMap::new(), |mut acc, pedido| {
            *acc.entry(pedido.cliente_id).or_insert(0) += 1;
            acc
        });

    let clientes_multiples_pedidos: Vec<_> = clientes.iter()
        .filter(|c| conteo_pedidos.get(&c.id).unwrap_or(&0) > &1)
        .collect();

    println!("Clientes con más de un pedido: {:?}", clientes_multiples_pedidos);
}
```

5. "ORDER BY" - Ordenar clientes por el total de productos pedidos

```rust
fn main() {
    // ... (usar las mismas definiciones de clientes y pedidos)

    let mut clientes_con_total: Vec<_> = clientes.iter()
        .map(|c| {
            let total_productos: u32 = pedidos.iter()
                .filter(|p| p.cliente_id == c.id)
                .map(|p| p.cantidad)
                .sum();
            (c, total_productos)
        })
        .collect();

    clientes_con_total.sort_by_key(|&(_, total)| std::cmp::Reverse(total));

    for (cliente, total) in clientes_con_total {
        println!("Cliente: {}, Total de productos pedidos: {}", cliente.nombre, total);
    }
}
```

6. "INNER JOIN" con filtro - Clientes de Nueva York y sus pedidos

```rust
fn main() {
    // ... (usar las mismas definiciones de clientes y pedidos)

    let clientes_ny = clientes.iter()
        .filter(|c| c.ciudad == "Nueva York");

    for cliente in clientes_ny {
        let pedidos_cliente: Vec<_> = pedidos.iter()
            .filter(|p| p.cliente_id == cliente.id)
            .collect();

        println!("Cliente de NY: {}, Pedidos: {:?}", cliente.nombre, pedidos_cliente);
    }
}
```

Estos ejemplos demuestran cómo puedes usar iteradores en Rust para realizar operaciones similares a las consultas SQL en vectores de estructuras. Algunas observaciones importantes:

1. En Rust, estas operaciones se realizan en memoria, a diferencia de SQL que trabaja con bases de datos.
2. Para conjuntos de datos grandes, estas operaciones en memoria pueden no ser tan eficientes como las consultas SQL optimizadas.
3. Rust ofrece gran flexibilidad para combinar y encadenar estas operaciones de manera muy expresiva.
4. Para casos de uso más complejos o con grandes volúmenes de datos, considera usar una base de datos real con Rust.

Estos patrones son útiles para manipular datos en memoria de manera eficiente y pueden ser una buena alternativa para conjuntos de datos pequeños a medianos o para procesamiento de datos en aplicaciones que no requieren una base de datos completa.