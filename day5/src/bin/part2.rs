fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part2(&contents);
    println!("RESULT: {result}");
}

fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap();
    let seeds = populate_seeds(seeds_line);
    println!("{:?}", seeds);
    let maps = populate_maps(input);
    // let mut low = i64::MAX;
    // let mut low_seed = 0;
    let mut loc = 0;
    loop {
        let seed = location_to_seed(loc, &maps);
        for i in 0..seeds.len()/2 {
            if seed >= seeds[i*2] && seed < seeds[i*2]+seeds[i*2+1] {
                return loc
            }
        }
        loc += 1;
    }
    // for i in 0..seeds.len()/2 {
    //     for seed in seeds[i*2]..seeds[i*2]+seeds[i*2+1] {
    //         let location = seed_to_location(seed, &maps);
    //         if location < low {
    //             low = location;
    //             low_seed = seed;
    //         }
    //     }
    // }
    // low_seed
    
    // let locations: Vec<_> = seeds.iter().map(|seed| seed_to_location(*seed, &maps)).collect();
    // *locations.iter().min().unwrap()
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
                sections.push(curr_maps.clone());
            }
            curr_maps.clear(); 
            skip = true;
            continue;
        }
        if skip {
            skip = false;
            continue;
        }
        let nums: Vec<_> = line.split(" ").filter(|line| !line.is_empty()).map(|n| n.parse::<i64>().unwrap()).collect();
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

fn location_to_seed(location: i64, maps: &Vec<Vec<(i64, i64, i64)>>) -> i64 {
    let mut loc_mod = location;
    for sec in maps.iter().rev() {
        for map in sec.iter() {
            let src_rng_start = map.1;
            let dst_rng_start = map.0;
            let rng_len = map.2;
            if loc_mod >= dst_rng_start && loc_mod < dst_rng_start + rng_len {
                let offset = loc_mod - dst_rng_start;
                loc_mod = src_rng_start + offset;
                break;
            }
        }
    }
    loc_mod
}

#[cfg(test)]
mod tests {
    use crate::{seed_to_location, populate_maps, populate_seeds, location_to_seed};

    use super::part2;

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

        // let seeds = populate_seeds(input.lines().next().unwrap());
        // let maps = populate_maps(input);

        assert_eq!(part2(input), 46);
    }
}
