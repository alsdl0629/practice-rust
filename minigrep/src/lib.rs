// 라이브러리 크레이트

use std::fs;
use std::error::Error;

pub fn run(config: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; // 파일 열기

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(()) // 에러 처리만 하겠다는 뜻
}

pub struct Args {
    pub query: String,
    pub file_path: String,
}

impl Args {
    pub fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("query, file_path를 입력해주세요!")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Args { query, file_path })
    }
}

// 반환된 데이터가 contents 만큼 오래 살 것
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
