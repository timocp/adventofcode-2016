use crate::Puzzle;

pub struct Solver {
    addresses: Vec<IPv7>,
}

struct IPv7 {
    address: String,
}

impl From<&str> for IPv7 {
    fn from(s: &str) -> Self {
        let address = s.to_string();
        Self { address }
    }
}

impl IPv7 {
    fn supports_tls(&self) -> bool {
        let mut in_hypernet = false;
        let mut pos = 0;
        let mut is_abba = false;
        loop {
            if in_hypernet {
                let end_hypernet = self.address[pos..].find(']').unwrap();
                if has_abba(&self.address[pos..(pos + end_hypernet)]) {
                    return false;
                }
                pos += end_hypernet + 1;
            } else {
                if let Some(start_hypernet) = self.address[pos..].find('[') {
                    if !is_abba && has_abba(&self.address[pos..(pos + start_hypernet)]) {
                        is_abba = true;
                    }
                    pos += start_hypernet + 1;
                } else {
                    if !is_abba && has_abba(&self.address[pos..]) {
                        is_abba = true;
                    }
                    return is_abba;
                }
            }
            in_hypernet = !in_hypernet;
        }
    }
}

fn has_abba(part: &str) -> bool {
    (0..part.len() - 3).map(|i| &part[i..i + 4]).any(|s| {
        s.chars().nth(0) == s.chars().nth(3)
            && s.chars().nth(1) == s.chars().nth(2)
            && s.chars().nth(0) != s.chars().nth(1)
    })
}

impl Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            addresses: input.lines().map(|line| IPv7::from(line)).collect(),
        }
    }

    fn part1(&self) -> String {
        self.addresses
            .iter()
            .filter(|address| address.supports_tls())
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}

#[test]
fn test() {
    assert!(IPv7::from("abba[mnop]qrst").supports_tls());
    assert!(!IPv7::from("abcd[bddb]xyyx").supports_tls());
    assert!(!IPv7::from("aaaa[qwer]tyui").supports_tls());
    assert!(IPv7::from("ioxxoj[asdfgh]zxcvbn").supports_tls());
}