use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("input.txt").unwrap();
    let mut acc = 0;
    for line in file_contents.lines() {
        let mut d_arr = [None, None];
        for c in line.chars() {
            if c.is_digit(10) {
                if d_arr[0].is_none() {
                    d_arr[0] = Some(c)
                }
                d_arr[1] = Some(c);
            }
        }
        let n_str: String = d_arr.map(|e| e.unwrap()).iter().collect();
        let n: i32 = n_str.parse().unwrap();
        acc += n
    }
    println!("{}", acc);
}
