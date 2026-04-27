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
    
    while a > 1 {
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

fn main() {
    // Schlüsselgenerierung
    let p: u64 = 61;
    let q: u64 = 53;
    let n: u64 = p * q;
    let phi_n: u64 = (p - 1) * (q - 1);
    
    let e: u64 = 17;  // Öffentlicher Exponent
    let d = mod_inverse(e as i64, phi_n as i64).expect("Inverses sollte existieren") as u64;
    
    println!("=== RSA Schlüssel ===");
    println!("p = {}, q = {}", p, q);
    println!("n = {} (Modulus)", n);
    println!("phi(n) = {}", phi_n);
    println!("e = {} (öffentlicher Exponent)", e);
    println!("d = {} (privater Exponent)", d);
    
    // Verschlüsselung und Entschlüsselung
    let message: u64 = 42;
    let encrypted = mod_pow(message, e, n);
    let decrypted = mod_pow(encrypted, d, n);
    
    println!("\n=== Verschlüsselung ===");
    println!("Nachricht: {}", message);
    println!("Verschlüsselt: {}^{} mod {} = {}", message, e, n, encrypted);
    println!("Entschlüsselt: {}^{} mod {} = {}", encrypted, d, n, decrypted);
}

// Schnelle modulare Exponentiation
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exp >>= 1;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
    }
    
    result
}