use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq, Clone)]
struct Cuenta {
    nombre: String,
    id: String,
    pin: String,
    saldo: f64,
}

impl Cuenta {
    fn nueva(nombre: String, id: String, pin: String) -> Cuenta {
        Cuenta {
            nombre,
            id,
            pin,
            saldo: 0.0,
        }
    }

    fn depositar(&mut self, cantidad: f64) {
        self.saldo += cantidad;
        println!(
            "----------------------------------\n\
            Depositaste ${:.2}. Tu nuevo saldo es ${:.2}.",
            cantidad, self.saldo
        );
    }

    /// Retira una cantidad de la cuenta si:
    /// - el PIN es correcto
    /// - el saldo es suficiente.
    fn retirar(&mut self, cantidad: f64, pin: &String) -> Result<(), String> {
        if &self.pin != pin {
            return Err("----------------------------------\n\
            PIN incorrecto."
                .to_string());
        }

        if cantidad <= 0.0 {
            return Err("----------------------------------\n\
            La cantidad a retirar debe ser mayor a 0."
                .to_string());
        }

        if self.saldo >= cantidad {
            self.saldo -= cantidad;
            println!(
                "----------------------------------\n\
                Retiraste ${:.2}. Tu nuevo saldo es ${:.2}.",
                cantidad, self.saldo
            );
            Ok(())
        } else {
            Err(format!(
                "----------------------------------\n\
                Saldo insuficiente. Tu saldo es ${:.2}.",
                self.saldo
            ))
        }
    }

    fn consultar_saldo(&self) -> f64 {
        println!(
            "----------------------------------\n\
        Tu saldo es ${:.2}.",
            self.saldo
        );
        self.saldo
    }
}

struct Banco {
    // Numero de cuenta -> Cuenta
    cuentas: HashMap<String, Cuenta>,
    siguiente_cuenta: u32,
}

impl Banco {
    fn nuevo() -> Banco {
        Banco {
            cuentas: HashMap::new(),
            siguiente_cuenta: 1,
        }
    }

    fn abrir_cuenta(&mut self, nombre: String, id: String, pin: String) -> String {
        let numero_cuenta = self.generar_numero_cuenta();
        self.cuentas.insert(
            numero_cuenta.clone(),
            Cuenta::nueva(nombre.clone(), id, pin),
        );
        println!(
            "----------------------------------\n\
            > Cuenta creada para {}. Número de cuenta: {}",
            nombre, numero_cuenta
        );
        numero_cuenta
    }

    fn generar_numero_cuenta(&mut self) -> String {
        let numero_cuenta = format!("ES{:04}", self.siguiente_cuenta);
        self.siguiente_cuenta += 1;
        numero_cuenta
    }

    fn obtener_cuenta(&self, nombre: &String, id: &String) -> Option<&Cuenta> {
        self.cuentas
            .values()
            .find(|cuenta| &cuenta.id == id && &cuenta.nombre == nombre)
    }

    fn obtener_cuenta_mut(&mut self, numero_cuenta: &String) -> Option<&mut Cuenta> {
        self.cuentas.get_mut(numero_cuenta)
    }

    fn menu(&mut self) {
        loop {
            println!("\n====== Menú ======");
            println!("1. Abrir cuenta");
            println!("2. Depositar");
            println!("3. Retirar");
            println!("4. Consultar saldo");
            println!("5. Recuperar cuenta");
            println!("6. Salir");
            println!("===================");

            let mut opcion = String::new();
            io::stdin()
                .read_line(&mut opcion)
                .expect("Error al leer la entrada");
            let opcion: u32 = opcion.trim().parse().expect("Por favor ingresa un número");

            match opcion {
                1 => {
                    let nombre = self.leer_input("Ingresa tu nombre:");
                    let id = self.leer_input("Ingresa tu ID:");
                    let pin = self.leer_input("Ingresa un PIN:");
                    self.abrir_cuenta(nombre, id, pin);
                }
                2 => {
                    let numero_cuenta = self.leer_input("Ingresa tu número de cuenta:");
                    let cantidad = self.leer_cantidad("Ingresa la cantidad a depositar:");
                    if let Some(cuenta) = self.obtener_cuenta_mut(&numero_cuenta) {
                        cuenta.depositar(cantidad);
                    } else {
                        println!("> Cuenta no encontrada.");
                    }
                }
                3 => {
                    let numero_cuenta = self.leer_input("Ingresa tu número de cuenta:");
                    let pin = self.leer_input("Ingresa tu PIN:");
                    let cantidad = self.leer_cantidad("Ingresa la cantidad a retirar:");
                    if let Some(cuenta) = self.obtener_cuenta_mut(&numero_cuenta) {
                        match cuenta.retirar(cantidad, &pin) {
                            Ok(_) => (),
                            Err(e) => println!("{}", e),
                        }
                    } else {
                        println!("> Cuenta no encontrada.");
                    }
                }
                4 => {
                    let numero_cuenta = self.leer_input("Ingresa tu número de cuenta:");
                    if let Some(cuenta) = self.cuentas.get(&numero_cuenta) {
                        cuenta.consultar_saldo();
                    } else {
                        println!("> Cuenta no encontrada.");
                    }
                }
                5 => {
                    let nombre = self.leer_input("Ingresa tu nombre:");
                    let id = self.leer_input("Ingresa tu ID:");
                    if let Some(cuenta) = self.obtener_cuenta(&nombre, &id) {
                        println!("> Cuenta encontrada: Número de cuenta: {}", cuenta.id);
                    } else {
                        println!("> Cuenta no encontrada.");
                    }
                }
                6 => break,
                _ => println!("> Opción no válida."),
            }
        }
    }

    fn leer_input(&self, prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");
        input.trim().to_string()
    }

    fn leer_cantidad(&self, prompt: &str) -> f64 {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");
        input.trim().parse().expect("Por favor ingresa un número")
    }
}

fn main() {
    let mut banco = Banco::nuevo();
    banco.menu();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abrir_cuenta() {
        let mut banco = Banco::nuevo();
        let numero_cuenta =
            banco.abrir_cuenta("1234".to_string(), "Juan".to_string(), "0000".to_string());
        assert_eq!(banco.cuentas.len(), 1);
        assert!(banco.cuentas.contains_key(&numero_cuenta));
    }

    #[test]
    fn test_depositar() {
        let mut cuenta = Cuenta::nueva("Juan".to_string(), "1234".to_string(), "0000".to_string());
        cuenta.depositar(100.0);
        assert_eq!(cuenta.saldo, 100.0);
    }

    #[test]
    fn test_retirar_exitoso() {
        // crear cuenta
        let mut cuenta = Cuenta::nueva("Juan".to_string(), "1234".to_string(), "0000".to_string());
        // depositar 100
        cuenta.depositar(100.0);
        // retirar 50 y revisar saldo
        let _resultado = cuenta.retirar(50.0, &"0000".to_string()).unwrap();
        assert_eq!(cuenta.saldo, 50.0);
        // retirar el resto y revisar saldo
        let _resultado = cuenta.retirar(cuenta.saldo, &"0000".to_string()).unwrap();
        assert_eq!(cuenta.saldo, 0.0);
    }

    #[test]
    fn test_retirar_pin_incorrecto() {
        let mut cuenta = Cuenta::nueva("Juan".to_string(), "1234".to_string(), "0000".to_string());
        cuenta.depositar(100.0);
        let resultado = cuenta.retirar(50.0, &"1234".to_string());
        assert!(resultado.is_err());
        assert_eq!(cuenta.saldo, 100.0);
    }

    #[test]
    fn test_retirar_saldo_insuficiente() {
        let mut cuenta = Cuenta::nueva("Juan".to_string(), "1234".to_string(), "0000".to_string());
        cuenta.depositar(100.0);
        let resultado = cuenta.retirar(150.0, &"0000".to_string());
        assert!(resultado.is_err());
        assert_eq!(cuenta.saldo, 100.0);
    }

    #[test]
    fn test_consultar_saldo() {
        // creat cuenta
        let mut cuenta = Cuenta::nueva("Juan".to_string(), "1234".to_string(), "0000".to_string());
        assert_eq!(cuenta.consultar_saldo(), 0.0);
        // depositar 100 y consultar saldo de nuevo
        cuenta.depositar(100.0);
        assert_eq!(cuenta.consultar_saldo(), 100.0);
    }

    #[test]
    fn test_obtener_cuenta() {
        let mut banco = Banco::nuevo();
        let numero_cuenta =
            banco.abrir_cuenta("Juan".to_string(), "1234".to_string(), "0000".to_string());
        let cuenta = banco.obtener_cuenta(&"Juan".to_string(), &"1234".to_string());
        assert_eq!(
            numero_cuenta,
            format!("ES{:04}", banco.siguiente_cuenta - 1)
        );
        assert_eq!(cuenta.unwrap().nombre, "Juan");
        assert_eq!(cuenta.unwrap().id, "1234");
        assert_eq!(cuenta.unwrap().pin, "0000");
        assert_eq!(cuenta.unwrap().saldo, 0.0);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_varios_usuarios_y_varias_operaciones() {
            // instanciar banco y abrir cuentas
            let mut banco = Banco::nuevo();
            let numero_cuenta1 =
                banco.abrir_cuenta("Juan".to_string(), "1234".to_string(), "0000".to_string());
            let numero_cuenta2 =
                banco.abrir_cuenta("Pedro".to_string(), "5678".to_string(), "1111".to_string());

            assert_eq!(
                numero_cuenta1,
                format!("ES{:04}", banco.siguiente_cuenta - 2)
            );
            assert_eq!(
                numero_cuenta2,
                format!("ES{:04}", banco.siguiente_cuenta - 1)
            );

            // obtener cuentas y comprobar que los datos son correctos
            let cuenta1 = banco
                .obtener_cuenta(&"Juan".to_string(), &"1234".to_string())
                .unwrap();

            let cuenta2 = banco
                .obtener_cuenta(&"Pedro".to_string(), &"5678".to_string())
                .unwrap();

            assert_eq!(
                cuenta1,
                &Cuenta {
                    nombre: "Juan".to_string(),
                    id: "1234".to_string(),
                    pin: "0000".to_string(),
                    saldo: 0.0
                }
            );
            assert_eq!(
                cuenta2,
                &Cuenta {
                    nombre: "Pedro".to_string(),
                    id: "5678".to_string(),
                    pin: "1111".to_string(),
                    saldo: 0.0
                }
            );

            // depositar y retirar dinero de la cuenta1
            let cuenta1 = banco.obtener_cuenta_mut(&numero_cuenta1).unwrap();
            cuenta1.depositar(420.0);
            assert_eq!(cuenta1.saldo, 420.0);
            cuenta1.depositar(100.0);
            assert_eq!(cuenta1.saldo, 420.0 + 100.0);
            cuenta1.retirar(50.0, &"0000".to_string()).unwrap();
            assert_eq!(cuenta1.saldo, 420.0 + 100.0 - 50.0);
            cuenta1.retirar(cuenta1.saldo, &"0000".to_string()).unwrap();
            assert_eq!(cuenta1.saldo, 0.0);

            let cuenta2 = banco.obtener_cuenta_mut(&numero_cuenta2).unwrap();
            cuenta2.depositar(50.0);
            assert_eq!(cuenta2.saldo, 50.0);
            cuenta2.retirar(10.0, &"1111".to_string()).unwrap();
            assert_eq!(cuenta2.saldo, 50.0 - 10.0);
            let _err = cuenta2.retirar(100.0, &"1111".to_string()).is_err();
            assert_eq!(cuenta2.saldo, 50.0 - 10.0);
            cuenta2.depositar(100.0);
            assert_eq!(cuenta2.saldo, 50.0 - 10.0 + 100.0);
            cuenta2.retirar(cuenta2.saldo, &"1111".to_string()).unwrap();
            assert_eq!(cuenta2.saldo, 0.0);
        }
    }
}
