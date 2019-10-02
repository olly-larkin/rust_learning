fn has_factors(n: u32) -> bool {
    let mut i = 2;
    loop {
        if n % i == 0 {
            return true;
        }
        if i * i >= n {
            return false;
        }
        i += 1;
    }
}

pub fn nth(n: u32) -> u32 {
    let mut m = n;
    let mut ans = 2;
    let mut guess = 2;
    while m != 0{ 
        guess += 1;
        if !has_factors(guess) {
            ans = guess;
            m -= 1;
        }
    }
    ans
}
