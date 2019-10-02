pub fn factors(n: u64) -> Vec<u64> {
    for i in 2..=n {
        if n % i == 0 {
            let mut ret = factors(n/i);
            ret.insert(0, i);
            return ret;
        }
    }
    vec![]
}
