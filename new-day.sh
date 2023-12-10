#!/bin/bash

day=$1

cargo new --vcs none "$day"
rm -f "$day/src/main.rs"
mkdir -p "$day/src/bin"
echo "
fn main() {
    let contents = std::fs::read_to_string(\"input.txt\").unwrap();
    let result = part1(&contents);
    println!(\"RESULT {result}\");
}

fn part1(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests{
    use super::part1;

    #[test]
    fn sample() {
        let input = \"\";

	assert_eq!(part1(input), RESULT);
    }
}
" > "$day/src/bin/part1.rs"

echo "
fn main() {
    let contents = std::fs::read_to_string(\"input.txt\").unwrap();
    let result = part2(&contents);
    println!(\"RESULT {result}\");
}

fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests{
    use super::part2;

    #[test]
    fn sample() {
        let input = \"\";

	assert_eq!(part2(input), RESULT);
    }
}
" > "$day/src/bin/part2.rs"
