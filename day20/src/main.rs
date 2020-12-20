use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    id: usize,
    grid: Vec<Vec<bool>>,
    edges: [Vec<bool>; 4],
    width: usize
}

fn reversed<T : Clone>(v: &Vec<T>) -> Vec<T> {
    v.iter().rev().cloned().collect()
}

fn flip_grid_hori(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    grid.iter().map(|v| reversed(v)).collect()
}

fn rotate_grid(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let width = grid[0].len();
    let mut nv = Vec::with_capacity(width);
    for i in 0..width {
        nv.push(grid.iter().rev().map(|v| v[i]).collect());
    }
    nv
}

impl Tile {
    fn parse(tile_str: &str) -> Self {
        // TODO: lazy static    
        let re = Regex::new(r"Tile (\d+):").unwrap();
        let t: Vec<&str> = tile_str.split("\n")
            .map(|s| s.trim()).filter(|s| s.len() > 0)
            .collect();
        let id = re.captures(t[0])
            .expect("First line of tile should describe ID")
            .get(1).expect("Expected capture group 1")
            .as_str()
            .parse::<usize>().expect("Regex number match has failed us");

        let g: Vec<Vec<bool>> = t.iter()
            .skip(1)
            .map(|s| s.chars().map(|c| c == '#').collect())
            .collect();
        let width = g.len();
        for r in &g {
            assert_eq!(r.len(), width);
        }

        let mut slf = Self {
            id: id,
            grid: g,
            edges: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            width: width
        };  

        // Egdes are clockwise (NESW - 0123)
        slf.edges[0] = slf.grid[0].clone();
        slf.edges[1] = slf.grid.iter().map(|v| v[v.len()-1]).collect();
        slf.edges[2] = slf.grid[slf.grid.len() - 1].iter().rev().cloned().collect();
        slf.edges[3] = slf.grid.iter().rev().map(|v| v[0]).collect();

        slf
    }

    fn edge(&self, orientation: usize, edge: usize) -> &Vec<bool> {
        &self.edges[(edge + orientation) % 4]
    }

    fn edge_fit_symmetric(&self, orientation: usize, edge: usize, other: &Vec<bool>) -> bool {
        self.edge(orientation, edge).iter().cmp(
            other.iter().rev()
        ) == Ordering::Equal ||
        self.edge(orientation, edge).iter().cmp(
            other.iter()
        ) == Ordering::Equal
    }

    fn edge_fit_asym(&self, orientation: usize, edge: usize, other: &Vec<bool>) -> bool {
        // Always compare reversed edges (each edge is a slice going 0->N clockwise)
        self.edge(orientation, edge).iter().cmp(
            other.iter().rev()
        ) == Ordering::Equal 
    }

    fn flip_horizontally(&self) -> Self {
        Self {
            id: self.id,
            grid: flip_grid_hori(&self.grid),
            edges: [
                reversed(&self.edges[0]),
                reversed(&self.edges[3]),
                reversed(&self.edges[2]),
                reversed(&self.edges[1])
            ],
            width: self.width
        }
    }

    fn rotate(&self, orient: usize) -> Self {
        if orient == 0 {
            self.clone()
        } else {
            Self {
                id: self.id,
                grid: rotate_grid(&self.grid),
                edges: [
                    self.edges[3].clone(),
                    self.edges[0].clone(),
                    self.edges[1].clone(),
                    self.edges[2].clone()
                ],
                width: self.width
            }.rotate(orient-1)
        }
    }
}

fn parse_tiles(contents: &str) -> HashMap<usize, Tile> {
    let contents = contents.replace("\r", "");
    contents.split("\n\n")
        .filter(|s| s.len() > 0)
        .map(|tile| {
            let t = Tile::parse(tile);
            (t.id, t)
        })
        .collect()
}

fn edge_match_count(tiles: &HashMap<usize, Tile>, tile: &Tile, dir: usize) -> usize {
    let side = &tile.edges[dir];
    let mut same_count = 0;

    for (ib, tb) in tiles.iter() {
        if *ib != tile.id {
            for side_b_d in 0..4 {
                if tb.edge_fit_symmetric(0, side_b_d, side) {
                    same_count += 1;
                }
            }
        }
    }

    return same_count;
}

fn edge_sides(tiles: &HashMap<usize, Tile>, tile: &Tile) -> Vec<usize> {
    (0..4).filter(|side_d| edge_match_count(tiles, tile, *side_d) == 0).collect()
}

// Find the pieces with only 2 matching edges
fn find_pieces_with_n_edges(tiles: &HashMap<usize, Tile>, n_edges: usize) -> Vec<usize> {
    let mut corners = Vec::new();

    for (ia, ta) in tiles.iter() {
        if edge_sides(tiles, ta).len() == n_edges {
            corners.push(*ia);
        }
    }

    return corners;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2i(i32, i32);

impl Vec2i {
    fn fromu(x: usize, y: usize) -> Self {
        Self(x as i32, y as i32)
    }

    fn add(&self, other: Vec2i) -> Self {
        Vec2i(self.0+other.0, self.1+other.1)
    }
}

fn direction(d: usize) -> Vec2i {
    match d {
        0 => Vec2i(0, -1),
        1 => Vec2i(1, 0),
        2 => Vec2i(0, 1),
        3 => Vec2i(-1, 0),
        _ => panic!("Invalid direction...")
    }
}

struct Solver {
    tiles: HashMap<usize, Tile>,
    positions: HashMap<Vec2i, Tile>,
    width: usize
}

fn assert_sqrt(n: usize) -> usize {
    for i in 1..n {
        if i*i == n {
            return i;
        }
    }
    panic!("{} does not have an integer square root", n);
}

impl Solver {
    fn new(tiles: HashMap<usize, Tile>) -> Self {
        Self {
            width: assert_sqrt(tiles.len()),
            tiles: tiles,
            positions: HashMap::new()
        }
    }

    fn is_in_bounds(&self, pos: Vec2i) -> bool {
        pos.0 >= 0 && pos.0 < self.width as i32 && pos.1 >= 0 && pos.1 < self.width as i32
    }

    fn check_tile(&mut self, pos: Vec2i, tile: &Tile) -> bool {
        // See if it fits in by checking:
        //  All existing surroundings match the edge perfectly
        //  Any out-of-bounds edges are: `edge_sides == 0`
        // For each direction (check in bounds, check matches edges if there are things there)
        for dir in 0..4 {
            let npos = pos.add(direction(dir));
            if self.is_in_bounds(npos) {
                // Check whats here and that the edge is ok if there is something there
                if let Some(other) = self.positions.get(&npos) {
                    if !tile.edge_fit_asym(0, dir, other.edge(0, (dir+2) % 4)) {
                        return false;
                    }
                }
            } else {
                if edge_match_count(&self.tiles, tile, dir) != 0 {
                    return false;
                }
            }
        }
        return true;
    }

    // Find an appropriate option from `available` list to fill at `pos`
    // Returns the chosen tile and panics if none is found
    fn fill_next(&mut self, pos: Vec2i, available: &Vec<usize>) -> usize {
        // for each, try flipping horizontally and doing each rotation
        for tile_id in available {
            let mut tile = self.tiles.get(tile_id).expect("Invalid tile ID").clone();
            for flipped in 0..2 {
                for i in 0..4 {
                    if self.check_tile(pos, &tile) {
                        self.positions.insert(pos, tile.clone());
                        return *tile_id;
                    }

                    tile = tile.rotate(1);
                }
                
                tile = tile.flip_horizontally();
            }
        }

        println!("Failed trying to fill {:?} from {:?}", pos, available);
        self.print_positions();
        panic!("Couldn't find anything to fill!");
    }

    fn fill_take(&mut self, pos: Vec2i, available: &mut Vec<usize>) {
        let id = self.fill_next(pos, available);
        available.remove(available.iter().position(|x| *x == id).unwrap());
    }

    fn solve(&mut self) {
        let mut corners = find_pieces_with_n_edges(&self.tiles, 2);
        let mut edges = find_pieces_with_n_edges(&self.tiles, 1);
        let mut centres = find_pieces_with_n_edges(&self.tiles, 0);

        // Take a corner
        self.fill_take(Vec2i(0, 0), &mut corners);

        // Fill the top edge pieces
        for x in 1..(self.width-1) {
            self.fill_take(Vec2i::fromu(x, 0), &mut edges);
        }

        // Fill top right corner
        self.fill_take(Vec2i::fromu(self.width-1, 0), &mut corners);

        // Fill the right edge
        for y in 1..(self.width-1) {
            self.fill_take(Vec2i::fromu(self.width-1, y), &mut edges);
        }

        // Fill bottom right corner
        self.fill_take(Vec2i::fromu(self.width-1, self.width-1), &mut corners);

        // Fill in left edge
        for y in 1..(self.width-1) {
            self.fill_take(Vec2i::fromu(0, y), &mut edges);
        }

        // Fill bottom left corner
        self.fill_take(Vec2i::fromu(0, self.width-1), &mut corners);

        // Fill the top edge pieces
        for x in 1..(self.width-1) {
            self.fill_take(Vec2i::fromu(x, self.width-1), &mut edges);
        }

        // Fill the middle pieces
        for y in 1..(self.width-1) {
            for x in 1..(self.width-1) {
                self.fill_take(Vec2i::fromu(x, y), &mut centres);
            }
        }
    }

    // Min, Max
    fn bounds(&self) -> (Vec2i, Vec2i) {
        let mut minx = 0;
        let mut maxx = 0;
        let mut miny = 0;
        let mut maxy = 0;

        for p in self.positions.keys() {
            minx = std::cmp::min(minx, p.0);
            maxx = std::cmp::max(maxx, p.0);
            miny = std::cmp::min(miny, p.1);
            maxy = std::cmp::max(maxy, p.1);
        }

        return (Vec2i(minx, miny), Vec2i(maxx, maxy));
    }

    fn make_grid(&self) -> Vec<Vec<bool>> {
        let mut out = Vec::new();

        let (min, max) = self.bounds();
        assert_eq!(min, Vec2i(0, 0));
        let width = self.positions[&Vec2i(0, 0)].width;

        for y in min.1 ..=(max.1) {
            for row in 1..(width-1) {
                let mut rowv = Vec::new();

                for x in min.0 ..=(max.0) {
                    for col in 1..(width-1) {
                        let tile = self.positions.get(&Vec2i(x, y)).expect("Ah shit");
                        rowv.push(tile.grid[row][col]);
                    }
                }
                out.push(rowv);
            }
        }

        return out;
    }

    fn print_positions(&self) {
        let (min, max) = self.bounds();

        for y in min.1 ..=(max.1) {
            for x in min.0 ..=(max.0) {
                if let Some(tile) = self.positions.get(&Vec2i(x, y)) {
                    print!("{:04} ", tile.id);
                } else {
                    print!("---- ");
                }
            }
            println!("");
        }
    }
}

fn debug_print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for x in row {
            print!("{}", if *x { '#' } else { '.' });
        }
        println!("");
    }
}

fn make_sea_monster() -> Vec<Vec<bool>> {
    let s = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
    
    s.split("\n")
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect()
}

fn grid_or(grid: Vec<Vec<bool>>, other: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    grid.iter().zip(other.iter())
        .map(|(a, b)| a.iter().zip(b.iter()).map(|(a, b)| *a || *b).collect())
        .collect()
}

fn grid_and(grid: Vec<Vec<bool>>, other: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    grid.iter().zip(other.iter())
        .map(|(a, b)| a.iter().zip(b.iter()).map(|(a, b)| *a && *b).collect())
        .collect()
}

fn grid_not(grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    grid.iter().map(|v| v.iter().map(|x| !*x).collect()).collect()
}

fn grid_im_check(grid: Vec<Vec<bool>>, image: &Vec<Vec<bool>>, off_x: usize, off_y: usize) -> Vec<Vec<bool>> {
    let empty: Vec<Vec<bool>> = grid.iter()
        .map(|v| v.iter().map(|_| false).collect()).collect();
    let mut ngrid = empty.clone();
    let mut is_good = true;
    assert!(image.len() + off_y <= grid.len());
    for y in 0..image.len() {
        assert!(image[y].len() + off_x <= grid[y].len());
        for x in 0..image[y].len() {
            if image[y][x] {
                if grid[y+off_y][x+off_x] {
                    ngrid[y+off_y][x+off_x] = true;
                } else {
                    is_good = false;
                }
            }
        }
    }
    
    if is_good {
        ngrid
    } else {
        empty
    }
}

fn count_non_seamonster(grid: Vec<Vec<bool>>) -> usize {
    let mut seamon = make_sea_monster();
    let gwidth = grid.len();

    // Make a grid of the seamonster cells
    let mut sms: Vec<Vec<bool>> = grid.iter()
        .map(|v| v.iter().map(|_| false).collect()).collect();

    for flipped in 0..2 {
        for i in 0..4 {
            // Enumerate all offsets
            let h = seamon.len();
            let w = seamon[0].len();

            for off_y in 0..(gwidth-h) {
                for off_x in 0..(gwidth-w) {
                    sms = grid_or(sms, grid_im_check(grid.clone(), &seamon, off_x, off_y));
                }
            }

            seamon = rotate_grid(&seamon);
        }
        
        seamon = flip_grid_hori(&seamon);
    }

    debug_print_grid(&sms);

    // Non sea monster grid
    let nonsm = grid_and(grid, grid_not(sms));
    nonsm.iter().map(|v| v.iter().filter(|x| **x).count()).sum()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");
    let tiles = parse_tiles(&contents);

    let corners = find_pieces_with_n_edges(&tiles, 2);
    println!("Corners are {:?}", corners);
    let product = corners.iter().fold(1, |a, c| a*c);
    println!("Product is {}", product);

    let mut solver = Solver::new(tiles);
    solver.solve();
    solver.print_positions();

    let p2_sm = count_non_seamonster(solver.make_grid());
    println!("Part 2 non seamonsters = {}", p2_sm);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let sample = include_str!("test_sample_1.txt");
        let tiles = parse_tiles(sample);
        assert!(tiles.get(&3079).is_some());
        assert_eq!(tiles.len(), 9);

        for (_, tile) in tiles.iter() {
            for edge in &tile.edges {
                assert_eq!(edge.len(), 10);
            }
        }

        assert!(tiles[&2311].grid[0][3]);
        assert!(!tiles[&2311].grid[1][3]);
        assert_eq!(tiles[&2311].edges[1][2..=4], [false, true, false]);
        assert_eq!(tiles[&2311].edges[3][0..=4], [false, true, false, false, true]);

        let corners = find_pieces_with_n_edges(&tiles, 2);
        println!("Corners: {:?}", corners);
        let product = corners.iter().fold(1, |a, c| a*c);
        assert_eq!(product, 20899048083289);

        println!("Edges: {:?}", find_pieces_with_n_edges(&tiles, 1));

        let tile = tiles[&2311].clone();
        assert_ne!(tile.flip_horizontally(), tile);
        assert_eq!(tile.flip_horizontally().flip_horizontally(), tile);
        assert_eq!(tile.rotate(4), tile);

        let mut solver = Solver::new(tiles);
        solver.solve();
        solver.print_positions();
        println!("+===+");
        debug_print_grid(&solver.make_grid());
        println!("+===+");

        assert_eq!(count_non_seamonster(solver.make_grid()), 273);
    }

    #[test]
    fn test_flips() {
        let ta = Tile::parse("Tile 0:
##..
.##.
..##
...#");

        let tf = Tile::parse("Tile 0:
..##
.##.
##..
#...");

        assert_eq!(ta.flip_horizontally(), tf);
        
        let tr = Tile::parse("Tile 0:
...#
..##
.##.
##..");

        assert_eq!(ta.rotate(1), tr);
    }
}