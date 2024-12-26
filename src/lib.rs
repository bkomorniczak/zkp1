use num_bigint::BigUint;

pub fn expo(n: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    n.modpow(exp, modulus)
}

pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), q)
    }
    return q - (c * x).modpow(&BigUint::from(1u32), q)
}

pub fn verify(r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, alpha: &BigUint, beta: &BigUint, c:&BigUint, s: &BigUint, q: &BigUint, p: &BigUint) -> bool {
    let cond1: bool = *r1 == alpha.modpow(s,p) * y1.modpow(c,p);
    let cond2: bool = *r1 == beta.modpow(s,p) * y1.modpow(c,p);
    cond1 && cond2
}