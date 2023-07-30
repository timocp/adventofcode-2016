use crate::Part;

use md5::{Digest, Md5};

pub fn run(input: &str, part: Part) -> String {
    let door_id = input.trim();
    format!(
        "{}",
        match part {
            Part::One => bf_password(door_id),
            Part::Two => "".to_string(),
        }
    )
}

fn bf_password(door_id: &str) -> String {
    let mut password = String::with_capacity(8);
    let mut hasher = Md5::new();

    // store the number as ascii digits, so that it can be passed directly to the hasher.  this is
    // about 20% faster than storing it as an integer and converting it to a string each time we
    // need to hash it.
    let mut i: Vec<u8> = vec![b'0'];

    for _ in 0..8 {
        loop {
            hasher.update(door_id);
            hasher.update(&i);
            let hash = hasher.finalize_reset();
            increment(&mut i);

            if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
                password.push((hash[2] + if hash[2] < 10 { 48 } else { 87 }) as char);
                break;
            }
        }
    }

    password
}

// increment a number stored as a Vec of ascii digits
fn increment(i: &mut Vec<u8>) {
    for digit in i.iter_mut().rev() {
        if *digit < b'9' {
            *digit += 1;
            return;
        } else {
            *digit = b'0';
        }
    }
    i.insert(0, b'1');
}

#[test]
fn test() {
    assert_eq!("18f47a30", bf_password("abc"));
}
