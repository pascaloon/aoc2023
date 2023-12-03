use std::mem;

// SHARED ---------------------------------------
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

struct CurrentWord {
    pub word: Vec<char>,
    pub mapped_anchors: Vec<(char, i32, i32)>
}

fn get_anchors(map: &Map, cx: i32, cy: i32) -> Vec<(char, i32, i32)> {
    let mut anchors = Vec::new();
    for y in (cy-1)..=(cy+1) {
        for x in (cx-1)..=(cx+1) {
            match map.get(x, y) {
                None | Some('.') => { },
                Some(d) if d.is_ascii_digit() => { },
                Some('\n') | Some('\r') => panic!("eeeee"),
                Some(c) => anchors.push((c, x, y))
            }
        }
    }

    anchors
}

fn is_anchored(map: &Map, cx: i32, cy: i32) -> bool {
    get_anchors(map, cx, cy).len() > 0
}

pub fn map_numbers(map: &Map) -> Vec<CurrentWord> {
    let mut nums = Vec::new();
    for y in 0..map.height {
        let mut current_num: Option<CurrentWord> = None;
        for x in 0..map.width {
            match map.get(x, y) {
                Some(c) if c.is_ascii_digit() => {
                    match &mut current_num {
                        Some(w) => {
                            w.word.push(c);
                            let anchors = get_anchors(map, x, y);
                            for anchor in anchors {
                                if !w.mapped_anchors.contains(&anchor) {
                                    w.mapped_anchors.push(anchor);
                                }
                            }
                        },
                        None => {
                            let word = vec![c];
                            let mapped_anchors = get_anchors(map, x, y);
                            current_num = Some(CurrentWord { word, mapped_anchors})
                        }
                    }
                },
                _ => {
                    let mut cn: Option<CurrentWord> = None;
                    mem::swap(&mut cn, &mut current_num);
                    match cn {
                        Some(n) => nums.push(n),
                        _ => { }
                    }
                }
            }
        }

        match current_num {
            Some(n) => nums.push(n),
            _ => { }
        }
    }

    nums
}

// PART 1 --------------------------------------

pub fn part1_inner(content: &str) -> i32 {
    let map = Map::new(content.as_bytes());

    let nums = map_numbers(&map);

    nums.iter()
        .filter(|n| n.mapped_anchors.len() > 0)
        .map(|n| n.word.iter().collect::<String>().parse::<i32>().unwrap())
        .sum()
}

pub fn part1(content: String) {
    println!("result: {}", part1_inner(&content));
}

// PART 2 --------------------------------------

fn get_gear_parts(nums: &Vec<CurrentWord>, x: i32, y:i32) -> Vec<&CurrentWord> {
    let mut found= Vec::new();

    for num in nums {
        for a in &num.mapped_anchors {
            match a {
                ('*', xx, yy) if *xx == x && *yy == y => found.push(num),
                _ => { }
            }
        }
    }

    found
}

pub fn part2_inner(content: &str) -> i32 {
    let map = Map::new(content.as_bytes());

    let nums = map_numbers(&map);

    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            match map.get(x, y) {
                Some('*') => {
                    let parts = get_gear_parts(&nums, x, y);
                    println!("len: {:?}", parts.iter().map(|p|p.word.iter().collect::<String>().parse::<i32>().unwrap()).collect::<Vec<i32>>());
                    if parts.len() == 2 {
                        let n1 = parts[0].word.iter().collect::<String>().parse::<i32>().unwrap();
                        let n2 = parts[1].word.iter().collect::<String>().parse::<i32>().unwrap();
                        sum += n1 * n2;
                    }
                },
                _ => {}
            }
        }
    }

    sum


}

pub fn part2(content: String) {
    println!("result: {}", part2_inner(&content));
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

    #[test]
    fn part2_sample() {
        assert_eq!(467835, part2_inner(SAMPLE));
    }
}