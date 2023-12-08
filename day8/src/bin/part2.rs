use std::collections::HashMap;
fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part2(&contents);
    println!("RESULT: {result}");
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Right,
    Left,
}

type Node = (String, (String, String));

fn parse_node(line: &str) -> Node {
    let label = line.split(" ").next().unwrap().to_string();
    let options = line.split("=").skip(1).next().unwrap().trim();
    let options: Vec<_> = options.split(", ").collect();
    let options = (
        options[0].get(1..).unwrap().to_string(),
        options[1].get(..3).unwrap().to_string(),
    );
    (label, options)
}

fn gcd(x: usize, y: usize) -> usize {
    if x == 0 {
        y
    } else if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}

fn lcm_mult(mut nums: Vec<usize>) -> usize {
    if nums.is_empty() {
        1
    } else {
        lcm(nums.pop().unwrap(), lcm_mult(nums))
    }
}

fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().skip(2).map(|line| parse_node(line)).collect();

    let target = "ZZZ";
    let mut curr: Vec<_> = lines
        .iter()
        .filter(|n| n.0.ends_with("A"))
        .map(|n| n.clone())
        .collect();

    let lines: HashMap<String, (String, String)> = HashMap::from_iter(lines.clone().into_iter());
    let directions: Vec<_> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Dir::Right,
            'L' => Dir::Left,
            _ => panic!("invalid direction"),
        })
        .collect();

    // =========== MY ORIGINAL APPROACH =================
    // Takes too long to compute, but should work
    //
    // let mut step = 0;
    // loop {
    //     let mut new_curr = Vec::new();
    //     for node in curr.iter() {
    //         let dir = directions[step % directions.len()];
    //         let options = lines.get(*node).unwrap();
    //         new_curr.push(
    //             match dir {
    //                 Dir::Right => &options.1,
    //                 Dir::Left => &options.0,
    //             }
    //         );
    //     }
    //     curr = new_curr;
    //     step += 1;
    //     if curr.iter().all(|node| node.ends_with("Z")) {
    //         break;
    //     }
    // }
    // step as i32

    // ============ LCM approach (non-general solution) =======================
    // As suggested in https://www.reddit.com/r/adventofcode/comments/18dg1hw/2023_day_8_part_2_about_the_correctness_of_a/
    let steps_individual: Vec<_> = curr
        .iter()
        .map(|node| {
            let mut step = 0;
            let mut node = &node.0;
            loop {
                let dir = directions[step % directions.len()];
                let options = lines.get(node).unwrap();
                node = match dir {
                    Dir::Right => &options.1,
                    Dir::Left => &options.0,
                };
                step += 1;
                if node.ends_with('Z') {
                    break;
                }
            }
            step
        })
        .collect();

    println!("{:?}", steps_individual);
    lcm_mult(steps_individual)
}

#[cfg(test)]
mod tests {
    use super::part2;

    #[test]
    fn sample() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(part2(input), 66);
    }
}
