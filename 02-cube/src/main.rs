use bytes::{Bytes, Buf};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

fn main() {
    let mut sum = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut line = Bytes::from(line.as_bytes().to_vec());

        if !line.starts_with(b"Game ") {
            panic!("Unexpected format");
        }
        line.advance(b"Game ".len());

        let _id = take_number(&mut line).unwrap();
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

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for set in &sets {
            red = red.max(set.red);
            green = green.max(set.green);
            blue = blue.max(set.blue);
        }
        let power = red * green * blue;

        sum += power;
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

