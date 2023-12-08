fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part1(&contents);
    println!("RESULT: {result}");
}

fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap();
    let seeds = populate_seeds(seeds_line);
    println!("{:?}", seeds);
    let maps = populate_maps(input);
    let locations: Vec<_> = seeds
        .iter()
        .map(|seed| seed_to_location(*seed, &maps))
        .collect();
    *locations.iter().min().unwrap()
}

fn populate_seeds(seeds: &str) -> Vec<i64> {
    let nums = seeds.split(": ").skip(1).next().unwrap();
    nums.split(" ").map(|n| n.parse::<i64>().unwrap()).collect()
}

fn populate_maps(input: &str) -> Vec<Vec<(i64, i64, i64)>> {
    let mut lines = input.lines().skip(1);
    let mut sections = vec![];

    let mut curr_maps = vec![];
    let mut skip = false;
    while let Some(line) = lines.next() {
        if line == "" {
            if !curr_maps.is_empty() {
                println!("");
                sections.push(curr_maps.clone());
            }
            curr_maps.clear();
            skip = true;
            continue;
        }
        if skip {
            println!("Skipped line: {line}");
            skip = false;
            continue;
        }
        println!("{line}");
        let nums: Vec<_> = line
            .split(" ")
            .filter(|line| !line.is_empty())
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        curr_maps.push((nums[0], nums[1], nums[2]))
    }
    if !curr_maps.is_empty() {
        sections.push(curr_maps.clone());
    }
    sections
}

fn seed_to_location(seed: i64, maps: &Vec<Vec<(i64, i64, i64)>>) -> i64 {
    let mut seed_mod = seed;
    for sec in maps.iter() {
        for map in sec.iter() {
            let src_rng_start = map.1;
            let dst_rng_start = map.0;
            let rng_len = map.2;
            if seed_mod >= src_rng_start && seed_mod <= src_rng_start + rng_len - 1 {
                let offset = seed_mod - src_rng_start;
                seed_mod = dst_rng_start + offset;
                break;
            }
        }
    }
    seed_mod
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn sample() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part1(input), 35);
    }
}
