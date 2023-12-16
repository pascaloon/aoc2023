use std::collections::HashMap;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

// PARSING -------------------------------------

#[derive(Parser)]
#[grammar = "./day8_grammar.pest"]
pub struct InputFile;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Left,
    Right
}

#[derive(Debug)]
struct Input<'a> {
    pub directions: Vec<Dir>,
    pub nodes_map: HashMap<&'a str, Vec<&'a str>>
}

fn parse(content: &str) -> Input {
    let file = InputFile::parse(Rule::input, &content)
        .expect("couldn't parse content!")
        .next().unwrap();

    let mut directions = Vec::new();
    let mut nodes_map = HashMap::new();
    for r in file.into_inner() {
        match r.as_rule() {
            Rule::directions => { parse_directions(r, &mut directions); },
            Rule::node_map => { parse_nodes_map(r, &mut nodes_map); }
            _ => { }
        };
    }

    Input { directions, nodes_map }
}

fn parse_nodes_map<'a>(r: Pair<'a, Rule>, nodes_map: &mut HashMap<&'a str, Vec<&'a str>>) {
    let mut ids = Vec::with_capacity(3);
    for rr in r.into_inner() {
        match rr.as_rule() {
            Rule::id => { ids.push(rr.as_str()) },
            _ => unreachable!()
        };
    }

    assert_eq!(3, ids.len());

    let key = ids[0];
    let left_right = ids[1..].into_iter().cloned().collect();
    match nodes_map.get(key) {
        None => { nodes_map.insert(key, left_right); }
        Some(_) => { panic!("Found 2 lines with id '{}'", ids[0]); }
    }

}

fn parse_directions(r: Pair<Rule>, directions: &mut Vec<Dir>) {
    for rr in r.into_inner() {
        match rr.as_rule() {
            Rule::left => { directions.push(Dir::Left); },
            Rule::right => { directions.push(Dir::Right); },
            _ => unreachable!()
        };
    }
}

// PART 1 --------------------------------------

fn dir_to_index(dir: Dir) -> usize {
    match dir {
        Dir::Left => 0,
        Dir::Right => 1
    }
}

fn part1_inner(content: &str) -> u64 {
    let input = parse(content);

    const START_NODE: &'static str = "AAA";
    const END_NODE: &'static str = "ZZZ";

    let mut dir_idx = 0usize;
    let mut current_node = START_NODE;
    let mut steps = 0;
    while !current_node.eq(END_NODE) {
        let choices = input.nodes_map.get(current_node).expect("couldn't find node");
        let choice = dir_to_index(input.directions[dir_idx]);
        current_node = choices[choice];
        steps += 1;
        dir_idx += 1;
        if dir_idx >= input.directions.len() {
            dir_idx = 0;
        }
    }

    steps
}

pub fn part1(content: String) {
    println!("result: {}", part1_inner(&content));
}

// PART 2 --------------------------------------


fn part2_inner(content: &str) -> u64 {
    0
}

pub fn part2(content: String) {
    println!("result: {}", part2_inner(&content));
}


#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_1: &'static str = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

    #[test]
    fn part1_sample1() {
        assert_eq!(2, part1_inner(SAMPLE_1));
    }

    static SAMPLE_2: &'static str = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    #[test]
    fn part1_sample2() {
        assert_eq!(6, part1_inner(SAMPLE_2));
    }

    // #[test]
    // fn part2_sample() {
    //     assert_eq!(5905, part2_inner(SAMPLE));
    // }
}
