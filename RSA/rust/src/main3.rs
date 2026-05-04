use std::str::Bytes;

use rand::random;

fn main() {
    // Schlüsselgenerierung
    let p: u128 = loop {
        let num = random_number(10000, 1000000);
        if is_prime(num, 5) {
            break num;
        }
    };
    let q: u128 = loop {
        let num = random_number(10000, 1000000);
        if is_prime(num, 5) {
            break num;
        }
    };
    let n: u128 = p * q;
    let phi_n: u128 = (p - 1) * (q - 1);
    
    let e: u128 = 65537;  // Öffentlicher Exponent
    let d = mod_inverse(e as i64, phi_n as i64).expect("Inverses sollte existieren") as u128;
    
    println!("=== RSA Schlüssel ===");
    println!("p = {}, q = {}", p, q);
    println!("n = {} (Modulus)", n);
    println!("phi(n) = {}", phi_n);
    println!("e = {} (öffentlicher Exponent)", e);
    println!("d = {} (privater Exponent)", d);
    
    // Verschlüsselung und Entschlüsselung
    let text: &str = "Rust ist toll!";
    let bytes: &[u8] = text.as_bytes();
    let textnumber: u128 = bytes_to_u128(bytes);
    let encrypted = mod_pow(textnumber, e, n);
    let decrypted_number = mod_pow(encrypted, d, n);
    let decrypted_bytes = u128_to_bytes(decrypted_number);
    let decrypted = String::from_utf8(decrypted_bytes).unwrap();
    
    println!("\n=== Verschlüsselung ===");
    println!("Nachricht: {}", text);
    println!("Verschlüsselt: {}^{} mod {} = {}", textnumber, e, n, encrypted);
    println!("Entschlüsselt: {}^{} mod {} = {}", encrypted, d, n, decrypted);
}

fn random_number(min: u128, max: u128) -> u128 {
    random::<u128>() % (max - min + 1) + min
}

// Schnelle modulare Exponentiation
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u128;
        }
        exp >>= 1;
        base = ((base as u128 * base as u128) % modulus as u128) as u128;
    }
    
    result
}

fn is_prime(n: u128, rounds: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 { return false; }

    let mut d = n - 1;
    let mut r = 0;

    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    for _ in 0..rounds {
        let a = 2 + rand::random::<u128>() % (n - 3);
        if !miller_rabin_test(n, a, d, r) {
            return false;
        }
    }

    true
}

fn miller_rabin_test(n: u128, a: u128, d: u128, r: u64) -> bool {
    let mut x = mod_pow(a, d, n);

    if x == 1 || x == n - 1 {
        return true;
    }

    for _ in 0..r - 1 {
        x = mod_pow(x, 2, n);
        if x == n - 1 {
            return true;
        }
    }

    false
}

// Erweiterte Euklidischer Algorithmus für modulares Inverses
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (mut a, mut m, mut x0, mut x1) = (a, m, 0i64, 1i64);
    let m0 = m; // ursprüngliches m speichern
    
    if m == 1 {
        return Some(0);
    }

    if m == 0 {
    return None;
    }
    
    while m != 0 {
        let q = a / m;
        let (old_m, old_x0) = (m, x0);
        m = a % m;
        a = old_m;
        x0 = x1 - q * x0;
        x1 = old_x0;
    }
    
    if x1 < 0 {
        x1 += m0; // korrekt!
    }
    
    Some(x1)
}

fn bytes_to_u128(bytes: &[u8]) -> u128 {
    let mut result = 0u128;

    for &b in bytes {
        result = result * 256 + b as u128;
    }

    result
}

fn u128_to_bytes(mut num: u128) -> Vec<u8> {
    let mut bytes = Vec::new();

    while num > 0 {
        bytes.push((num % 256) as u8);
        num /= 256;
    }

    bytes.reverse();
    bytes
}