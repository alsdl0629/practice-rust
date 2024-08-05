mod calculate_temperature;
mod fibonacci;

use std::io;

fn main() {
    // 화씨 온도와 섭씨 온도 간 변환하기
    // println!("섭씨 or 화씨 온도를 입력해 주세요!");
    //
    // let mut temperature = String::new();
    //
    // io::stdin()
    //     .read_line(&mut temperature)
    //     .expect("입력 실패");
    //
    // let temperature = calculate_temperature::convert_temperature_unit(temperature);
    // println!("temperature = {temperature}");

    // n번째 피보나치 수 생성하기
    let fibonacci = fibonacci::fibonacci(6);
    println!("fibonacci = {fibonacci}");
}
