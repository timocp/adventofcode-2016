use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&input),
            Part::Two => "?".to_string(),
        }
    )
}

fn part1(input: &[Vec<Direction>]) -> String {
    let mut pos = 5;
    let mut output = "".to_string();
    for line in input {
        for dir in line {
            match dir {
                Direction::Up => {
                    if pos > 3 {
                        pos -= 3;
                    }
                }
                Direction::Down => {
                    if pos < 7 {
                        pos += 3;
                    }
                }
                Direction::Left => {
                    if pos % 3 != 1 {
                        pos -= 1;
                    }
                }
                Direction::Right => {
                    if pos % 3 != 0 {
                        pos += 1;
                    }
                }
            }
        }
        output.push_str(&pos.to_string());
    }

    output
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unexpected character: {}", c),
                })
                .collect()
        })
        .collect()
}

#[test]
fn test() {
    let test_input = "ULL\nRRDDD\nLURDL\nUUUUD\n";
    assert_eq!("1985", run(test_input, Part::One));
}
