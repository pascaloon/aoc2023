// PARSING -------------------------------------

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64
}

fn parse(content: &str) -> Vec<Race> {
    let clean_line = content.trim().replace("\r\n", "\n");
    let lines: Vec<&str> = clean_line.split('\n').collect();
    assert_eq!(2, lines.len());
    fn parse_line(line: &str) -> Vec<u64> {
        line.split(" ")
            .skip(1)
            .filter(|s| s.len() > 0)
            .map(|int| int.parse().unwrap())
            .collect()
    }

    let input: Vec<Vec<u64>> = lines.iter().map(|s| parse_line(s)).collect();
    let mut races = Vec::new();
    for i in 0..input[0].len() {
        races.push(Race { time: input[0][i], record_distance: input[1][i] });
    }

    races

}

// PART 1 --------------------------------------


fn count_race_winning_states(race: &Race) -> u64 {
    let mut wins_count = 0;
    for pressing_time in 0..=race.time {
        let velocity = pressing_time;
        let time_to_run = race.time - pressing_time;
        let total_distance = time_to_run * velocity;
        if total_distance > race.record_distance {
            wins_count += 1;
        }
    }

    wins_count
}

fn part1_inner(content: &str) -> u64 {
    let races = parse(content);
    println!("{:?}", races);
    races.iter()
        .fold(1, |acc, race| acc * count_race_winning_states(race))
}


pub fn part1(content: String) {
    println!("{}", part1_inner(&content));
}

// PART 2 --------------------------------------

fn part2_inner(content: &str) -> u64 {
    0
}

pub fn part2(content: String) {
    println!("Final Result: {}", part2_inner(&content));
}


#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &'static str = r#"
Time:      7  15   30
Distance:  9  40  200
"#;

    #[test]
    fn part1_sample() {
        assert_eq!(4 * 8 * 9, part1_inner(SAMPLE));
    }
}
