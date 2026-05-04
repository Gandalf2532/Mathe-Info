use std::fmt;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::ops::RemAssign;

#[derive(Clone, Debug)]
pub struct BigInt {
    pub a: Vec<u8>
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", BigInt::to_u128(&self))
    }
}

//"simple", essenzielle Funktionen zu BigInt hinzufügen
impl BigInt {
    pub fn new() -> Self {
        Self { a: Vec::new() }
    }

    pub fn from_u64(mut n: u64) -> Self {
        let mut a = Vec::new();

        if n == 0 {
            return Self { a };
        }

        while n > 0 {
            a.push((n & 0xFF) as u8); // n % 256
            n >>= 8;                  // n / 256
        }

        Self { a }
    }

    pub fn from_u128(mut n: u128) -> Self {
        let mut a = Vec::new();

        if n == 0 {
            return Self { a };
        }

        while n > 0 {
            a.push((n & 0xFF) as u8); // n % 256
            n >>= 8;                  // n / 256
        }

        Self { a }
    }

    pub fn to_u128(&self) -> u128 {
        let mut result: u128 = 0;
        let mut shift = 0;

        for &byte in &self.a {
            result |= (byte as u128) << shift;
            shift += 8;
        }

        result
    }

    fn normalize(&mut self) {
        while let Some(&last) = self.a.last() {
            if last == 0 {
                self.a.pop();
            } else {
                break;
            }
        }
    }

    fn add(&self, other: &Self) -> Self {
        let mut result: Vec<u8> = Vec::new();
        let mut carry: u16 = 0;

        let max_len = self.a.len().max(other.a.len());

        for i in 0..max_len {
            let x = *self.a.get(i).unwrap_or(&0) as u16;
            let y = *other.a.get(i).unwrap_or(&0) as u16;

            let sum = x + y + carry;

            result.push((sum & 0xFF) as u8);
            carry = sum >> 8;
        }

        if carry > 0 {
            result.push(carry as u8);
        }

        let mut res = BigInt { a: result };
        res.normalize();
        res
    }

    fn sub(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        let mut borrow: i16 = 0;

        if self < other {
            panic!("Subtraction into the negativ is not implemented to BigInt")
        }

        let max_len = self.a.len();

        for i in 0..max_len {
            let x = self.a[i] as i16;
            let y = *other.a.get(i).unwrap_or(&0) as i16;

            let mut diff = x - y - borrow;

            if diff < 0 {
                diff += 256;
                borrow = 1;
            } else {
                borrow = 0;
            }

            result.push(diff as u8);
        }

        let mut res = Self { a: result };
        res.normalize();
        res
    }

    fn equal(&self, other: &Self) -> bool {
        let max_len = self.a.len().max(other.a.len());

        for i in 0..max_len {
            let x = *self.a.get(i).unwrap_or(&0);
            let y = *other.a.get(i).unwrap_or(&0);

            if x != y {
                return false;
            }
        }

        true
    }

    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        let len_a = self.a.len();
        let len_b = other.a.len();

        // zuerst Länge vergleichen
        if len_a < len_b {
            return Ordering::Less;
        }
        if len_a > len_b {
            return Ordering::Greater;
        }

        // gleiche Länge → von MSB vergleichen
        for i in (0..len_a).rev() {
            let x = self.a[i];
            let y = other.a[i];

            if x < y {
                return Ordering::Less;
            }
            if x > y {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    }

    fn mul(&self, other: &Self) -> Self {
        if self.a.is_empty() || other.a.is_empty() {
            return Self::new();
        }

        // Ergebnis-Vektor mit maximal möglicher Länge
        let mut result = vec![0u64; self.a.len() + other.a.len()];

        for i in 0..self.a.len() {
            for j in 0..other.a.len() {
                // Berechnung als u64, um Overflows beim Summieren zu vermeiden
                let prod = (self.a[i] as u64) * (other.a[j] as u64);
                result[i + j] += prod;
            }
        }

        // Carry propagation
        let mut carry = 0u64;
        let mut out = Vec::with_capacity(result.len());

        for val in result {
            let sum = val + carry;
            out.push((sum & 0xFF) as u8); // Extrahiere das unterste Byte
            carry = sum >> 8;             // Der Rest ist der Carry
        }

        // Falls am Ende noch ein Carry übrig ist
        while carry > 0 {
            out.push((carry & 0xFF) as u8);
            carry >>= 8;
        }

        let mut res = Self { a: out };
        res.normalize();
        res
    }

    fn shl_byte(&self) -> Self {
        let mut a = vec![0];
        a.extend_from_slice(&self.a);
        Self { a }
    }

    fn mul_u8(&self, n: u8) -> Self {
        let mut result = Vec::new();
        let mut carry: u16 = 0;

        for &x in &self.a {
            let prod = x as u16 * n as u16 + carry;
            result.push((prod & 0xFF) as u8);
            carry = prod >> 8;
        }

        if carry > 0 {
            result.push(carry as u8);
        }

        Self { a: result }
    }

    fn div(&self, other: &Self) -> Self {
        assert!(!other.a.is_empty()); // division by zero

        let mut remainder = BigInt::new();
        let mut quotient = Vec::new();

        for &digit in self.a.iter().rev() {
            // remainder = remainder * 256 + digit
            remainder = remainder.shl_byte();
            if remainder.a.is_empty() {
                remainder.a.push(digit);
            } else {
                remainder.a[0] = digit;
            }

            // finde q_digit (0..255)
            let mut q = 0u8;

            for i in (0..=255).rev() {
                let prod = other.mul_u8(i);
                if prod <= remainder {
                    q = i;
                    remainder = remainder.sub(&prod);
                    break;
                }
            }

            quotient.push(q);
        }

        quotient.reverse(); // zurück zu Little Endian

        let mut res = BigInt { a: quotient };
        res.normalize();
        res
    }

    pub fn div_rem(&self, other: &Self) -> (BigInt, BigInt) {
        assert!(!other.a.is_empty(), "Division by zero");

        let mut remainder = BigInt::new();
        let mut quotient_digits = Vec::new();

        for &digit in self.a.iter().rev() {
            remainder = remainder.shl_byte();
            if remainder.a.is_empty() {
                remainder.a.push(digit);
            } else {
                remainder.a[0] = digit;
            }

            let mut q = 0u8;
            // Hier könnte man noch optimieren, aber für den Anfang:
            for i in (0..=255).rev() {
                let prod = other.mul_u8(i);
                if prod <= remainder {
                    q = i;
                    remainder = remainder.sub(&prod);
                    break;
                }
            }
            quotient_digits.push(q);
        }

        quotient_digits.reverse();
        let mut q_res = BigInt { a: quotient_digits };
        q_res.normalize();
        
        (q_res, remainder)
    }

    fn rem(&self, other: &Self) -> Self {
        self.div_rem(other).1
    }

    pub fn is_even(&self) -> bool {
        self.a.get(0).unwrap_or(&0) % 2 == 0
    }

    pub fn is_zero(&self) -> bool {
        // Da normalize() bei dir 0-Bytes entfernt, 
        // ist ein leerer Vektor eine saubere Null.
        self.a.is_empty()
    }
}

//traits für BigInt hinzufügen
macro_rules! impl_op {
    ($trait:ident, $method:ident, $internal:ident) => {
        // 1. &BigInt + &BigInt (Die Basis)
        impl<'a, 'b> std::ops::$trait<&'b BigInt> for &'a BigInt {
            type Output = BigInt;
            fn $method(self, other: &'b BigInt) -> BigInt {
                BigInt::$internal(self, other)
            }
        }

        // 2. BigInt + BigInt
        impl std::ops::$trait<BigInt> for BigInt {
            type Output = BigInt;
            fn $method(self, other: BigInt) -> BigInt {
                std::ops::$trait::$method(&self, &other)
            }
        }

        // 3. BigInt + &BigInt
        impl<'a> std::ops::$trait<&'a BigInt> for BigInt {
            type Output = BigInt;
            fn $method(self, other: &'a BigInt) -> BigInt {
                std::ops::$trait::$method(&self, other)
            }
        }

        // 4. &BigInt + BigInt
        impl<'a> std::ops::$trait<BigInt> for &'a BigInt {
            type Output = BigInt;
            fn $method(self, other: BigInt) -> BigInt {
                std::ops::$trait::$method(self, &other)
            }
        }
    };
}

impl_op!(Add, add, add);
impl_op!(Sub, sub, sub);
impl_op!(Mul, mul, mul);
impl_op!(Div, div, div);
impl_op!(Rem, rem, rem);

impl RemAssign<&BigInt> for BigInt {
    fn rem_assign(&mut self, rhs: &BigInt) {
        if rhs.is_zero() {
            panic!("Division by zero");
        }
        // Nutze deine vorhandene div_rem Funktion
        let (_, remainder) = self.div_rem(rhs);
        self.a = remainder.a;
    }
}

// Optional: Damit auch x %= y (ohne &) funktioniert
impl RemAssign<BigInt> for BigInt {
    fn rem_assign(&mut self, rhs: BigInt) {
        self.rem_assign(&rhs);
    }
}


impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

impl Eq for BigInt {}
