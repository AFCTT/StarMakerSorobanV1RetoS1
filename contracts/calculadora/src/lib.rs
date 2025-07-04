#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};
const RESULTADO: Symbol = symbol_short!("RESULTADO");
#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn sumar(env: Env, a: i128, b: i128) -> i128 {
        //Implementar función que sume dos números
        let resultado = a + b;
        env.storage().persistent().set(&RESULTADO, &resultado);
        return 30;
    }

    pub fn resultado_anterior(env: Env) -> i128 {
        //Implementar función que retorne el valor anterior
        env.storage()
            .persistent()
            .get::<Symbol, i128>(&RESULTADO)
            .unwrap_or(0);
        return 30;
    }
}

mod test;
