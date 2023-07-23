use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&input),
            Part::Two => 0,
        }
    )
}

fn part1(input: &[Instruction]) -> u32 {
    let mut state = State::new();
    for instruction in input {
        state.perform(instruction);
    }
    state.position.0.abs() as u32 + state.position.1.abs() as u32
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match turn {
            Turn::Left => match self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            Turn::Right => match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        }
    }
}

struct State {
    direction: Direction,
    position: (i32, i32),
}

impl State {
    fn new() -> Self {
        Self {
            direction: Direction::North,
            position: (0, 0),
        }
    }

    fn perform(&mut self, instruction: &Instruction) {
        self.direction = self.direction.turn(instruction.turn);
        match self.direction {
            Direction::North => self.position.1 += instruction.walk as i32,
            Direction::East => self.position.0 += instruction.walk as i32,
            Direction::South => self.position.1 -= instruction.walk as i32,
            Direction::West => self.position.0 -= instruction.walk as i32,
        };
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Right,
}

struct Instruction {
    turn: Turn,
    walk: u32,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim_end_matches('\n')
        .split(", ")
        .map(|instruction| Instruction {
            turn: match instruction.chars().next().unwrap() {
                'L' => Turn::Left,
                'R' => Turn::Right,
                _ => panic!("Invalid turn"),
            },
            walk: instruction[1..].parse().unwrap(),
        })
        .collect()
}

#[test]
fn test() {
    assert_eq!(5, part1(&parse_input("R2, L3")));
    assert_eq!(2, part1(&parse_input("R2, R2, R2")));
    assert_eq!(12, part1(&parse_input("R5, L5, R5, R3")));
}
