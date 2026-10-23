pub fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    
    return fibonacci(n-2) + fibonacci(n - 1);
}
