// PARSE --------------------------------------
fn parse(content: &str) -> Vec<Vec<i64>> {
    let content = content.replace("\r\n", "\n");
    content.split("\n")
        .filter(|l|l.len() > 0)
        .map(|l| parse_line(l))
        .collect()
}

fn parse_line(l: &str) -> Vec<i64> {
    l.split(" ").map(|w|w.parse().unwrap()).collect()
}

// PART 1 --------------------------------------

fn find_next(history: &Vec<i64>) -> i64 {
    let mut derivatives = Vec::new();
    derivatives.push(history.clone());

    while derivatives.last().unwrap().iter().any(|n| *n != 0i64){
        let last = derivatives.last().unwrap();
        let mut deltas = Vec::with_capacity(last.len()-1);
        for i in 1..last.len() {
            let delta = last[i] - last[i -1];
            deltas.push(delta);
        }
        derivatives.push(deltas);
    }

    derivatives.last_mut().unwrap().push(0i64);
    for i in (0..(derivatives.len()-1)).rev() {
        let delta = *derivatives[i + 1].last().unwrap();
        let last_num = *derivatives[i].last().unwrap();
        derivatives[i].push(last_num + delta);
    }

    *derivatives.first().unwrap().last().unwrap()
}

fn part1_inner(content: &str) -> i64 {
    let histories = parse(content);
    let tot: usize = histories.iter().map(|v|v.len()).sum();
    println!("numbers count: {tot}");
    histories.iter().map(find_next).sum()
}

pub fn part1(content: String) {
    println!("result: {}", part1_inner(&content));
}

// PART 2 --------------------------------------

pub fn part2_inner(content: &str) -> i64 {
    0
}

pub fn part2(content: String) {
    println!("result: {}", part2_inner(&content));
}


#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_1: &'static str = "0 3 6 9 12 15";
    static SAMPLE_2: &'static str = "1 3 6 10 15 21";
    static SAMPLE_3: &'static str = "10 13 16 21 30 45";
    static SAMPLE_ALL: &'static str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn part1_sample1() {
        assert_eq!(18, part1_inner(SAMPLE_1));
    }

    #[test]
    fn part1_sample2() {
        assert_eq!(28, part1_inner(SAMPLE_2));
    }

    #[test]
    fn part1_sample3() {
        assert_eq!(68, part1_inner(SAMPLE_3));
    }

    #[test]
    fn part1_sample_all() {
        assert_eq!(114, part1_inner(SAMPLE_ALL));
    }
}
