fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part2(&contents);
    println!("RESULT: {result}");
}

fn part2(input: &str) -> i64 {
    let time: &str = input
        .lines()
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap();
    let time = time
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("");
    let time: i64 = time.parse().unwrap();

    let distance: &str = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap();
    let distance = distance
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("");
    let distance: i64 = distance.parse().unwrap();

    let mut c = 0;
    for t in 0..time {
        let dist = t * (time - t);
        if dist > distance {
            c += 1;
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::part2;

    #[test]
    fn sample() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(part2(input), 71503)
    }
}
