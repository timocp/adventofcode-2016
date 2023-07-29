use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => input.iter().filter(|triangle| possible(triangle)).count(),
            Part::Two => 0,
        }
    )
}

fn possible(triangle: &[u32]) -> bool {
    triangle[0] + triangle[1] > triangle[2]
        && triangle[0] + triangle[2] > triangle[1]
        && triangle[1] + triangle[2] > triangle[0]
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn test() {
    assert!(!possible(&[5, 10, 25]));
}
