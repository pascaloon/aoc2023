use std::mem;

// PARSING ---------------------------------------
struct Map<'a> {
    chars: &'a [u8],
    pub width: i32,
    pub height: i32,
}

impl<'a> Map<'a> {
    pub fn new(chars: &'a [u8]) -> Map<'a> {
        let width = chars.iter().position(|c| char::from(*c) == '\n').unwrap() as i32;
        let height = (chars.len() as i32 / (width+1)) as i32; // <- input needs to end with a newline
        Map { chars, width, height }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            None
        } else {
            self.chars
                .get(((y * (self.width+1)) + x) as usize)
                .map(|c| char::from(*c))
        }
    }
}

// PART 1 --------------------------------------
struct CurrentWord {
    pub word: Vec<char>,
    pub anchored: bool
}

fn is_anchored(map: &Map, cx: i32, cy: i32) -> bool {
    for y in (cy-1)..=(cy+1) {
        for x in (cx-1)..=(cx+1) {
            match map.get(x, y) {
                None | Some('.') => { },
                Some(d) if d.is_ascii_digit() => { },
                Some('\n') | Some('\r') => panic!("eeeee"),
                _ => return true
            }
        }
    }

    false
}

pub fn part1_inner(content: &str) -> i32 {
    let map = Map::new(content.as_bytes());

    let mut nums = Vec::new();
    for y in 0..map.height {
        let mut current_num: Option<CurrentWord> = None;
        for x in 0..map.width {
            match map.get(x, y) {
                Some(c) if c.is_ascii_digit() => {
                    match &mut current_num {
                        Some(w) => {
                            w.word.push(c);
                            w.anchored = w.anchored || is_anchored(&map, x, y);
                        },
                        None => {
                            let word = vec![c];
                            let anchored = is_anchored(&map, x, y);
                            current_num = Some(CurrentWord { word, anchored })
                        }
                    }
                },
                _ => {
                    let mut cn: Option<CurrentWord> = None;
                    mem::swap(&mut cn, &mut current_num);
                    match cn {
                        Some(n) if n.anchored => nums.push(n),
                        _ => { }
                    }
                }
            }
        }

        match current_num {
            Some(n) if n.anchored => nums.push(n),
            _ => { }
        }
    }

    nums.iter()
        .map(|n| n.word.iter().collect::<String>().parse::<i32>().unwrap())
        .sum()
}

pub fn part1(content: String) {
    println!("result: {}", part1_inner(&content));
}

// PART 2 --------------------------------------

pub fn part2_inner(content: &str) -> i32 {
    0
}

pub fn part2(content: String) {
    println!("result: {}", part1_inner(&content));
}


// TESTS ----------------------------------------

#[cfg(test)]
mod tests {
    use crate::day3::*;

    static SAMPLE: &'static str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn part1_sample() {
        assert_eq!(4361, part1_inner(SAMPLE));
    }

    // #[test]
    // fn part2_sample() {
    //     assert_eq!(2286, part2_inner(SAMPLE));
    // }
}