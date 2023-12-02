

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


pub fn part2(content: String) {

}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part1_inner};

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
}