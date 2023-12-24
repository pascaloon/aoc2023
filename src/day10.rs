use std::collections::HashSet;

#[derive(PartialEq, Debug)]
enum Tile {
    Ground,
    Start,
    NorthSouth,
    WestEast,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast
}

enum Rel {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    width: i32,
    height: i32,
    start: usize
}

impl Map {
    // pub fn get_valid_neighbors(&self, t: usize) -> Vec<usize> {
    //     let mut r = Vec::with_capacity(4);
    //     let (tx, ty) = self.get_pos(t).unwrap();
    //     if let Some(p) = self.get_tile(tx-1, ty) {
    //
    //     }
    //
    //     r
    // }

    pub fn get_pos(&self, t: usize) -> Option<(i32, i32)> {
        let (rx, ry) = (t as i32 % self.width, t as i32 / self.width);
        if rx < 0 || rx >= self.width || ry < 0 || ry >= self.height {
            None
        } else {
            Some((rx, ry))
        }
    }

    pub fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            let t = (y * self.width + x) as usize;
            if t >= self.tiles.len() {
                None
            } else {
                Some(t)
            }
        }
    }

    fn get_rel_pos((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> Option<Rel> {
        match (x2 - x1, y2 - y1) {
            (0, -1) => Some(Rel::Top),
            (1, 0) => Some(Rel::Right),
            (0, 1) => Some(Rel::Bottom),
            (-1, 0) => Some(Rel::Left),
            _ => None
        }
    }

    pub fn is_connected(&self, (x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> bool {
        let Some(rel) = Map::get_rel_pos((x1, y1), (x2, y2)) else {
            return false;
        };

        let t1 = self.get_tile(self.get_index(x1, y1).unwrap());
        let t2 = self.get_tile(self.get_index(x2, y2).unwrap());

        match t1 {
            Tile::Ground => {}
            Tile::Start => {}
            Tile::NorthSouth => {}
            Tile::WestEast => {}
            Tile::NorthEast => {}
            Tile::NorthWest => {}
            Tile::SouthWest => {}
            Tile::SouthEast => {}
        };

        match (rel, t1,t2) {
            (_, Tile::Ground, _) | (_, _, Tile::Ground) => false,

            (Rel::Top, Tile::NorthSouth, Tile::NorthSouth) => true,
            (Rel::Top, Tile::NorthSouth, Tile::SouthWest) => true,
            (Rel::Top, Tile::NorthSouth, Tile::SouthEast) => true,
            (Rel::Top, Tile::NorthSouth, Tile::Start) => true,

            (Rel::Top, Tile::NorthEast, Tile::NorthSouth) => true,
            (Rel::Top, Tile::NorthEast, Tile::SouthWest) => true,
            (Rel::Top, Tile::NorthEast, Tile::SouthEast) => true,
            (Rel::Top, Tile::NorthEast, Tile::Start) => true,

            (Rel::Top, Tile::NorthWest, Tile::NorthSouth) => true,
            (Rel::Top, Tile::NorthWest, Tile::SouthWest) => true,
            (Rel::Top, Tile::NorthWest, Tile::SouthEast) => true,
            (Rel::Top, Tile::NorthWest, Tile::Start) => true,

            (Rel::Top, Tile::Start, Tile::NorthSouth) => true,
            (Rel::Top, Tile::Start, Tile::SouthWest) => true,
            (Rel::Top, Tile::Start, Tile::SouthEast) => true,

            (Rel::Right, Tile::WestEast, Tile::WestEast) => true,
            (Rel::Right, Tile::WestEast, Tile::NorthWest) => true,
            (Rel::Right, Tile::WestEast, Tile::SouthWest) => true,
            (Rel::Right, Tile::WestEast, Tile::Start) => true,

            (Rel::Right, Tile::NorthEast, Tile::WestEast) => true,
            (Rel::Right, Tile::NorthEast, Tile::NorthWest) => true,
            (Rel::Right, Tile::NorthEast, Tile::SouthWest) => true,
            (Rel::Right, Tile::NorthEast, Tile::Start) => true,

            (Rel::Right, Tile::SouthEast, Tile::WestEast) => true,
            (Rel::Right, Tile::SouthEast, Tile::NorthWest) => true,
            (Rel::Right, Tile::SouthEast, Tile::SouthWest) => true,
            (Rel::Right, Tile::SouthEast, Tile::Start) => true,

            (Rel::Right, Tile::Start, Tile::WestEast) => true,
            (Rel::Right, Tile::Start, Tile::NorthWest) => true,
            (Rel::Right, Tile::Start, Tile::SouthWest) => true,

            (Rel::Bottom, Tile::NorthSouth, Tile::NorthSouth) => true,
            (Rel::Bottom, Tile::NorthSouth, Tile::NorthEast) => true,
            (Rel::Bottom, Tile::NorthSouth, Tile::NorthWest) => true,
            (Rel::Bottom, Tile::NorthSouth, Tile::Start) => true,

            (Rel::Bottom, Tile::SouthWest, Tile::NorthSouth) => true,
            (Rel::Bottom, Tile::SouthWest, Tile::NorthEast) => true,
            (Rel::Bottom, Tile::SouthWest, Tile::NorthWest) => true,
            (Rel::Bottom, Tile::SouthWest, Tile::Start) => true,

            (Rel::Bottom, Tile::SouthEast, Tile::NorthSouth) => true,
            (Rel::Bottom, Tile::SouthEast, Tile::NorthEast) => true,
            (Rel::Bottom, Tile::SouthEast, Tile::NorthWest) => true,
            (Rel::Bottom, Tile::SouthEast, Tile::Start) => true,

            (Rel::Bottom, Tile::Start, Tile::NorthSouth) => true,
            (Rel::Bottom, Tile::Start, Tile::NorthEast) => true,
            (Rel::Bottom, Tile::Start, Tile::NorthWest) => true,

            (Rel::Left, Tile::WestEast, Tile::WestEast) => true,
            (Rel::Left, Tile::WestEast, Tile::NorthEast) => true,
            (Rel::Left, Tile::WestEast, Tile::SouthEast) => true,
            (Rel::Left, Tile::WestEast, Tile::Start) => true,

            (Rel::Left, Tile::NorthWest, Tile::WestEast) => true,
            (Rel::Left, Tile::NorthWest, Tile::NorthEast) => true,
            (Rel::Left, Tile::NorthWest, Tile::SouthEast) => true,
            (Rel::Left, Tile::NorthWest, Tile::Start) => true,

            (Rel::Left, Tile::SouthWest, Tile::WestEast) => true,
            (Rel::Left, Tile::SouthWest, Tile::NorthEast) => true,
            (Rel::Left, Tile::SouthWest, Tile::SouthEast) => true,
            (Rel::Left, Tile::SouthWest, Tile::Start) => true,

            (Rel::Left, Tile::Start, Tile::WestEast) => true,
            (Rel::Left, Tile::Start, Tile::NorthEast) => true,
            (Rel::Left, Tile::Start, Tile::SouthEast) => true,

            _ => false,
        }
    }

    pub fn get_tile(&self, t: usize) -> &Tile {
        self.tiles.get(t).unwrap()
    }
}

fn parse(content: &str) -> Map {
    let lines_separator = if content.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };

    let lines = content
        .trim()
        .split(lines_separator);

    let mut tiles: Vec<Tile> = Vec::new();
    let mut height = 0i32;
    for line in lines {
        height += 1;
        for c in line.chars() {
            let tile = match c {
               '|' => Tile::NorthSouth,
               '-' => Tile::WestEast,
               'L' => Tile::NorthEast,
               'J' => Tile::NorthWest,
               '7' => Tile::SouthWest,
               'F' => Tile::SouthEast,
               '.' => Tile::Ground,
               'S' => Tile::Start,
               _ => panic!("Couldn't parse Tile '{}'", c)
            };
            tiles.push(tile);
        }
    }
    let width = tiles.len() as i32 / height;
    let start = tiles.iter().position(|t| *t == Tile::Start).expect("Map has no start!");
    Map { tiles, width, height, start }
}


fn is_same_as_last(last: &Option<usize>, val: usize) -> bool {
    match last {
        None => false,
        Some(x) => *x == val
    }
}

fn build_loop(map: &Map) -> Vec<usize> {
    let mut last = None;
    let mut current = map.start;
    let mut nodes = Vec::new();
    loop {
        if nodes.len() >= 1 && current == map.start {
            break;
        }
        nodes.push(current);

        let (x, y) = map.get_pos(current).unwrap();
        // println!("({}, {})", x, y);
        // assert!(count < 100);

        if let Some(top) = map.get_index(x, y - 1) {
            if map.is_connected((x, y), (x, y - 1)) && !is_same_as_last(&last, top) {
                last = Some(current);
                current = top;
                continue;
            }
        }
        if let Some(right) = map.get_index(x + 1, y) {
            if map.is_connected((x, y), (x + 1, y)) && !is_same_as_last(&last, right) {
                last = Some(current);
                current = right;
                continue;
            }
        }
        if let Some(bottom) = map.get_index(x, y + 1) {
            if map.is_connected((x, y), (x, y + 1)) && !is_same_as_last(&last, bottom) {
                last = Some(current);
                current = bottom;
                continue;
            }
        }
        if let Some(left) = map.get_index(x - 1, y){
            if map.is_connected((x, y), (x - 1, y)) && !is_same_as_last(&last, left)  {
                last = Some(current);
                current = left;
                continue;
            }
        }


        panic!("main dead loop");
    }

    nodes
}

// PART 1 --------------------------------------

fn part1_inner(content: &str) -> i64 {
    let map = parse(content);
    let nodes = build_loop(&map);
    nodes.len() as i64 / 2
}

pub fn part1(content: String) {
    println!("result: {}", part1_inner(&content));
}

// PART 2 --------------------------------------

pub fn part2_inner(content: &str) -> i64 {
    let map = parse(content);
    let loop_nodes: HashSet<usize> = HashSet::from_iter(build_loop(&map).into_iter());

    // row checks
    let mut horizontal_checks: HashSet<usize> = HashSet::new();
    for y in 0..map.height {
        let mut in_loop = false;
        for x in 0..map.width {
            let tile = map.get_index(x, y).unwrap();
            match loop_nodes.get(&tile) {
                Some(_) => {
                    in_loop = !in_loop;
                }
                None if in_loop  => {
                    horizontal_checks.insert(tile);
                }
                _ => {}
            };

        }
    }

    // column checks
    let mut in_nodes: Vec<usize> = Vec::new();
    let mut out_nodes: Vec<usize> = Vec::new();
    for x in 0..map.width {
        let mut in_loop = false;
        for y in 0..map.height {
            let tile = map.get_index(x, y).unwrap();
            match loop_nodes.get(&tile) {
                Some(_) => {
                    in_loop = !in_loop;
                }
                None if in_loop && horizontal_checks.contains(&tile)  => {
                    in_nodes.push(tile);
                }
                _ => {
                    out_nodes.push(tile);
                }
            };
        }
    }

    print_map(&map, &in_nodes, &out_nodes);


    in_nodes.len() as i64
}

fn print_map(map: &Map, in_nodes: &Vec<usize>, out_nodes: &Vec<usize>) {
    for y in 0..map.height {
        for x in 0..map.width {
            let index = map.get_index(x, y).unwrap();
            let tile = map.get_tile(index);
            let s = match tile {
                _ if in_nodes.contains(&index) => "I",
                _ if out_nodes.contains(&index) => "O",
                Tile::Ground => ".",
                Tile::Start => "S",
                Tile::NorthSouth => "|",
                Tile::WestEast => "-",
                Tile::NorthEast => "L",
                Tile::NorthWest => "J",
                Tile::SouthWest => "7",
                Tile::SouthEast => "F",
            };
            print!("{}", s);
        }
        println!();
    }
}

pub fn part2(content: String) {
    println!("result: {}", part2_inner(&content));
}

// TESTS --------------------------------------


#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_1: &'static str = r#"
.....
.S-7.
.|.|.
.L-J.
.....
"#;

    static SAMPLE_2: &'static str = r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#;

    static SAMPLE_3: &'static str = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

    #[test]
    fn part1_sample1() {
        assert_eq!(4, part1_inner(SAMPLE_1));
    }

    #[test]
    fn part1_sample2() {
        assert_eq!(4, part1_inner(SAMPLE_2));
    }

    #[test]
    fn part1_sample3() {
        assert_eq!(8, part1_inner(SAMPLE_3));
    }

    static SAMPLE_4: &'static str = r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;

    static SAMPLE_5: &'static str = r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;

    static SAMPLE_6: &'static str = r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;
    #[test]
    fn part2_sample4() {
        assert_eq!(4, part2_inner(SAMPLE_4));
    }

    #[test]
    fn part2_sample5() {
        assert_eq!(8, part2_inner(SAMPLE_5));
    }

    #[test]
    fn part2_sample6() {
        assert_eq!(10, part2_inner(SAMPLE_6));
    }

}
