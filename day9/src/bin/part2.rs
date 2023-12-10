
fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part2(&contents);
    println!("RESULT {result}");
}

fn part2(input: &str) -> i32 {
    let mut acc = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
        let prev = prev_value(nums);
        acc += prev;
    }
    acc
}

fn prev_value(nums: Vec<i32>) -> i32 {
    let mut diffs = Vec::new();
    if nums.iter().all(|n| *n == 0) || nums.is_empty() {
        return 0
    }
    for i in 0..(nums.len() - 1) {
        diffs.push(nums[i+1] - nums[i]);
    }
    let out = nums.iter().next().unwrap_or(&0) - prev_value(diffs); 
    return out
}

#[cfg(test)]
mod tests{
    use super::part2;

    #[test]
    fn sample() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

	assert_eq!(part2(input), 2);
    }
}

