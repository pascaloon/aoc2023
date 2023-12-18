use std::collections::HashMap;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

// PARSING -------------------------------------

#[derive(Parser)]
#[grammar = "./day8_grammar.pest"]
pub struct InputFile;

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Clone, Debug)]
enum PathFunction {
    Const(u64),
    Linear(u64, u64),
    Any(Vec<PathFunction>)
}

impl PathFunction {
    pub fn len(&self) -> u64 {
        match &self {
            PathFunction::Const(i) => {*i}
            PathFunction::Linear(s, l) => {s + l},
            PathFunction::Any(funcs) => {
                funcs.iter().map(|f| f.len()).max().unwrap()
            }
        }
    }
    pub fn as_linear(&self) -> (u64, u64) {
        match &self {
            PathFunction::Linear(x, y) => (*x, *y),
            _ => panic!("..."),
        }
    }

    pub fn get_biggest(&self) -> &PathFunction {
        match &self {
            PathFunction::Const(_) => panic!("..."),
            PathFunction::Linear(_, _)  => &self,
            PathFunction::Any(funcs) => funcs.iter().max_by(|f1, f2| f1.len().cmp(&f2.len())).unwrap()
        }
    }
}

fn part2_inner(content: &str) -> u64 {
    let input = parse(content);
    const START_NODE: &'static str = "A";
    const END_NODE: &'static str = "Z";

    let lanes: Vec<&str> = input.nodes_map.keys()
        .filter(|k| k.ends_with(START_NODE))
        .cloned()
        .collect();

    let mut functions: Vec<PathFunction> = Vec::with_capacity(lanes.len());
    for lane in &lanes {
        let mut dir_idx = 0usize;
        let mut current_node = *lane;
        let mut path: Vec<(&str, usize)> = Vec::new();
        while !path.contains(&(current_node, dir_idx)) {
            path.push((current_node, dir_idx));
            let choices = input.nodes_map.get(current_node).expect("couldn't find node");
            let choice = dir_to_index(input.directions[dir_idx]);
            current_node = choices[choice];

            dir_idx += 1;
            if dir_idx >= input.directions.len() {
                dir_idx = 0;
            }

        }

        // println!("Path: {:?}", path);
        assert!(path.iter().any(|(p, _)| p.ends_with(END_NODE)));


        // Extract "functions" / "equations" out of loops / lanes
        // 1. No loop
        // 2. No Z
        // 3. N Z

        // 1 2 3 Z1 4 Z2 / 3 Z1 4 Z2 / 3 Z1 4 Z2
        // Offset: 1 2
        // Loop: 3 Z 4 Z
        // fz1(n) = Offset(lane_begin) + Offset(loop_begin) + loop_length * (n-1)
        // fz1(n) = loop_start + (z1_pos - loop_start) + loop_length * (n-1)
            // where n >= 1
        // fz1(1) = Offset + |3 Z1| = 2 + 2 = 4
        // fz1(2) = Offset + |3 Z1 4 Z2 3 Z1| = 2 + 6 = 8
        // fz1(3) = Offset + |3 Z1 4 Z2 3 Z1 4 Z2 3 Z1| = 2 + 10 = 12

        let loop_begin = path.iter().position(|x| x.eq(&(current_node, dir_idx))).unwrap();
        let loop_size = path.len() - loop_begin;

        let mut local_functions: Vec<PathFunction> = Vec::with_capacity(lanes.len());

        for (i, (n, _)) in path.iter().enumerate() {
            if n.ends_with(END_NODE) {
                if i < loop_begin {
                    local_functions.push(PathFunction::Const(i as u64))
                } else {
                    local_functions.push(PathFunction::Linear(i as u64, loop_size as u64))
                }
            }
        }

        if local_functions.len() > 1 {
            functions.push(PathFunction::Any(local_functions));
        } else if local_functions.len() == 1 {
            functions.push(local_functions[0].clone());
        }

    }

    solve_functions(&functions)
}

fn is_const(f: &&PathFunction) -> bool {
    match f {
        PathFunction::Const(_) => true,
        _ => false
    }
}
fn solve_functions(functions: &Vec<PathFunction>) -> u64 {
    let mut functions = functions.clone();

    // sort from biggest loop to smallest
    functions.sort_by(|a, b| b.len().cmp(&a.len()));

    // hope no constant fn
    assert_eq!(0, functions.iter().filter(is_const).count());

    let mut mult = 0;
    let mut done = false;
    let (biggest_loop_start, biggest_loop_len) = functions[0].get_biggest().as_linear();

    while !done {
        let target = biggest_loop_start + (mult * biggest_loop_len);
        done = true;
        for func in functions.iter().skip(1).map(|f| f) {
            done = solve_function(target, func);
            if !done {
                mult += 1;
                break;
            }
        }
    }

    biggest_loop_start + (mult * biggest_loop_len)

}

fn solve_function(target: u64, func: &PathFunction) -> bool {
    match func {
        PathFunction::Const(_) => {
            panic!("Not supported for now!");
        }
        PathFunction::Linear(start, len) => {
            let m = target - start;
            m % len == 0
        }
        PathFunction::Any(funcs) => {
            funcs.iter().any(|f| solve_function(target, f))
        }
    }
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

    static SAMPLE_3: &'static str = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn part2_sample3() {
        assert_eq!(6, part2_inner(SAMPLE_3));
    }

    // #[test]
    // fn part2_sample() {
    //     assert_eq!(5905, part2_inner(SAMPLE));
    // }
}
