use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => input.iter().filter(|triangle| possible(triangle)).count(),
            Part::Two => count_verticle(&input),
        }
    )
}

fn possible(triangle: &[u32]) -> bool {
    triangle[0] + triangle[1] > triangle[2]
        && triangle[0] + triangle[2] > triangle[1]
        && triangle[1] + triangle[2] > triangle[0]
}

fn count_verticle(input: &[Vec<u32>]) -> usize {
    let mut count = 0;
    for i in 0..input.len() / 3 {
        for j in 0..3 {
            let triangle = [input[i * 3][j], input[i * 3 + 1][j], input[i * 3 + 2][j]];
            if possible(&triangle) {
                count += 1;
            }
        }
    }
    count
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
