// fn digit_num(mut num: u32) -> u32 {
//     let mut tot = 0;
//     while num != 0 {
//         tot += 1;
//         num /= 10;
//     }
//     tot
// }

fn digit_num(num: u32) -> u32 {
    format!("{}", num).len() as u32
}

// pub fn is_armstrong_number(num: u32) -> bool {
//     let mut sum = 0;
//     let mut temp = num;
//     let dig = digit_num(num);

//     while temp != 0 {
//         sum += (temp % 10).pow(dig);
//         temp /= 10;
//     }

//     sum == num
// }

pub fn is_armstrong_number(num: u32) -> bool {
    let dig = digit_num(num);
    format!("{}", num)
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .fold(0, |a, b| a + b.pow(dig)) == num
}