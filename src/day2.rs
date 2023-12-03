use std::collections::HashMap;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

// PARSING ---------------------------------------

#[derive(Debug)]
struct Game<'a> {
    id: i32,
    sets: Vec<Set<'a>>
}
#[derive(Debug)]
struct Set<'a> {
    cubes: Vec<Cubes<'a>>
}
#[derive(Debug)]
struct Cubes<'a> {
    color: &'a str,
    count: i32,
}

#[derive(Parser)]
#[grammar = "./day2_grammar.pest"]
pub struct Games;

fn parse_game(game: Pair<Rule>) -> Game {
    let mut id: i32 = 0;
    let mut sets: Vec<Set> = Vec::new();
    for r in game.into_inner() {
        match r.as_rule() {
            Rule::game_id => id = r.as_str().parse().unwrap(),
            Rule::sets => sets = r.into_inner().map(|set| parse_set(set)).collect(),
            _ => panic!("{:?}", r)
        };
    }

    Game { id, sets }
}

fn parse_set(set: Pair<Rule>) -> Set {
    let cubes = set.into_inner()
        .filter(|r| r.as_rule() == Rule::cubes)
        .map(|r| parse_cubes(r))
        .collect();

    Set { cubes }
}

fn parse_cubes(cube: Pair<Rule>) -> Cubes {
    let mut count = 0;
    let mut color = "";

    for r in cube.into_inner() {
        match r.as_rule() {
            Rule::cubes_count => count = r.as_str().parse().unwrap(),
            Rule::cubes_type => color = r.as_str(),
            _ => {}
        }
    }

    Cubes { color, count }
}

fn parse_input(content: &str) -> Vec<Game> {
    let games_file = Games::parse(Rule::games, &content)
        .expect("couldn't parse content!")
        .next().unwrap();

    let games: Vec<Game> = games_file.into_inner()
        .filter(|r| r.as_rule() == Rule::game)
        .map(|r| parse_game(r))
        .collect();

    games
}


// PART 1 --------------------------------------

pub fn part1_inner(content: &str) -> i32 {
    let games = parse_input(&content);
    // let contraints: Vec<Cubes> = vec![
    //     Cubes {color: "red", count: 12},
    //     Cubes {color: "green", count: 13},
    //     Cubes {color: "blue", count: 14},
    // ];

    let mut constraints = HashMap::with_capacity(3);
    constraints.insert("red", 12);
    constraints.insert("green", 13);
    constraints.insert("blue", 14);

    let mut sum = 0;
    for game in &games {
        let mut ok = true;
        for set in &game.sets {
            for cubes in &set.cubes {
                if cubes.count > *constraints.get(cubes.color).unwrap() {
                    ok = false;
                    break;
                }
            }
            if ok == false {
                break;
            }
        }
        if ok {
            sum += game.id;
        }
    }

    sum
}

pub fn part1(content: String) {
    println!("result: {}", part1_inner(&content));
}


// PART 2 --------------------------------------


pub fn part2_inner(content: &str) -> i32 {
    let games = parse_input(&content);
    let mut sum = 0;
    for game in &games {
        let mut maxs: HashMap<&str, i32> = HashMap::with_capacity(3);

        for set in &game.sets {
            for cubes in &set.cubes {
                match maxs.get(cubes.color) {
                    Some(last) if *last < cubes.count => { maxs.insert(cubes.color, cubes.count); },
                    None => { maxs.insert(cubes.color, cubes.count); },
                    _ => {}
                };
            }
        }

        let mut power = 1;
        for (_, count) in maxs.into_iter() {
            power *= count;
        }

        sum += power;

    }

    sum
}

pub fn part2(content: String) {
    println!("result: {}", part2_inner(&content));
}

// TESTS ----------------------------------------

#[cfg(test)]
mod tests {
    use crate::day2::*;

    static SAMPLE: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn part1_sample() {
        assert_eq!(8, part1_inner(SAMPLE));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(2286, part2_inner(SAMPLE));
    }
}