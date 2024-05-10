use std::collections::HashMap;

pub const ESCROW_ADDRESS: &str = "escrow";

// definimos nuestro propio enum para representar el resultado de una
// operación que puede ser exitosa o fallida.
#[derive(PartialEq)]
pub enum Resultado<T, E> {
    // puede ser un valor de tipo `T`, encapsulado en un `Ok.
    Ok(T),
    // o puede ser un error `E` encapsulado en un `Error`
    Error(E),
}

// definimos un error simple que puede ser cualquier cosa
pub enum Error {
    UserNotFound,
    InsufficientBalance,
    InvalidEscrowFeePercent,
    NeedEscrowFeeReceiver,
    EscrowNeedsToBeOpen,
}

#[derive(PartialEq)]
enum EscrowStatus {
    Open,
    Lock,
    Close,
}

pub struct Escrow {
    sender: String,
    offer_coin: String,
    offer_amount: f32,
    ask_coin: String,
    ask_amount: f32,
    status: EscrowStatus,
}

// create a map to store the users and their balances
pub struct UserBalances {
    // user address => token address => token balance
    balances: HashMap<String, HashMap<String, f32>>,
}

pub fn new_escrow(
    user_balances: &mut UserBalances,
    sender: String,
    offer_coin: String,
    offer_amount: f32,
    ask_coin: String,
    ask_amount: f32,
) -> Result<Escrow, Error> {
    // Accedemos al balances del usuario usando get_mut para poder modificarlo luego
    match user_balances.balances.get_mut(&sender) {
        Some(coin_balances) => {
            // Verificamos si el balance para la moneda ofrecida es suficiente.
            match coin_balances.get_mut(&offer_coin) {
                Some(balance) if *balance >= offer_amount => {
                    // Restamos el balance del usuario.
                    *balance -= offer_amount;

                    // Sumamos el balance al contrato
                    user_balances
                        .balances
                        .entry(ESCROW_ADDRESS.to_string())
                        .or_default()
                        .entry(offer_coin.clone())
                        .and_modify(|balance| *balance += offer_amount)
                        .or_insert(offer_amount);

                    // Creamos la nueva instancia de Escrow.
                    Ok(Escrow {
                        sender,
                        offer_coin,
                        offer_amount,
                        ask_coin,
                        ask_amount,
                        status: EscrowStatus::Open,
                    })
                }
                _ => Err(Error::InsufficientBalance),
            }
        }
        None => Err(Error::UserNotFound),
    }
}

pub fn _deposit(mut escrow: Escrow, _receiver: String, _ask_amount: f32) -> Result<Escrow, Error> {
    if escrow.status != EscrowStatus::Open {
        return Err(Error::EscrowNeedsToBeOpen);
    }

    Ok(escrow)
}
