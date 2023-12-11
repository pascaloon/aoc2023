use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

// PARSING ---------------------------------------
#[derive(Parser)]
#[grammar = "./day4_grammar.pest"]
pub struct Cards;

#[derive(Debug)]
struct Card {
    _id: i32,
    winning_nums: Vec<i32>,
    nums: Vec<i32>
}

fn parse(content: &str) -> Vec<Card> {
    let cards_file = Cards::parse(Rule::cards, &content)
        .expect("couldn't parse content!")
        .next().unwrap();

    let cards = cards_file.into_inner()
        .filter(|r| r.as_rule() == Rule::card)
        .map(|r| parse_card(r))
        .collect();

    cards
}

fn parse_rule_as_i32(r: Pair<Rule>) -> i32 {
    r.as_str().parse().unwrap()
}
fn parse_card(card: Pair<Rule>) -> Card {
    let mut winning_nums = Vec::new();
    let mut nums = Vec::new();
    let mut id: Option<i32> = None;
    for r in card.into_inner() {
        match r.as_rule() {
            Rule::num => { nums.push(parse_rule_as_i32(r)); }
            Rule::winning_num => { winning_nums.push(parse_rule_as_i32(r)); }
            Rule::card_id => { id = Some(parse_rule_as_i32(r)); }
            _ => {}
        }
    }

    Card {_id: id.unwrap(), winning_nums, nums}
}

// SHARED --------------------------------------

fn count_winning_nums(card: &Card) -> u32 {
    card.nums.iter()
        .filter(|num| card.winning_nums.contains(num))
        .count() as u32
}


// PART 1 --------------------------------------
fn part1_inner(content: &str) -> i32 {
    parse(content).iter()
        .map(count_winning_nums)
        .map(|w| match w { 0 => 0, _ => 2i32.pow(w-1)})
        .sum()
}

pub fn part1(content: String) {
    println!("result: {}", part1_inner(&content));
}

// PART 2 --------------------------------------


fn part2_inner(content: &str) -> i32 {
    let cards = parse(content);
    let mut queue: Vec<usize> = (0..cards.len()).collect();
    queue.sort();

    let mut qi = 0usize;
    while qi < queue.len() {
        let card_index= queue[qi];
        let card = &cards[card_index];
        let winning_nums = count_winning_nums(card) as usize;
        for wi in 0..winning_nums {
            let next_card_index = card_index + wi + 1usize;
            if next_card_index < cards.len() {
                queue.insert(qi + wi + 1, next_card_index);
            }
        }

        qi += 1;
    }

    queue.len() as i32
}

pub fn part2(content: String) {
    println!("result: {}", part2_inner(&content));
}


// TESTS ----------------------------------------

#[cfg(test)]
mod tests {
    use crate::day4::*;

    static SAMPLE: &'static str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn part1_sample() {
        assert_eq!(13, part1_inner(SAMPLE));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(30, part2_inner(SAMPLE));
    }
}