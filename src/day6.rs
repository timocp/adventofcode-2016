use crate::Puzzle;

pub struct Solver {
    codes: Vec<String>,
}

impl Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            codes: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        most_common_letters(&self.codes)
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}

fn most_common_letters(codes: &[String]) -> String {
    let counts = codes
        .iter()
        .fold(vec![vec![0; 26]; codes[0].len()], |mut counts, code| {
            for (i, c) in code.chars().enumerate() {
                counts[i][c as usize - 'a' as usize] += 1;
            }
            counts
        });

    counts
        .iter()
        .map(|count| {
            let mut max = 0;
            let mut max_index = 0;
            for (i, &c) in count.iter().enumerate() {
                if c > max {
                    max = c;
                    max_index = i;
                }
            }
            max_index
        })
        .map(|i| (i + 'a' as usize) as u8 as char)
        .collect()
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[test]
fn test() {
    let test_input = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
";
    assert_eq!("easter", Solver::new(test_input).part1());
}
