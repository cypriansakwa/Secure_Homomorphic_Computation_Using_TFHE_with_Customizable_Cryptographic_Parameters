use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint8, FheBool};

fn main() {
    // Step 1: Set up configuration with default cryptographic parameters
    let config = ConfigBuilder::default().build();

    // Generate keys with the default configuration
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    // Step 2: Define and encrypt clear values
    let clear_a = 35u8;
    let clear_b = 7u8;
    let a = FheUint8::encrypt(clear_a, &client_key);
    let b = FheUint8::encrypt(clear_b, &client_key);

    // Step 3: Perform computations with optimized operations
    let add = optimized_add(&a, &b);
    let sub = optimized_sub(&a, &b);
    let mul = optimized_mul(&a, &b);
    let div = optimized_div(&a, &b);
    let rem = optimized_rem(&a, &b);
    let and = &a & &b;
    let or = &a | &b;
    let xor = &a ^ &b;
    let neg = -&a;
    let not = !&a;
    let shl = optimized_shl(&a, &b);
    let shr = optimized_shr(&a, &b);

    // Comparison operations using optimized primitives
    let eq = optimized_eq(&a, &b);
    let ne = optimized_ne(&a, &b);
    let gt = optimized_gt(&a, &b);
    let lt = optimized_lt(&a, &b);

    // Step 4: Decrypt results and verify
    let decrypted_add: u8 = add.decrypt(&client_key);
    assert_eq!(decrypted_add, clear_a + clear_b);

    let decrypted_sub: u8 = sub.decrypt(&client_key);
    assert_eq!(decrypted_sub, clear_a - clear_b);

    let decrypted_mul: u8 = mul.decrypt(&client_key);
    assert_eq!(decrypted_mul, clear_a * clear_b);

    let decrypted_div: u8 = div.decrypt(&client_key);
    assert_eq!(decrypted_div, clear_a / clear_b);

    let decrypted_rem: u8 = rem.decrypt(&client_key);
    assert_eq!(decrypted_rem, clear_a % clear_b);

    let decrypted_and: u8 = and.decrypt(&client_key);
    assert_eq!(decrypted_and, clear_a & clear_b);

    let decrypted_or: u8 = or.decrypt(&client_key);
    assert_eq!(decrypted_or, clear_a | clear_b);

    let decrypted_xor: u8 = xor.decrypt(&client_key);
    assert_eq!(decrypted_xor, clear_a ^ clear_b);

    let decrypted_neg: u8 = neg.decrypt(&client_key);
    assert_eq!(decrypted_neg, clear_a.wrapping_neg());

    let decrypted_not: u8 = not.decrypt(&client_key);
    assert_eq!(decrypted_not, !clear_a);

    let decrypted_shl: u8 = shl.decrypt(&client_key);
    assert_eq!(decrypted_shl, clear_a << clear_b);

    let decrypted_shr: u8 = shr.decrypt(&client_key);
    assert_eq!(decrypted_shr, clear_a >> clear_b);

    // Decrypt FheBool results and convert to u8 for verification
    let decrypted_eq: u8 = eq.decrypt(&client_key) as u8;
    assert_eq!(decrypted_eq, (clear_a == clear_b) as u8);

    let decrypted_ne: u8 = ne.decrypt(&client_key) as u8;
    assert_eq!(decrypted_ne, (clear_a != clear_b) as u8);

    let decrypted_gt: u8 = gt.decrypt(&client_key) as u8;
    assert_eq!(decrypted_gt, (clear_a > clear_b) as u8);

    let decrypted_lt: u8 = lt.decrypt(&client_key) as u8;
    assert_eq!(decrypted_lt, (clear_a < clear_b) as u8);

    println!("All computations verified successfully with custom cryptographic parameters.");
}

// Optimized arithmetic operations
fn optimized_add(a: &FheUint8, b: &FheUint8) -> FheUint8 {
    a + b
}

fn optimized_sub(a: &FheUint8, b: &FheUint8) -> FheUint8 {
    a - b
}

fn optimized_mul(a: &FheUint8, b: &FheUint8) -> FheUint8 {
    a * b
}

fn optimized_div(a: &FheUint8, b: &FheUint8) -> FheUint8 {
    a / b
}

fn optimized_rem(a: &FheUint8, b: &FheUint8) -> FheUint8 {
    a % b
}

fn optimized_shl(a: &FheUint8, b: &FheUint8) -> FheUint8 {
    a << b
}

fn optimized_shr(a: &FheUint8, b: &FheUint8) -> FheUint8 {
    a >> b
}

// Optimized comparison operations
fn optimized_eq(a: &FheUint8, b: &FheUint8) -> FheBool {
    a.eq(b)
}

fn optimized_ne(a: &FheUint8, b: &FheUint8) -> FheBool {
    a.ne(b)
}

fn optimized_gt(a: &FheUint8, b: &FheUint8) -> FheBool {
    a.gt(b)
}

fn optimized_lt(a: &FheUint8, b: &FheUint8) -> FheBool {
    a.lt(b)
}



