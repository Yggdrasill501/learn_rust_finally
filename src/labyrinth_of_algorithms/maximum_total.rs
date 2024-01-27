pub fn maxTotal(numbers: &[i32[) -> i32{
    let mut sum = 0;
    for &number in numbers{
        sum += number;
    }
    sum
}
