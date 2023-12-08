use std::{cmp::Ordering, collections::HashMap, ops::Add};

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let result = part1(&contents);
    println!("RESULT: {result}");
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    Pair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    hand: String,
    bid: i32,
}

impl Hand {
    fn new(hand: String, bid: i32) -> Hand {
        Hand { hand, bid }
    }

    fn get_type(&self) -> HandType {
        let mut hmap: HashMap<char, usize> = HashMap::new();
        for card in self.hand.chars() {
            hmap.entry(card).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut values: Vec<_> = hmap.into_values().collect();
        values.sort();
        values.reverse();
        match values[0] {
            5 => HandType::FiveKind,
            4 => HandType::FourKind,
            3 if values[1] == 2 => HandType::FullHouse,
            3 => HandType::ThreeKind,
            2 if values[1] == 2 => HandType::TwoPair,
            2 => HandType::Pair,
            _ => HandType::HighCard,
        }
    }

    fn has_better_cards(&self, other: &Hand) -> bool {
        let card_order = "23456789TJQKA";
        let hand: Vec<_> = self.hand.chars().collect();
        let other_hand: Vec<_> = other.hand.chars().collect();
        for i in 0..5 {
            let p1 = card_order.find(hand[i]);
            let p2 = card_order.find(other_hand[i]);
            if !(p1 == p2) {
                return p1 > p2;
            }
        }
        panic!("Same cards");
    }
}

fn part1(input: &str) -> i32 {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").filter(|l| !l.is_empty()).collect();
            let hand = parts[0].to_string();
            let bid = parts[1].parse().unwrap();
            Hand::new(hand, bid)
        })
        .collect();
    hands.sort_unstable_by(|a, b| {
        let ord = b.get_type().cmp(&a.get_type());
        match ord {
            Ordering::Equal if a.has_better_cards(b) => Ordering::Greater,
            Ordering::Equal if b.has_better_cards(a) => Ordering::Less,
            Ordering::Less | Ordering::Greater => ord,
            _ => panic!("something went wrong"),
        }
    });
    println!("{:?}", hands);

    hands
        .iter()
        .enumerate()
        .fold(0, |prev, (i, h)| prev + h.bid * (i + 1) as i32)
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn sample() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(part1(input), 6440);
    }
}
