#[derive(Clone, Debug)]
struct Card {
    id: u32,
    winners: Vec<i32>,
    nums: Vec<i32>,
}

impl Card {
    fn new(card: &str) -> Card {
        let parts: Vec<_> = card.split(":").collect();
        let id = parts[0].split(" ").last().unwrap().parse().unwrap();
        let numbers: Vec<_> = parts[1].split(" | ").collect();
        let winners: Vec<i32> = numbers[0]
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let nums: Vec<i32> = numbers[1]
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();

        Card { id, winners, nums }
    }

    fn matching(&self) -> u32 {
        let mut c = 0;
        for num in self.nums.iter() {
            if self.winners.contains(&num) {
                c += 1;
            }
        }
        c
    }

    fn get_copies(&self, card_set: &Vec<Card>) -> Vec<Card> {
        let mut copies = vec![];
        for i in 0..self.matching() {
            copies.push(card_set[(self.id + i) as usize].clone())
        }

        copies
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part2(&contents);
    println!("RESULT: {result}");
}

fn part2(input: &str) -> usize {
    let original_cards: Vec<_> = input.lines().map(|line| Card::new(line)).collect();
    let mut cards = original_cards.to_vec();

    let mut i = 0;
    while let Some(card) = cards.get(i) {
        for card_copy in card.get_copies(&original_cards) {
            cards.push(card_copy);
        }

        i += 1;
    }

    cards.len()
}

#[cfg(test)]
mod tests {

    use super::part2;

    #[test]
    fn sample() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(input), 13)
    }
}
