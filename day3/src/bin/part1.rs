fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part1(&contents);
    println!("RESULT: {result}");
}

fn part1(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let mut acc = 0;
    for i in 0..lines.len() {
        let chars: Vec<char> = lines[i].chars().collect();
        for j in 0..chars.len() {
            if chars[j] == '*' {
                let mut adjacent_numbers = Vec::new();
                let prev_line = lines.get(i-1).unwrap_or(&"");
                let next_line = lines.get(i+1).unwrap_or(&"");
                for c in (j-1)..=(j+1) {
                    if prev_line.chars().collect::<Vec<char>>()[c].is_numeric() {
                        let n = get_number(prev_line, c);
                        if !adjacent_numbers.contains(&n) {
                            adjacent_numbers.push(n);
                        }
                    }
                    if next_line.chars().collect::<Vec<char>>()[c].is_numeric() {
                        let n = get_number(next_line, c);
                        if !adjacent_numbers.contains(&n) {
                            adjacent_numbers.push(n);
                        }
                    }
                    if let Some(char) = chars.get(j+1) {
                        if char.is_numeric() {
                            let n = get_number(lines[i], j+1);
                            if !adjacent_numbers.contains(&n) {
                                adjacent_numbers.push(n);
                            }
                        }
                    }
                    if let Some(char) = chars.get(j-1) {
                        if char.is_numeric() {
                            let n = get_number(lines[i], j-1);
                            if !adjacent_numbers.contains(&n) {
                                adjacent_numbers.push(n);
                            }
                        }
                    }
                } 
                acc += adjacent_numbers.iter().sum::<i32>();
            }
        }
    }
    acc
}

fn get_number(line: &str, idx: usize) -> i32 {

    let chars: Vec<char> = line.chars().collect();
    let mut start_idx = 0;
    let mut end_idx = 0;
    let mut parsing = false;

    for i in 0..chars.len() {
        if chars[i].is_numeric() && !parsing {
            start_idx = i;
            end_idx = i;
            parsing = true;
        } else if chars[i].is_numeric() && parsing {
            end_idx = i;
        } else if !chars[i].is_numeric() && parsing {
            parsing = false;
        }

        if !parsing && idx >= start_idx && idx <= end_idx {
            break;
        }
    }

    line[start_idx..=end_idx].parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn sample() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(input), 4361)
    }
}
