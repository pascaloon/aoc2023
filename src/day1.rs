

fn part1_inner(content: String) -> i32 {
    content.split("\n")
        .into_iter()
        .map(|v| match (v.find(|c: char| c.is_ascii_digit()), v.rfind(|c: char| c.is_ascii_digit())) {
            (Some(d), Some(rd)) => (v[d..=d].to_string() + &v[rd..=rd].to_string()).parse(),
            (None, None) => Ok(0),
            _ => panic!("how!?")
        }.unwrap())
        .sum()
}


pub fn part1(content: String) {
    println!("result: {}", part1_inner(content));
}

// -------------------------------------

const NUMBERS: [(&'static str, &'static str); 10] = [
    ("0", "zero"), ("1", "one"), ("2", "two"), ("3", "three"), ("4", "four"),
    ("5", "five"), ("6", "six"), ("7", "seven"), ("8", "eight"), ("9", "nine")
];

fn matches_digit(v: &str) -> Option<&str> {
    if v.chars().next().unwrap().is_ascii_digit() {
        return Some(&v[0..1]);
    }

    for i in 0..10 {
        let (d, num) = NUMBERS[i];
        if v.starts_with(num) {
            return Some(d);
        }
    }

    None
}

pub fn part2_inner(content: String) -> i32 {
    let mut sum = 0;
    for line in content.split("\n") {
        let mut ld = None;
        for i in 0..line.len() {
            if let Some(x) = matches_digit(&line[i..]) {
                ld = Some(x);
                break;
            }
        }

        if ld == None {
            continue;
        }

        let mut rd = None;
        for i in (0..line.len()).rev() {
            if let Some(x) = matches_digit(&line[i..]) {
                rd = Some(x);
                break;
            }
        }

        let ld = ld.unwrap();
        let rd = rd.unwrap();
        let mut s = String::with_capacity(2);
        s.push_str(ld);
        s.push_str(rd);
        let v: i32 = s.parse().unwrap();
        sum += v;

    }

    sum
}

pub fn part2(content: String) {
    println!("result: {}", part2_inner(content));
}

// --------------------------------------

#[cfg(test)]
mod tests {
    use crate::day1::*;

    #[test]
    fn part1_sample() {
        let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
        assert_eq!(142, part1_inner(input.into()));
    }

    #[test]
    fn part2_sample() {
        let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;
        assert_eq!(281, part2_inner(input.into()));
    }
}