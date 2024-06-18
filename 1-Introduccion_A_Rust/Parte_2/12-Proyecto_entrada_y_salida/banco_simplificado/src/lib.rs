pub struct Banco {
    pub cuentas: Vec<Cuenta>,
    pub num_cuentas: u32,
}

#[derive(PartialEq, Debug)]
pub struct Cuenta {
    pub titular: String,
    pub iban: String,
    pub saldo: f64,
}

impl Banco {
    pub fn nueva_cuenta(&mut self, titular: String) {
        self.num_cuentas += 1;
        let cuenta = Cuenta {
            titular: titular,
            iban: format!("ES{:04}", self.num_cuentas),
            saldo: 0.0,
        };
        self.cuentas.push(cuenta);
    }
    pub fn depositar(&mut self, iban: &str, cantidad: f64) {
        for cuenta in &mut self.cuentas {
            if cuenta.iban == iban {
                cuenta.saldo += cantidad;
                break;
            }
        }
    }
    pub fn retirar(&mut self, iban: &str, cantidad: f64) {
        for cuenta in &mut self.cuentas {
            if cuenta.iban == iban {
                cuenta.saldo -= cantidad;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_tests() {
        let mut banco = Banco {
            cuentas: Vec::new(),
            num_cuentas: 0,
        };

        banco.nueva_cuenta("Alice".to_string());
        banco.nueva_cuenta("Marta".to_string());

        assert_eq!(
            banco.cuentas,
            vec![
                Cuenta {
                    titular: "Alice".to_string(),
                    iban: "ES0001".to_string(),
                    saldo: 0.,
                },
                Cuenta {
                    titular: "Marta".to_string(),
                    iban: "ES0002".to_string(),
                    saldo: 0.,
                },
            ]
        );

        banco.depositar("ES0001", 100.0);
        banco.depositar("ES0002", 200.0);

        assert_eq!(
            banco.cuentas,
            vec![
                Cuenta {
                    titular: "Alice".to_string(),
                    iban: "ES0001".to_string(),
                    saldo: 100.0,
                },
                Cuenta {
                    titular: "Marta".to_string(),
                    iban: "ES0002".to_string(),
                    saldo: 200.0,
                },
            ]
        );

        banco.retirar("ES0001", 50.0);
        banco.retirar("ES0002", 100.0);

        assert_eq!(
            banco.cuentas,
            vec![
                Cuenta {
                    titular: "Alice".to_string(),
                    iban: "ES0001".to_string(),
                    saldo: 50.0,
                },
                Cuenta {
                    titular: "Marta".to_string(),
                    iban: "ES0002".to_string(),
                    saldo: 100.0,
                },
            ]
        );

        banco.retirar("ES0001", 100.0);
        assert_eq!(banco.cuentas[0].saldo, -50.0);
    }
}
