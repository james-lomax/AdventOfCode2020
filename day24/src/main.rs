use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2i(i32, i32);

impl Vec2i {
    fn add(&self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

fn next_direction(chars: &mut dyn Iterator<Item = char>) -> Option<Vec2i> {
    match chars.next() {
        Some('n') => {
            match chars.next().expect("Need moore characters!") {
                'e' => Some(Vec2i(-1, -1)),
                'w' => Some(Vec2i(0, -1)),
                _ => panic!("Bad character!")
            }
        }
        Some('s') => {
            match chars.next().expect("Need moore characters!") {
                'e' => Some(Vec2i(0, 1)),
                'w' => Some(Vec2i(1, 1)),
                _ => panic!("Bad character!")
            }
        }
        Some('w') => Some(Vec2i(1, 0)),
        Some('e') => Some(Vec2i(-1, 0)),
        None => None,
        _ => panic!("Bad character!")
    }
}

fn tracepos(line: &str) -> Vec2i {
    let mut it = line.chars();
    let mut turtle = Vec2i(0, 0);
    while let Some(d) = next_direction(&mut it) {
        turtle = turtle.add(d);
    }
    turtle
}

fn flip(contents: &str) -> HashMap<Vec2i, bool> {
    let mut out: HashMap<Vec2i, bool> = HashMap::new();
    for line in contents.split("\n").map(|s| s.trim()).filter(|s| s.len() > 0) {
        let p = tracepos(line);
        if let Some(t) = out.get_mut(&p) {
            *t = !*t;
        } else {
            out.insert(p, true);
        }
    }
    out
}

fn count_black(grid: &HashMap<Vec2i, bool>) -> usize {
    grid.values().filter(|v| **v).count()
}

const all_dirs: [Vec2i; 6] = [
    Vec2i(0, -1), Vec2i(-1, -1),
    Vec2i(1, 0), Vec2i(-1, 0),
    Vec2i(1, 1), Vec2i(0, 1)
];

fn step(grid: HashMap<Vec2i, bool>) -> HashMap<Vec2i, bool> {
    let mut counts = HashMap::<Vec2i, usize>::new();

    // Loop over all things and count... all directions
    for (p, b) in grid.iter() {
        if *b {
            for d in &all_dirs {
                let x = p.add(*d);
                if let Some(c) = counts.get_mut(&x) {
                    *c += 1;
                } else {
                    counts.insert(x, 1);
                }
            }
        }
    }
    
    // Updated tiles
    counts.drain().map(|(p, c)| {
        (p, match grid.get(&p) {
            Some(true) => !(c == 0 || c > 2),
            _ => c == 2
        })
    }).filter(|(p, b)| *b).collect()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");

    let mut grid = flip(&contents);
    println!("Part 1 = {}", count_black(&grid));

    for _ in 0..100 {
        grid = step(grid);
    }
    println!("Part 2 = {}", count_black(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(next_direction(&mut "nesw".chars()), Some(Vec2i(-1, -1)));

        let sample = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        let grid = flip(sample);
        assert_eq!(count_black(&grid), 10);
        let grid = step(grid);
        assert_eq!(count_black(&grid), 15);
        let mut grid = step(grid);
        assert_eq!(count_black(&grid), 12);

        for _ in 3..=100 {
            grid = step(grid);
        }
        assert_eq!(count_black(&grid), 2208);
    }
}
