// 라이브러리 크레이트
use std::{env, fs};
use std::error::Error;

pub fn run(config: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; // 파일 열기

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(()) // 에러 처리만 하겠다는 뜻
}

pub struct Args {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Args {
    pub fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("query, file_path를 입력해주세요!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // 환경 변수 우선순위
        let ignore_case_by_command_line = if (&args[3]).parse::<u32>().unwrap() == 1 {
            true
        } else {
            false
        };

        let ignore_case_by_env = env::var("IGNORE_CASE").is_ok();

        let result = if ignore_case_by_env {
            Ok(Args { query, file_path, ignore_case: ignore_case_by_env })
        } else {
            Ok(Args { query, file_path, ignore_case:ignore_case_by_command_line })
        };

        result
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
