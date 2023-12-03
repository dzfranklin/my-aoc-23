use std::io::Read;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut m = Vec::new();
    std::io::stdin()
        .read_to_end(&mut m)
        .expect("failed to read stdin");
    if !m.ends_with(b"\n") {
        m.push(b'\n');
    }

    let stride = m.iter().position(|c| *c == b'\n').expect("has newline") + 1;
    let width = stride - 1; // account for \n
    let height = m.len() / stride;

    let s = stride as isize;
    #[rustfmt::skip]
    let adj: &[isize] = &[
        -s - 1, -s, -s + 1,
        -1, 1,
        s - 1, s, s + 1,
    ];

    let mut numbers_by_star = HashMap::<usize, Vec<usize>>::new();
    let mut adjacent_stars = HashSet::new();
    let mut digits = Vec::new();
    for h in 0..height {
        for j in 0..width {
            let i = h * stride + j;
            let c = m[i];

            if c.is_ascii_digit() {
                digits.push(c);

                for o in adj {
                    let i = (i as isize) + o;
                    if i < 0 || i > m.len() as isize - 1 {
                        continue;
                    }
                    let i = i as usize;
                    if m[i] == b'*' {
                        adjacent_stars.insert(i);
                    }
                }
            }

            if (!c.is_ascii_digit() || j == width - 1) && !digits.is_empty() {
                let num = parse_rev_digits(&digits); 
                for star in &adjacent_stars {
                    numbers_by_star.entry(*star).or_default().push(num);
                }
                digits.clear();
                adjacent_stars.clear();
            }
        }
    }

    let mut sum = 0;
    for (_star, numbers) in numbers_by_star {
        eprintln!("{_star}: {numbers:?}");
        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1];
        }
    }

    println!("{sum}");
}

fn parse_rev_digits(digits: &[u8]) -> usize {
    let mut n = 0;
    for i in 0..digits.len() {
        let c = digits[i];
        assert!(c >= b'0' && c <= b'9');
        let d = (c - b'0') as usize;
        let exp = digits.len() as u32 - i as u32 - 1;
        n += d * 10_usize.pow(exp);
    }
    return n;
}
