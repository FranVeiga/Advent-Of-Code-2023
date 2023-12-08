use std::collections::HashMap;
fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part1(&contents);
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

fn part1(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().skip(2).map(|line| parse_node(line)).collect();

    let target = "ZZZ";
    let mut curr = "AAA";

    let lines: HashMap<String, (String, String)> = HashMap::from_iter(lines.into_iter());
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
    let mut step = 0;
    loop {
        let dir = directions[step % directions.len()];
        let options = lines.get(curr).unwrap();
        curr = match dir {
            Dir::Right => &options.1,
            Dir::Left => &options.0,
        };
        step += 1;
        if curr == target {
            break;
        }
    }

    step as i32
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn sample() {
        let input1 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input1), 6);
    }

    #[test]
    fn sample2() {
        let input2 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input2), 2);
    }
}
