use std::io;
use bytes::{Bytes, Buf};

const DIGIT_NAMES: &[&[u8]] = &[
    b"zero",
    b"one",
    b"two",
    b"three",
    b"four",
    b"five",
    b"six",
    b"seven",
    b"eight",
    b"nine",
];

fn main() -> Result<(), Box<dyn Send + Sync + std::error::Error + 'static>> {
    let mut sum: usize = 0;
    for line in io::stdin().lines() {
        let line = line?;
        let line = line.as_bytes();

        let mut first = None;
        'scan: for offset in 0..line.len() {
            let chunk = &line[offset..];

            if chunk[0] >= b'0' && chunk[0] <= b'9' {
                let d = (chunk[0] - b'0') as usize;
                first = Some(d);
                break;
            }

            for (d, candidate) in DIGIT_NAMES.iter().enumerate() {
                if chunk.starts_with(candidate) {
                   first = Some(d);
                   break 'scan;
               }
            }
        }
        let first = first.unwrap_or(0);

        let mut last = None;
        'scan: for offset in (0..line.len()).rev() {
            let chunk = &line[offset..];

            if chunk[0] >= b'0' && chunk[0] <= b'9' {
                let d = (chunk[0] - b'0') as usize;
                last = Some(d);
                break;
            }

            for (d, candidate) in DIGIT_NAMES.iter().enumerate() {
                if chunk.starts_with(candidate) {
                   last = Some(d);
                   break 'scan;
               }
            }
        }
        let last = last.unwrap_or(0);

        let n = first * 10 + last;
        sum += n;
    }
    println!("{}", sum);
    Ok(())
}
