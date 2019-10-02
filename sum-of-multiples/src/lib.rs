pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| {
            for n in factors {
                if *n == 0 {
                    return false;
                }
                if x % n == 0 {
                    return true;
                }
            }
            false
        })
        .sum()
}
