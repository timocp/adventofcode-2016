use crate::Puzzle;

pub struct Solver {
    addresses: Vec<IPv7>,
}

struct IPv7 {
    address: Vec<u8>,
}

impl From<&str> for IPv7 {
    fn from(s: &str) -> Self {
        let address = s.as_bytes().to_vec();
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
                let end_hypernet = self.address[pos..].iter().position(|&b| b == b']').unwrap();
                if has_abba(&self.address[pos..(pos + end_hypernet)]) {
                    return false;
                }
                pos += end_hypernet + 1;
            } else {
                if let Some(start_hypernet) = self.address[pos..].iter().position(|&b| b == b'[') {
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

    fn supports_ssl(&self) -> bool {
        let mut in_hypernet = false;
        let mut pos = 0;

        // boundary markers are (start, end + 1)
        let mut supernets = vec![];
        let mut hypernets = vec![];

        loop {
            if in_hypernet {
                let end_hypernet = self.address[pos..].iter().position(|&b| b == b']').unwrap();
                hypernets.push((pos, pos + end_hypernet));
                pos += end_hypernet + 1;
            } else {
                if let Some(start_hypernet) = self.address[pos..].iter().position(|&b| b == b'[') {
                    supernets.push((pos, pos + start_hypernet));
                    pos += start_hypernet + 1;
                } else {
                    supernets.push((pos, self.address.len()));
                    break;
                }
            }
            in_hypernet = !in_hypernet;
        }

        // for each ABA candidate in a supernet, see if its inverse exists in a hypernet
        for (start, end) in supernets {
            let abas = find_abas(&self.address[start..end]);
            for (a, b) in abas {
                for (start, end) in hypernets.iter() {
                    let part = &self.address[*start..*end];
                    if has_bab(part, a, b) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn has_abba(part: &[u8]) -> bool {
    (0..part.len() - 3)
        .map(|i| &part[i..i + 4])
        .any(|s| s[0] == s[3] && s[1] == s[2] && s[0] != s[1])
}

fn find_abas(part: &[u8]) -> Vec<(u8, u8)> {
    (0..part.len() - 2)
        .map(|i| &part[i..i + 3])
        .filter(|s| s[0] == s[2] && s[0] != s[1])
        .map(|s| (s[0], s[1]))
        .collect()
}

fn has_bab(part: &[u8], a: u8, b: u8) -> bool {
    (0..part.len() - 2).any(|i| part[i] == b && part[i + 1] == a && part[i + 2] == b)
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
        self.addresses
            .iter()
            .filter(|address| address.supports_ssl())
            .count()
            .to_string()
    }
}

#[test]
fn test() {
    assert!(IPv7::from("abba[mnop]qrst").supports_tls());
    assert!(!IPv7::from("abcd[bddb]xyyx").supports_tls());
    assert!(!IPv7::from("aaaa[qwer]tyui").supports_tls());
    assert!(IPv7::from("ioxxoj[asdfgh]zxcvbn").supports_tls());

    assert!(IPv7::from("aba[bab]xyz").supports_ssl());
    assert!(!IPv7::from("xyx[xyx]xyx").supports_ssl());
    assert!(IPv7::from("aaa[kek]eke").supports_ssl());
    assert!(IPv7::from("zazbz[bzb]cdb").supports_ssl());
}
