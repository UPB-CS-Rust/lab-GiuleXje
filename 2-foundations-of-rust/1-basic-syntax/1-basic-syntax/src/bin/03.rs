fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let mut max = 0;
    let mut min = std::u32::MAX;
    for x in input{
        if x > max {
            max = x;
        }
        if x < min {
            min = x;
        }
    }
    println!("{} is largest and {} is smallest", max, min);
}
