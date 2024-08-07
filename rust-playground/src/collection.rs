use std::collections::HashMap;

pub fn get_middle(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    let length = numbers.len();

    return numbers[length / 2];
}

pub fn get_mode(numbers: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for i in numbers {
        let value = map.entry(i).or_insert(0);
        *value += 1;
    }
    println!("map 상태 {:?}", map);

    let mut max_key = -999;
    let mut max_value = -999;

    for (key, value) in &map {
        if value > &max_value {
            max_key = **key;
            max_value = *value;
        }
    }

    return max_key;
}

pub fn change_pig_latin(str: &String) -> String {
    let mut result = String::new();

    let first_word_bytes = str.as_bytes()[0];
    let first_word_char = first_word_bytes as char;
    let first_word_ascii = first_word_char as u8;

    if 97 <= first_word_ascii && first_word_ascii <= 122 {
        let remain_word = &str[1..];
        result = format!("{remain_word}-{first_word_char}ay");
    } else {
        result = format!("{str}-hay");
    }
    return result;
}

pub fn add_workers(workers: &Vec<String>) {
    let mut worker_map = HashMap::new();

    for worker in workers {
        let worker: Vec<&str> = worker.split(" ").collect(); // 벡터로 변환
        worker_map.insert(worker[3], worker[1]);
    }

    println!("workers {:?}", worker_map);
}
