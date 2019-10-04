use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut already_seen = vec![];
    let chars = input.chars().filter(|&c| {
            if c.is_alphabetic() && !already_seen.contains(&c) {
                already_seen.push(c);
                true
            } else {
                false
            }
        }).collect::<Vec<char>>();
    let input = input
        .split("==")
        .map(|string| 
            string
                .split('+')
                .map(|string2| string2.trim())
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();       // get input into manageable format

    for nums in NumGeneratorIter::new() {
        let chars = chars.clone();
        let hash = chars.iter().cloned().zip(nums.iter().rev().cloned()).collect();

        if valid_solution(&input, &hash) {
            return Some(hash);
        }
    }

    None
}

struct NumGeneratorIter {
    nums: Vec<u8>,
    current_index: usize,
}

impl NumGeneratorIter {
    fn new() -> Self {
        NumGeneratorIter {
            nums: Vec::new(),
            current_index: 0,
        }
    }
}

impl Iterator for NumGeneratorIter {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.nums.len() == 0 {
            self.nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            self.current_index = 9;
            Some(self.nums.clone())
        } else  if self.nums == vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0] {
            None
        } else {
            self.nums = gen_next_perm(self.nums.clone(), &mut self.current_index);
            Some(self.nums.clone())
        }
    }
}

fn gen_next_perm(vec: Vec<u8>, index: &mut usize) -> Vec<u8> {
    let mut available = vec.clone().split_off(*index);
    available.sort();

    if vec[*index] == *available.last().unwrap() {
        *index -= 1;
        return gen_next_perm(vec, index);
    }

    let mut vec = vec;
    let current = vec[*index];
    vec[*index] = available.iter()
        .cloned()
        .filter(|&val| val > current)
        .next()
        .unwrap();
    
    let i = available.iter().position(|&val| val == vec[*index]).unwrap();
    available.remove(i);

    for i in *index + 1 ..= 9 {
        vec[i] = available[0];
        available.remove(0);
    }
    *index = 9;

    vec
}

#[derive(Debug)]
enum ParseError {
    AbsentChar(char),
    StartsWithZero,
}

fn valid_solution(input: &Vec<Vec<&str>>, map: &HashMap<char, u8>) -> bool {
    if input.len() != 2 {
        return false;
    }
    
    let left;
    let right;
    match parse_vec(&input[0], map) {
        Ok(result) => left = result,
        _ => return false,
    }
    match parse_vec(&input[1], map) {
        Ok(result) => right = result,
        _ => return false,
    }

    left == right
}

fn parse_vec(input: &Vec<&str>, map: &HashMap<char, u8>) -> Result<u64, ParseError> {
    let mut total = 0;
    for val in input.iter() {
        let mut string = String::new();
        for (i, c) in val.chars().enumerate() {
            if !map.contains_key(&c) {
                return Err(ParseError::AbsentChar(c));
            }
            if i == 0 && map[&c] == 0 {
                return Err(ParseError::StartsWithZero);
            }
            string.push(map[&c].to_string().chars().next().unwrap());
        }
        total += string.parse::<u64>().unwrap();
    }
    Ok(total)
}