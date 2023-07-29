use crate::Part;

use regex::Regex;

pub fn run(input: &str, part: Part) -> String {
    let rooms = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&rooms),
            Part::Two => 0,
        }
    )
}

fn parse_input(input: &str) -> Vec<Room> {
    input.lines().map(|line| line.into()).collect()
}

// sum of the sectod IDs of the real rooms
fn part1(input: &[Room]) -> u32 {
    input
        .iter()
        .filter(|room| room.is_real())
        .map(|room| room.sector_id)
        .sum()
}

struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn is_real(&self) -> bool {
        self.checksum == self.calc_checksum()
    }

    fn calc_checksum(&self) -> String {
        let mut counts = self
            .name
            .chars()
            .filter(|&c| c != '-')
            .fold([0; 26], |mut counts, c| {
                counts[c as usize - 'a' as usize] += 1;
                counts
            });
        let mut cs = String::with_capacity(5);
        for _ in 0..5 {
            let mut max = 0;
            let mut letter = 0;
            for (i, count) in counts.iter().enumerate() {
                if *count > max {
                    max = *count;
                    letter = i;
                }
            }
            cs.push((letter as u8 + b'a') as char);
            counts[letter] = 0;
        }
        cs
    }
}

impl From<&str> for Room {
    fn from(s: &str) -> Self {
        let re: Regex = Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").unwrap();
        let cap = re.captures(s).unwrap();
        Self {
            name: cap[1].to_string(),
            sector_id: cap[2].parse().unwrap(),
            checksum: cap[3].to_string(),
        }
    }
}

#[test]
fn test() {
    let rooms = [
        "aaaaa-bbb-z-y-x-123[abxyz]",
        "a-b-c-d-e-f-g-h-987[abcde]",
        "not-a-real-room-404[oarel]",
        "totally-real-room-200[decoy]",
    ]
    .map(|input| Room::from(input));

    assert_eq!(rooms[0].name, "aaaaa-bbb-z-y-x");
    assert_eq!(rooms[0].sector_id, 123);
    assert_eq!(rooms[0].checksum, "abxyz");
    assert!(rooms[0].is_real());

    assert_eq!(rooms[1].name, "a-b-c-d-e-f-g-h");
    assert_eq!(rooms[1].sector_id, 987);
    assert_eq!(rooms[1].checksum, "abcde");
    assert!(rooms[1].is_real());

    assert_eq!(rooms[2].name, "not-a-real-room");
    assert_eq!(rooms[2].sector_id, 404);
    assert_eq!(rooms[2].checksum, "oarel");
    assert!(rooms[2].is_real());

    assert_eq!(rooms[3].name, "totally-real-room");
    assert_eq!(rooms[3].sector_id, 200);
    assert_eq!(rooms[3].checksum, "decoy");
    assert!(!rooms[3].is_real());
}
