fn is_palindrome(p: usize) -> bool {
    let mut r = 0;
    let mut pow = 0;
    let mut v = vec![];
    let mut pp = p;
    while pp > 0 {
        pow += 1;
        let d = pp % 10usize.pow(pow);
        v.push(d / 10usize.pow(pow - 1));
        pp -= d;
    }
    for (ppow, &i) in v.iter().enumerate() {
        r += &i * 10usize.pow(pow - ppow as u32 - 1);
    }
    return r == p;
}


fn main() {
    let mut max = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let p = i * j;
            if p > max && is_palindrome(p) {
                max = p;
                // println!("{}x{}={}", i, j, p);
            }
        }
    }
    println!("{}", max);
}
