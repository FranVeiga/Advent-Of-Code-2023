fn main() {
    let games = std::fs::read_to_string("input.txt").unwrap();
    let possible_games: Vec<&str> = games.lines().filter(|game| is_possible(game)).collect();
    let sum_of_ids = sum_ids(possible_games);
    println!("Sum of ids of possible games: {sum_of_ids}");
}

fn is_possible(game: &str) -> bool {
    let reds = 12;
    let greens = 13;
    let blues = 14;

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
                        return false;
                    }
                }
                "blue" => {
                    if cube.0 > blues {
                        return false;
                    }
                }
                "green" => {
                    if cube.0 > greens {
                        return false;
                    }
                }
                _ => panic!("unexpected color"),
            }
        }
    }

    true
}

fn sum_ids(games: Vec<&str>) -> i32 {
    games
        .iter()
        .map(|line| {
            line.split(":")
                .next()
                .unwrap()
                .split(" ")
                .skip(1)
                .next()
                .expect("Should have game Id")
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}
