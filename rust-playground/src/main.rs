mod calculate_temperature;
mod fibonacci;
mod collection;

use std::io;
use crate::collection::add_workers;

#[tokio::main]
async fn main() {
    // 화씨 온도와 섭씨 온도 간 변환하기
    println!("섭씨 or 화씨 온도를 입력해 주세요!");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("입력 실패");

    let temperature = calculate_temperature::convert_temperature_unit(temperature);
    println!("temperature = {temperature}");

    // n번째 피보나치 수 생성하기
    let fibonacci = fibonacci::fibonacci(6);
    println!("fibonacci = {fibonacci}");

    calculate_temperature::get_current_temperature().await;

    // 벡터에서 중간 값 가져오기
    let mut numbers = vec![5, 10, 4, 2, 3, 77, 43];
    let result = collection::get_middle(&mut numbers);
    println!("중간값은 {:?}, 정렬된 배열 요소: {:?}", result, numbers);

    // 백터에서 가장 많은 요소 가져오기
    let numbers = vec![1, 5, 6, 6];
    let result = collection::get_mode(&numbers);
    println!("최빈값은 {:?}", result);

    // 문자열을 피그 라틴으로 변경
    let str = String::from("first");
    let result = collection::change_pig_latin(&str);
    println!("pig Latin = {:?}", result);

    // 회사원 추가
    add_workers(
        &vec![
            String::from("Add Sally to HR"),
            String::from("Add Amir to Sales"),
            String::from("Add Kangmin to Engineering"),
        ]
    );
}
