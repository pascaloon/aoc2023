use std::cmp::Ordering;
use std::collections::{HashMap};

// PARSING -------------------------------------

type Card = usize;

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>
}

#[derive(Debug)]
struct Play {
    hand: Hand,
    bid: u64
}

fn parse(content: &str, deck: &'static [char]) -> Vec<Play> {
    let parse_hand = |hand: &str| -> Hand {
        assert_eq!(5, hand.len());
        let cards = hand.chars()
            .map(|c| deck.iter().position(|cc| *cc == c).unwrap())
            .collect();
        Hand { cards }
    };
    let parse_play = |c: &str| -> Play {
        let parts: Vec<&str> = c.split(" ").collect();
        assert_eq!(2, parts.len());

        let hand = parse_hand(parts[0]);
        let bid = parts[1].parse().unwrap();
        Play { hand, bid }
    };

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
    //                          0    1    2    3    4    5    6    7    8    9   10   11   12
    const CARDS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    let mut plays = parse(content, &CARDS);
    plays.sort_by(|p1, p2| sort_by_hand(&p1.hand, &p2.hand));
    plays.iter().enumerate()
        .fold(0, |acc, (i, play)| acc + ((i as u64 + 1)*play.bid))
}

pub fn part1(content: String) {
    println!("{}", part1_inner(&content));
}

// PART 2 --------------------------------------

fn get_best_hand_type(h: &Hand) -> usize {
    let jk = h.cards.iter().position(|c| *c == 0);
    match jk {
        None => get_hand_type(h),
        Some(i) => {
            let mut h2 = h.clone();
            let mut max = get_hand_type(h);
            for card_value in 1..13 {
                h2.cards[i] = card_value;
                let val = get_best_hand_type(&h2);
                if val > max {
                    max = val;
                }
            }
            max
        }
    }
}

fn sort_by_hand_joker(h1: &Hand, h2: &Hand) -> Ordering {
    let ht1 = get_best_hand_type(h1);
    let ht2 = get_best_hand_type(h2);
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

fn part2_inner(content: &str) -> u64 {
    //                          0    1    2    3    4    5    6    7    8    9   10   11   12
    const CARDS: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
    let mut plays = parse(content, &CARDS);
    plays.sort_by(|p1, p2| sort_by_hand_joker(&p1.hand, &p2.hand));
    plays.iter().enumerate()
        .fold(0, |acc, (i, play)| acc + ((i as u64 + 1)*play.bid))
}

pub fn part2(content: String) {
    println!("{}", part2_inner(&content));
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
        assert_eq!(5905, part2_inner(SAMPLE));
    }
}
