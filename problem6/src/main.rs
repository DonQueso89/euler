use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Required cmd arg `n`");
        return;
    }

    let n: u64 = match args[1].parse() {
        Ok(v) => v,
        Err(_) => 100,
    };

    /**
     * e.g.:
     * n = 6
     * 5(0 0 0 0 0 6)
     * 4(0 0 0 0 5 6)
     * 3(0 0 0 4 5 6)
     * 2(0 0 3 4 5 6)
     * 1(0 2 3 4 5 6)
     * ---------------- +
     * times 2
     */
    let mut result: u64 = 0;
    let mut sum: u64 = (2..=n).fold(0, |acc, x| acc + x);
    for i in 2..=n {
        result += sum * (i - 1);
        sum -= i;
    }
    println!(
        "For n = {}, the square of sum - sum of squares = {}",
        n, 2*result
    );
}
