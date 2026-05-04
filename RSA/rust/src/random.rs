use crate::bigint::BigInt;
use rand::random_range;

pub struct Random {

}

impl Random {
    pub fn random_2048(min:u8, max:u8) -> BigInt {
        let mut res = BigInt::new();
        for _i in 0..256 {
            let randomnumber = random_range(min..max) as u8;
            res.a.push(randomnumber);
        }
        res
    }
}