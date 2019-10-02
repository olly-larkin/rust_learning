use std::collections::HashMap;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut map: HashMap<u64, bool> = (2..=upper_bound)
        .zip(vec![true; (upper_bound-1) as usize])
        .collect();
    for elem in 2..=upper_bound {
        if !*map.get(&elem).unwrap() {
            continue;
        } 
        for i in (2..=upper_bound).filter(|num| elem * num <= upper_bound) {
            *map.entry(i * elem).or_insert(false) = false;
        }
    }
    let mut ret: Vec<u64> = map
        .iter()
        .filter(|&(_, is_prime)| *is_prime)
        .map(|(num, _)| *num)
        .collect();
    ret.sort();
    ret
}
