pub fn fibonacci(n: usize) -> u64 {
    if n<2  {n as u64} else {fibonacci(n-1)+fibonacci(n-2)}
}