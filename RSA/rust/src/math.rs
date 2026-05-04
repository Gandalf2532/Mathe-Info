//Just for my bigint type...
use crate::bigint::BigInt;
use std::mem;

pub struct Math {

}

impl Math {
pub fn ggt(a: &BigInt, b: &BigInt) -> BigInt {
    let mut x = a.clone();
    let mut y = b.clone();

    while !y.is_zero() { // .is_zero() sollte &self nehmen
        x %= &y; 
        std::mem::swap(&mut x, &mut y);
    }
    x
}
    pub fn is_prob_prime(_a: BigInt) -> bool { //is probably prime
        true
    }
}