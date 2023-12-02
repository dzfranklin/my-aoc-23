use std::io;

fn main() -> Result<(), Box<dyn Send + Sync + std::error::Error + 'static>> {
    let mut sum = 0;
    for line in io::stdin().lines() {
        let mut digits = Vec::new();
        for c in line?.chars() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d);
            }
        }
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let n = first * 10 + last;
        sum += n;
    }
    println!("{}", sum);
    Ok(())
}
