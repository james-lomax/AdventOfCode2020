const test_input = `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`;

const { assert } = require('console');
const fs = require('fs');

function toGrid(contents) {
    return contents.split("\n")
        .map(s => s.trim().split(""))
}

function traverse(grid, vx, vy) {
    let x = 0;
    let y = 0;

    let trees = 0;

    while (y < grid.length) {
        let sublen = grid[y].length;

        if (grid[y][x % sublen] == '#') {
            trees++;
        }

        x += vx;
        y += vy;
    }

    return trees;
}

const test_grid = toGrid(test_input);
assert(traverse(test_grid, 3, 1) == 7, "traverse(...) failed");

const grid = toGrid(fs.readFileSync("input.txt", "utf8"));

console.log(traverse(grid, 3, 1));


const slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];

function mult_slopes(grid, slopes) {
    return slopes.map(s => traverse(grid, s[0], s[1])).reduce((acc, v) => acc*v, 1);
}

assert(mult_slopes(test_grid, slopes) == 336, "mult_slopes(...) failed")

console.log(mult_slopes(grid, slopes));

//console.log(grid.length)
//console.log(grid[0])
