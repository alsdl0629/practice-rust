pub fn fibonacci(n: u64) -> u64 {
   return if n <= 1 {
       n
   } else {
       fibonacci(n - 1) + fibonacci(n - 2)
   }
}