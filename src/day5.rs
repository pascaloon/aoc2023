use std::cmp::Ordering;
use std::collections::HashMap;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

// PARSING ---------------------------------------
#[derive(Parser)]
#[grammar = "./day5_grammar.pest"]
pub struct File;

#[derive(Debug)]
struct IdMap {
    source_range_start: u64,
    target_range_start: u64,
    range_length: u64,
}
#[derive(Debug)]
struct CategoryMap<'a> {
    source_category_name: &'a str,
    target_category_name: &'a str,
    maps: Vec<IdMap>
}

#[derive(Debug)]
struct Input<'a> {
    seeds: Vec<u64>,
    maps: Vec<CategoryMap<'a>>,
    categories: HashMap<&'a str, usize>
}

fn parse(content: &str) -> Input {
    let file = File::parse(Rule::file, &content)
        .expect("couldn't parse content!")
        .next().unwrap();

    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<CategoryMap> = Vec::new();
    let mut categories: HashMap<&str, usize> = HashMap::new();
    for r in file.into_inner() {
        match r.as_rule() {
            Rule::seed_id => { seeds.push(parse_rule_as_u64(r)); },
            Rule::category => {
                maps.push(parse_category_map(r));
                let cat = maps.last().unwrap();
                categories.insert(cat.source_category_name, maps.len() - 1);
            },
            _ => { }
        };
    }

    Input { seeds, maps, categories }
}

fn parse_category_map(map_rule: Pair<Rule>) -> CategoryMap {
    let mut source = None;
    let mut target = None;
    let mut maps = Vec::new();

    for r in map_rule.into_inner() {
        match r.as_rule() {
            Rule::source_category_name => { source = Some(r.as_str()); }
            Rule::target_category_name => { target = Some(r.as_str()); }
            Rule::category_ids_map => { maps.push(parse_category_ids_map(r)); }
            _ => { }
        };
    }

    CategoryMap { source_category_name: source.unwrap(), target_category_name: target.unwrap(), maps}
}

fn parse_category_ids_map(id_map_rule: Pair<Rule>) -> IdMap {
    let mut source_range_start = 0;
    let mut target_range_start = 0;
    let mut range_length = 0;

    for r in id_map_rule.into_inner() {
        match r.as_rule() {
            Rule::source_range_start => { source_range_start = parse_rule_as_u64(r); }
            Rule::target_range_start => { target_range_start = parse_rule_as_u64(r); }
            Rule::range_length => { range_length = parse_rule_as_u64(r); }
            _ => { }
        };
    }

    IdMap { source_range_start, target_range_start, range_length }
}

fn parse_rule_as_u64(r: Pair<Rule>) -> u64 {
    r.as_str().parse().unwrap()
}

// SHARED --------------------------------------

// PART 1 --------------------------------------


fn crawl_to_location(init_id: u64, input: &Input) -> u64 {
    const INIT_CATEGORY: &str = "seed";
    const END_CATEGORY: &str = "location";

    let mut id = init_id;
    let mut cat_name = INIT_CATEGORY;
    while cat_name.cmp(END_CATEGORY) != Ordering::Equal {
        let cat_map: &CategoryMap = input.maps.get(*input.categories.get(cat_name).unwrap()).unwrap();
        cat_name = cat_map.target_category_name;
        for range in &cat_map.maps {
            if range.source_range_start <= id && id <= (range.source_range_start + range.range_length) {
                id = range.target_range_start + (id - range.source_range_start);
                break;
            }
        }
    }

    id
}

fn part1_inner(content: &str) -> u64 {
    let input = parse(content);
    input.seeds.iter()
        .map(|s| crawl_to_location(*s, &input))
        .min().unwrap()
}

pub fn part1(content: String) {
    println!("{}", part1_inner(&content));
}

// PART 2 --------------------------------------


fn part2_inner(content: &str) -> u64 {
    0
}

pub fn part2(content: String) {
    println!("{0}", part2_inner(&content));
}


// TESTS ----------------------------------------

#[cfg(test)]
mod tests {
    use crate::day5::*;

    static SAMPLE: &'static str = r#"
seeds: 79 14 55 13

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
56 93 4
"#;

    #[test]
    fn part1_sample() {
        assert_eq!(35, part1_inner(SAMPLE));
    }

    #[test]
    fn part2_sample() {
        // assert_eq!(30, part2_inner(SAMPLE));
    }
}