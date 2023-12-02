use std::io::Read;

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

    let mut sum = 0;
    for h in 0..height {
        let mut num = Vec::new();
        let mut include = false;
        for j in 0..width {
            let i = h * stride + j;
            let c = m[i];

            if c.is_ascii_digit() {
                num.push(c);

                if !include {
                    for &offset in adj {
                        let neighbor = i as isize + offset;
                        if neighbor < 0 || neighbor >= m.len() as isize {
                            continue;
                        }

                        let neighbor = m[neighbor as usize];
                        if neighbor != b'\n' && neighbor != b'.' && !neighbor.is_ascii_digit() {
                            eprint!("{}", neighbor as char);
                            include = true;
                            break;
                        }
                    }
                }
            }

            if j == width - 1 || !c.is_ascii_digit() {
                if !num.is_empty() {
                    eprintln!("{} {}", include, parse_rev_digits(&num));
                    if include {
                        sum += parse_rev_digits(&num);
                    }
                    include = false;
                    num.clear();
                }
            }
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
