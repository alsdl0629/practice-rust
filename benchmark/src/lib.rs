// #![feature(test)]
// extern crate test;
//
// pub fn add(left: u64) -> u64 {
//     left + 2
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;
//
//     #[bench]
//     fn add_bench(b: &mut Bencher) {
//         b.iter(|| add(2))
//     }
// }

pub fn play_game(n: u32, print: bool) {
    let result = fizz_buzz_fibonacci(n);
    if print {
        println!("{result}");
    }
}

pub fn fizz_buzz_fibonacci(n: u32) -> String {
    if is_fibonacci_number(n) {
        "Fibonacci".to_string()
    } else {
        match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            (_, _) => n.to_string(),
        }
    }
}

fn is_fibonacci_number(n: u32) -> bool {
    for i in 0..=n {
        let (mut previous, mut current) = (0, 1);
        while current < i {
            let next = previous + current;
            previous = current;
            current = next;
        }
        if current == n {
            return true
        }
    }
    false
}
