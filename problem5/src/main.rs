fn prod(v: &Vec<usize>) -> usize {
    v.iter().fold(1, |acc, x| acc * x)
}


fn check(P: &Vec<usize>, R: &Vec<usize>) -> bool {
    let p = prod(P);
    for r in R {
        if p % r != 0 {
            return false;
        }
    }
    true
}

fn best_candidate(P: &Vec<usize>, R: &Vec<usize>) -> usize {
    let mut best_score = 0;
    let mut best_candidate = 0;
    for candidate in R {
        let p = prod(P) * *candidate;
        let mut score = 1;

        for r in R {
            if p % r == 0 {
                score *= *r;
            }
        }

        if score > best_score {
            best_score = score;
            best_candidate = *candidate;
        }
    }
    best_candidate
}


fn main() {
    // Any prime in the initial set i without 
    // a multiple in I must end up in the
    // Product set
    let mut P = vec![11, 13, 17, 19];
    let mut R = vec![2, 3, 4, 5, 6, 7, 8 , 9, 10,12, 14, 16,18];
    
    /* Iteratively find optimum by:
        - scoring each candidate c in the Reservoir
        - score s is the product of the numbers n_i in R
        for which prod(P, c) % n_i == 0, divided by c
        - move candidate with the highest s
        from the Reservoir set to the Product set
    */

    while !check(&P, &R) {
        let best = best_candidate(&P, &R);
        P.push(best);
        R.retain(|&x| x != best);
    }
    println!("{}", prod(&P));

}
