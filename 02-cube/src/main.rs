use bytes::{Bytes, Buf};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

fn main() {
    let mut sum = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut line = Bytes::from(line.as_bytes().to_vec());

        if !line.starts_with(b"Game ") {
            panic!("Unexpected format");
        }
        line.advance(b"Game ".len());

        let id = take_number(&mut line).unwrap();
        line.advance(2);

        let mut sets = Vec::new();
        let mut set = Set::default();
        loop {
            let n = take_number(&mut line).unwrap();
            line.advance(1);

            if line.starts_with(b"red") {
                set.red = n;
                line.advance(b"red".len());
            } else if line.starts_with(b"green") {
                set.green = n;
                line.advance(b"green".len());
            } else if line.starts_with(b"blue") {
                set.blue = n;
                line.advance(b"blue".len());
            } else {
                panic!("unexpected: {line:?}");
            }

            if line.is_empty() {
                sets.push(set);
                set = Set::default();
                break;
            }
            match line.get_u8() {
                b',' => {
                    line.advance(1);
                }
                b';' => {
                    line.advance(1);
                    sets.push(set);
                    set = Set::default();
                }
                other => panic!("unexpected: {other:?}"),
            }
        }

        let mut invalid = false;
        for set in &sets {
            if set.red > RED || set.green > GREEN || set.blue > BLUE {
                invalid = true;
                break;
            }
        }

        if !invalid {
            sum += id;
        }
    }

    println!("{sum}");
}

fn take_number(buf: &mut Bytes) -> Option<usize> {
    let mut digits = String::new();
    while let Some(c) = buf.get(0) {
        if !c.is_ascii_digit() {
            break;
        }
        digits.push(*c as char);
        buf.advance(1);
    }
    return digits.parse().ok()
}

