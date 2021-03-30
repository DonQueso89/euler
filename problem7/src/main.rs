fn is_prime(n: u64) -> bool {
    if n == 1 {
        return true;
    }
    
    let sq = (n as f64).sqrt() as u64;
    for d in 2..=sq {
        if n % d == 0 {
            return false;
        }
    }
    true
}

fn nth_prime(n: u16) -> u64 {
    let mut c = 0;
    let mut p: u64 = 1;
    while c < n {
        p += 1;
        if is_prime(p) {
            c += 1
        }
    }
    p
}

fn main() {
    println!("{}", nth_prime(10001));
}
