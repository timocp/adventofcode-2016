use std::io::Read;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

trait Puzzle {
    fn new(input: &str) -> Self;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() == 2 {
        run(args[1].parse().unwrap());
    } else {
        let t0 = Instant::now();
        for day in 1..=25 {
            run(day);
        }
        println!(
            "{:>80}",
            format!("TOTAL: {:.3}s", t0.elapsed().as_secs_f64())
        );
    }
}

macro_rules! run {
    ($day:expr, $solver:ty, $input:expr) => {{
        let t0 = Instant::now();
        let solver = <$solver>::new($input);
        measure(&format!("Day {:02}, part 1", $day), t0, || solver.part1());
        let t0 = Instant::now();
        measure(&format!("Day {:02}, part 2", $day), t0, || solver.part2());
    }};
}

fn run(day: usize) {
    let filename = format!("input/day{}.txt", day);
    if let Ok(input) = read_file(&filename) {
        match day {
            1 => run!(day, day1::Solver, &input),
            2 => run!(day, day2::Solver, &input),
            3 => run!(day, day3::Solver, &input),
            4 => run!(day, day4::Solver, &input),
            5 => run!(day, day5::Solver, &input),
            6 => run!(day, day6::Solver, &input),
            7 => run!(day, day7::Solver, &input),
            8 => run!(day, day8::Solver, &input),
            _ => (),
        }
    }
}

fn measure<F>(label: &str, t0: Instant, f: F)
where
    F: FnOnce() -> String,
{
    print!("{}: ", label);
    let result = f();
    println!(
        "{:56} {1:.3}s",
        if result.contains('\n') {
            result.lines().next().unwrap()
        } else {
            &result
        },
        t0.elapsed().as_secs_f64()
    );
}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut f = std::fs::File::open(filename)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    Ok(input)
}
