fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part1(&contents);
    println!("RESULT {result}");
}

fn part1(input: &str) -> i32 {
    let mut acc = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
        let next = next_value(nums);
        acc += next;
    }
    acc
}

fn next_value(nums: Vec<i32>) -> i32 {
    let mut diffs = Vec::new();
    if nums.iter().all(|n| *n == 0) || nums.is_empty() {
        return 0
    }
    for i in 0..(nums.len() - 1) {
        diffs.push(nums[i+1] - nums[i]);
    }
    let out = nums.iter().last().unwrap() + next_value(diffs); 
    return out
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn sample() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

        assert_eq!(part1(input), 114);
    }
}
