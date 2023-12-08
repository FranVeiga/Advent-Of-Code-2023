fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part1(&contents);
    println!("RESULT: {result}");
}

fn part1(input: &str) -> i32 {
    let mut acc = 0;
    let lines: Vec<&str> = input.lines().collect();
    let line_length = lines[0].len();
    for i in 0..lines.len() {
        let mut j = 0;
        let mut chars = lines[i].chars();
        while let Some(char) = chars.next() {
            let mut valid = false;
            let start_idx: usize = j;
            let mut end_idx = j;
            if char.is_numeric() {
                while chars.next().unwrap_or('.').is_numeric() {
                    j += 1;
                    end_idx = j;
                }
                j += 1;
                let search_start_idx = start_idx.checked_sub(1).unwrap_or(0);
                let above = lines
                    .get(i.checked_sub(1).unwrap_or(100000000000))
                    .unwrap_or(&"")
                    .get(search_start_idx..=((line_length - 1).min(end_idx + 1)))
                    .unwrap_or("");
                let below = lines
                    .get(i + 1)
                    .unwrap_or(&"")
                    .get(search_start_idx..=((line_length - 1).min(end_idx + 1)))
                    .unwrap_or("");

                for above_char in above.chars() {
                    if !(above_char.is_numeric() || above_char == '.') {
                        valid = true;
                        break;
                    }
                }
                for below_char in below.chars() {
                    if !(below_char.is_numeric() || below_char == '.') {
                        valid = true;
                        break;
                    }
                }

                // println!("{} -> {}", start_idx.checked_sub(1).unwrap_or(0), start_idx);
                if &lines[i]
                    .get(start_idx.checked_sub(1).unwrap_or(1000000)..start_idx)
                    .unwrap_or(".")
                    != &"."
                    || &lines[i].get(end_idx + 1..end_idx + 2).unwrap_or(".") != &"."
                {
                    valid = true;
                }
            }

            if valid {
                let num_str = &lines[i][start_idx..=end_idx];
                let num = num_str.parse::<i32>().unwrap();
                acc += num;
            }

            j += 1;
        }
    }
    acc
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
