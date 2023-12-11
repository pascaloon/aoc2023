use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

// PARSING -------------------------------------

type Card = usize;

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>
}

#[derive(Debug)]
struct Play {
    hand: Hand,
    bid: u64
}

fn parse(content: &str) -> Vec<Play> {
    //                          0    1    2    3    4    5    6    7    8    9   10   11    12   13
    const CARDS: [char; 14] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

    fn parse_hand(hand: &str) -> Hand {
        assert_eq!(5, hand.len());
        let cards = hand.chars()
            .map(|c| CARDS.iter().position(|cc| *cc == c).unwrap())
            .collect();
        Hand { cards }
    }
    fn parse_play(c: &str) -> Play {
        let parts: Vec<&str> = c.split(" ").collect();
        assert_eq!(2, parts.len());

        let hand = parse_hand(parts[0]);
        let bid = parts[1].parse().unwrap();
        Play { hand, bid }
    }

    content
        .replace("\r\n", "\n")
        .split("\n")
        .filter(|line|line.len() > 0)
        .map(parse_play)
        .collect()
}

// PART 1 --------------------------------------


fn count_cards(cards: &Vec<Card>) -> HashMap<Card, usize> {
    let mut map = HashMap::with_capacity(cards.len());
    for c in cards {
       if let Some(v) = map.get_mut(c) {
           *v += 1;
       } else {
           map.insert(c.clone(), 1);
       }
    }

    map
}

fn get_hand_type(h: &Hand) -> usize {
    let card_counts = count_cards(&h.cards);
    let card_types_count = card_counts.len();

    let pairs_count = card_counts.iter().filter(|(_, c)| **c == 2).count();
    let trio_count = card_counts.iter().filter(|(_, c)| **c == 3).count();

    match (card_types_count, pairs_count, trio_count) {
        (2, 0, 0) => 6, // Four of a kind
        (1, 0, 0) => 7, // Five of a kind
        (_, 0, 0) => 1, // High card
        (_, 1, 0) => 2, // One pair
        (_, 2, 0) => 3, // Two pairs
        (_, 0, 1) => 4, // Three of a kind
        (_, 1, 1) => 5, // Full house

        _ => unreachable!()
    }
}

fn sort_by_hand(h1: &Hand, h2: &Hand) -> Ordering {
    let ht1 = get_hand_type(h1);
    let ht2 = get_hand_type(h2);
    if ht1 < ht2 {
        Ordering::Less
    } else if ht1 > ht2 {
        Ordering::Greater
    } else {
        for (c1, c2) in h1.cards.iter().zip(h2.cards.iter()) {
            let cmp = c1.cmp(c2);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }
        unreachable!()
    }
}

fn part1_inner(content: &str) -> u64 {
    let mut plays = parse(content);
    println!("{:?}", plays);
    plays.sort_by(|p1, p2| sort_by_hand(&p1.hand, &p2.hand));
    plays.iter().enumerate()
        .fold(0, |acc, (i, play)| acc + ((i as u64 + 1)*play.bid))
}

pub fn part1(content: String) {
    println!("{}", part1_inner(&content));
}

// PART 2 --------------------------------------


fn part2_inner(content: &str) -> u64 {
    let mut plays = parse(content);
    println!("{:?}", plays);
    plays.sort_by(|p1, p2| sort_by_hand(&p1.hand, &p2.hand));
    plays.iter().enumerate()
        .fold(0, |acc, (i, play)| acc + ((i as u64 + 1)*play.bid))
}

pub fn part2(content: String) {
    println!("{}", part1_inner(&content));
}


#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &'static str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn part1_sample() {
        assert_eq!(765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5, part1_inner(SAMPLE));
    }

    #[test]
    fn part2_sample() {
        // assert_eq!(71503, part2_inner(SAMPLE));
    }
}
