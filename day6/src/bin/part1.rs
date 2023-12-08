fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part1(&contents);
    println!("RESULT: {result}");
}

fn part1(input: &str) -> i32 {
    let times: &str = input
        .lines()
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap();
    let times: Vec<_> = times
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let distances: &str = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap();
    let distances: Vec<i32> = distances
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    assert!(times.len() == distances.len());
    let n_races = times.len();
    let mut acc = 1; // may fail if there are none ways to win the race, but I don't really care.

    for i in 0..n_races {
        let mut c = 0;
        let total_time = times[i];
        let record_distance = distances[i];
        for time in 0..total_time {
            let dist = time * (total_time - time);
            if dist > record_distance {
                c += 1;
            }
        }
        println!("{c}");
        acc = acc * c;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn sample() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(part1(input), 288)
    }
}
