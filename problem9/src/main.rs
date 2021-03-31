fn main() {
    // limits found by eyeballing intersection points
    // of graphs a^2+b^2 = c^2 and a+b+c=1000
    for c in 410..500 {
        let rest = 1000 - c;
        for a in 1..=rest {
            if a*a+(rest-a)*(rest-a) == c*c {
                println!("{}x{}x{}={}", rest - a, a, c, a*(rest-a)*c);
                return
            } 
        }
    }
}

/*
a + b + c = 1000
a^2 + b^2 = c^2
a < b
b < c
sqrt(a^2 + b^2) = c
a + b + sqrt(a^2+b^2) = 1000

a + b = 1000 - c

*/