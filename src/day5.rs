use std::collections::HashMap;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use rayon::prelude::*;

// PARSING ---------------------------------------
#[derive(Parser)]
#[grammar = "./day5_grammar.pest"]
pub struct InputFile;

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
    let file = InputFile::parse(Rule::file, &content)
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

fn crawl_to_location(init_id: u64, input: &Input) -> u64 {
    const INIT_CATEGORY: &str = "seed";
    const END_CATEGORY: &str = "location";

    let mut id = init_id;

    let mut cat_name = INIT_CATEGORY;
    while !cat_name.eq(END_CATEGORY) {
        let cat_map: &CategoryMap = input.maps.get(*input.categories.get(cat_name).unwrap()).unwrap();
        cat_name = cat_map.target_category_name;
        for range in &cat_map.maps {
            if range.source_range_start <= id && id < (range.source_range_start + range.range_length) {
                id = range.target_range_start + (id - range.source_range_start);
                break;
            }
        }
    }

    // println!("{init_id} -> {id}");

    id
}

// PART 1 --------------------------------------

fn part1_inner(content: &str) -> u64 {
    let input = parse(content);
    input.seeds.iter()
        .map(|s| crawl_to_location(*s, &input))
        .min().unwrap()
}

pub fn part1(content: String) {
    println!("result: {}", part1_inner(&content));
}

// PART 2 --------------------------------------

fn part2_inner(content: &str) -> u64 {
    let input = parse(content);
    assert_eq!(input.seeds.len() % 2, 0);

    let mut locations = Vec::with_capacity(input.seeds.len() / 2);
    let mut i = 0;
    // println!("{:?}", input.seeds);

    while i < input.seeds.len() {
        let start = *input.seeds.get(i).unwrap();
        let count = *input.seeds.get(i + 1).unwrap();
        let end = start + count;

        let min = (start..end)
            .into_par_iter()
            .map(|id| crawl_to_location(id, &input))
            .min()
            .unwrap();

        locations.push(min);

        i += 2;
    }

    locations.into_iter()
        .min()
        .unwrap()
}

pub fn part2(content: String) {
    println!("result: {}", part2_inner(&content));
}


// TESTS ----------------------------------------

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
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


    fn load_input_file() -> String {
        let mut file = File::open("C:\\Projects\\aoc2023\\data\\day5.txt").unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        s
    }
    #[test]
    fn part2_pest_order() {
        for _ in 0..100 {
            let content = load_input_file();
            let r: Vec<u64> = content
                .replace("\r\n", "\n")
                .split('\n')
                .next().unwrap()
                .replace("seeds: ", "")
                .split(' ')
                .map(|c| c.parse().unwrap())
                .collect();

            let input = parse(&content);
            assert_eq!(r, input.seeds);
        }

    }

    #[test]
    fn part2_no_overlap() {
        let content = load_input_file();
        let input = parse(&content);
        assert_eq!(0, input.seeds.len() % 2);

        for i in (0..input.seeds.len()).step_by(2) {
            let start = input.seeds[i];
            let range = input.seeds[i+1];
            let end = start + range;

            for j in (0..input.seeds.len()).step_by(2) {
                if i == j { continue; }
                let start2 = input.seeds[j];
                let range2 = input.seeds[j+1];
                let end2 = start2 + range2;
                assert!(
                    (start < start2 && end < start2) ||
                    (start > end2 && end > end2)
                );
            }
        }



    }


    #[test]
    fn part2_sample() {
        assert_eq!(46, part2_inner(SAMPLE));
    }
}