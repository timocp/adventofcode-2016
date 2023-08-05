#! /bin/sh

set -e

mkdir -p src

day=$1

if [ "$day" = "" ]; then
    day=$(TZ=US/Eastern date '+%d' | sed 's/^0//')
fi

src="src/day$day.rs"

if ! git diff --exit-code > /dev/null; then
    echo "There are uncommitted changes" 2>&1
    exit 1
fi
if [ -e "$src" ]; then
    echo "Already exists: $src" 2>&1
    exit 1
fi

../aoc-input/download.sh

echo "Creating $src..."
cat > "$src" <<EOF
use crate::Puzzle;

pub struct Solver {
    // input: Vec<i32>
}

impl Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            // input: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        "".to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}

#[test]
fn test() {
    let test_input = "\
";
    // assert_eq!()
}
EOF
