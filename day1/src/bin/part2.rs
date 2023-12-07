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
        if nline.get(i..i+1).unwrap().chars().next().unwrap().is_digit(10) {
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
        if nline.get((nline.len()-i-1)..(nline.len()-i)).unwrap().chars().next().unwrap().is_digit(10) {
            break;
        }
        if let Some(result) = replace_number(&nline, &nline[nline.len()-1-i..]) {
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
    println!("{input}");
    if substr.starts_with("one") {
        Some(input.replace("one", "1"))
    }
    else if substr.starts_with("two") {
        Some(input.replace("two", "2"))
    }
    else if substr.starts_with("three") {
        Some(input.replace("three", "3"))
    }
    else if substr.starts_with("four") {
        Some(input.replace("four", "4"))
    }
    else if substr.starts_with("five") {
        Some(input.replace("five", "5"))
    }
    else if substr.starts_with("six") {
        Some(input.replace("six", "6"))
    }
    else if substr.starts_with("seven") {
        Some(input.replace("seven", "7"))
    }
    else if substr.starts_with("eight") {
        Some(input.replace("eight", "8"))
    }
    else if substr.starts_with("nine") {
        Some(input.replace("nine", "9"))
    }
    else {
        None
    }
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
