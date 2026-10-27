pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let (dived, reminder) = ((x / y), (x % y));
    return (dived, reminder)
}
