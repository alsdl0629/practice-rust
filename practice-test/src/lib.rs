pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn panic_test() {
        panic!("테스트 실패")
    }

    #[test]
    fn err_test() {
        let result: Result<(), String> = Err(String::from("Error"));
        assert!(result.is_err())
    }
}
