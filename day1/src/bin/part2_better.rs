use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("input.txt").unwrap();
    let mut acc = 0;
    for line in file_contents.lines() {
        acc += process_line(line);
    }

    println!("");
    println!("{}", acc);
}

fn process_line(line: &str) -> i32 {
    // let line = line.replace("one", "1")
    //     .replace("two", "2")
    //     .replace("three", "3")
    //     .replace("four", "4")
    //     .replace("five", "5")
    //     .replace("six", "6")
    //     .replace("seven", "7")
    //     .replace("eight", "8")
    //     .replace("nine", "9");
    // println!("{line}");
    let mut i = 0;
    let mut nline = line.to_string();
    while i < nline.len() {
        if nline
            .get(i..i + 1)
            .unwrap()
            .chars()
            .next()
            .unwrap()
            .is_digit(10)
        {
            break;
        }
        if let Some(result) = replace_number(&nline, &nline[i..]) {
            nline = result;
            break;
        }
        i += 1;
    }

    let mut i = 0;
    while i < nline.len() {
        if nline
            .get((nline.len() - i - 1)..(nline.len() - i))
            .unwrap()
            .chars()
            .next()
            .unwrap()
            .is_digit(10)
        {
            break;
        }
        if let Some(result) = replace_number(&nline, &nline[nline.len() - 1 - i..]) {
            nline = result;
            break;
        }
        i += 1;
    }

    println!("Transformed string: {nline}");

    let mut d_arr = [None, None];
    for c in nline.chars() {
        if c.is_digit(10) {
            if d_arr[0].is_none() {
                d_arr[0] = Some(c)
            }
            d_arr[1] = Some(c);
        }
    }
    let n_str: String = d_arr.map(|e| e.unwrap()).iter().collect();
    let n: i32 = n_str.parse().unwrap();
    n
}

fn replace_number(input: &str, substr: &str) -> Option<String> {
    let numbers = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    for number in numbers {
        if substr.starts_with(number.0) {
            return Some(input.replace(number.0, number.1));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::process_line;
    #[test]
    fn sample() {
        // two1nine
        // eightwothree
        // abcone2threexyz
        // xtwone3four
        // 4nineeightseven2
        // zoneight234
        // 7pqrstsixteen
        // 29, 83, 13, 24, 42, 14, and 76
        assert_eq!(process_line("two1nine"), 29);
        assert_eq!(process_line("eightwothree"), 83);
        assert_eq!(process_line("abcone2threexyz"), 13);
        assert_eq!(process_line("xtwone3four"), 24);
        assert_eq!(process_line("4nineeightseven2"), 42);
        assert_eq!(process_line("zoneight234"), 14);
        assert_eq!(process_line("7pqrstsixteen"), 76);
        assert_eq!(process_line("threeeight"), 38);
    }
}
