fn main() {
    let games = std::fs::read_to_string("input.txt").unwrap();
    let powers_sum: i32 = games.lines().map(|game| power(game)).sum();
    println!("Sum of powers of minimums: {powers_sum}");
}

fn power(game: &str) -> i32 {
    let mut reds = 0;
    let mut greens = 0;
    let mut blues = 0;

    let hands = game.split(":").skip(1).next().unwrap().trim();
    let hands: Vec<_> = hands.split(";").map(|hand| hand.trim()).collect();

    for hand in hands {
        let cubes = hand.split(",").map(|cube| {
            let cube: Vec<_> = cube.trim().split(" ").collect();
            (cube[0].parse::<i32>().unwrap(), cube[1])
        });

        for cube in cubes {
            match cube.1 {
                "red" => {
                    if cube.0 > reds {
                        reds = cube.0
                    }
                }
                "green" => {
                    if cube.0 > greens {
                        greens = cube.0
                    }
                }
                "blue" => {
                    if cube.0 > blues {
                        blues = cube.0
                    }
                }
                _ => panic!("unexpected color"),
            }
        }
    }
    reds * greens * blues
}
