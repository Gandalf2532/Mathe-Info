mod bigint;
mod random;
mod math;

use bigint::BigInt;
use math::Math;
use random::Random;

fn main() {
    let p = Random::random_2048(10, 255);
    let q = Random::random_2048(10, 255);
    let n = &p * &q;
    let phi_n = (&p-BigInt::from_u64(1)) * (&q-BigInt::from_u64(1));
    let mut tmp: BigInt;
    loop {
        println!("bla1");
        tmp = Random::random_2048(10, 255);
        println!("{:?}", tmp.a);
        println!("bla1");
        if &tmp < &phi_n {
            println!("bla2"); //hier hängt er sich auf
            println!("{}", Math::ggt(&tmp, &phi_n));
            if Math::ggt(&tmp, &phi_n) == BigInt::from_u64(1) {
                break;
            }
        }
    }
    let e = tmp;
    println!("{:?} \n {:?} \n {:?} \n {:?}", p.a, q.a, n.a, e.a);
    println!("{}", e.is_even())
}