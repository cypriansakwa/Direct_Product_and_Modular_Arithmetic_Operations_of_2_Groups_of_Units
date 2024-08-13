use num::integer::gcd;

// Function to find elements of Z_n*
fn zn_star(n: u32) -> Vec<u32> {
    (1..n).filter(|&x| gcd(x, n) == 1).collect()
}

// Function to compute the direct product of Z_n* and Z_m*
fn direct_product(n: u32, m: u32) -> Vec<(u32, u32)> {
    let zn_star_elements = zn_star(n);
    let zm_star_elements = zn_star(m);
    
    let mut product = Vec::new();
    for &a in &zn_star_elements {
        for &b in &zm_star_elements {
            product.push((a, b));
        }
    }
    product
}

// Function to compute the product (a, b) * (e, f) mod (n, m)
fn product_mod((a, b): (u32, u32), (e, f): (u32, u32), n: u32, m: u32) -> (u32, u32) {
    ((a * e) % n, (b * f) % m)
}

// Function to find the modular inverse of a number in Z_n*
fn mod_inverse(a: u32, n: u32) -> Option<u32> {
    let (g, x, _) = extended_gcd(a as i32, n as i32);
    if g != 1 {
        None // No modular inverse if gcd is not 1
    } else {
        Some(((x % n as i32 + n as i32) % n as i32) as u32) // Ensure the result is positive
    }
}

// Extended Euclidean Algorithm
fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// Function to compute the inverse of an element in Z_n* x Z_m*
fn inverse_mod((a, b): (u32, u32), n: u32, m: u32) -> (Option<u32>, Option<u32>) {
    let inv_a = mod_inverse(a, n);
    let inv_b = mod_inverse(b, m);
    (inv_a, inv_b)
}

fn main() {
    let n = 5;
    let m = 8;

    let elements = direct_product(n, m);

    println!("Direct product of Z_{}^* and Z_{}^*:", n, m);
    for (a, b) in &elements {
        println!("({}, {})", a, b);
    }

    // Example of computing (a, b) * (e, f)
    let (a, b) = elements[9]; // Example element from Z_n* x Z_m*
    let (e, f) = elements[11]; // Another example element from Z_n* x Z_m*
    let (t, s) = elements[12]; // Different example element from Z_n* x Z_m*

    let product = product_mod((a, b), (e, f), n, m);

    println!("\nProduct of ({}, {}) and ({}, {}) mod ({}, {}): ({}, {})", a, b, e, f, n, m, product.0, product.1);

    // Example of computing inverse of (t, s)
    let (inv_t, inv_s) = inverse_mod((t, s), n, m);
    println!(
        "Inverse of ({}, {}) in Z_{}^* x Z_{}^* is: ({:?}, {:?})",
        t,
        s,
        n,
        m,
        inv_t,
        inv_s
    );
}







