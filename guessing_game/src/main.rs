use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let target_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("값을 입력해 주세요!");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("입력 실패");

        let input: u32 = match input
            .trim()
            .parse() {
            Ok(input) => input,
            Err(_) => {
                println!("잘못된 입력값입니다.");
                continue
            },
        };

        println!("입력값 = {input}");

        match input.cmp(&target_number) {
            Ordering::Greater => println!("정답보다 큽니다!"),
            Ordering::Less => println!("정답보다 작습니다!"),
            Ordering::Equal => {
                println!("정답은 {input} 입니다! 축하드립니다!");
                break;
            }
        }
    }
}
