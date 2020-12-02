const fs = require('fs');

function check1(min, max, c, s) {
    let n = s.split("").filter(x => x == c).length;
    return n >= min && n <= max;
}

function xor(a, b) {
    return (a && !b) || (!a && b);
}

function check2(p1, p2, c, s) {
    return xor(s[p1-1] == c, s[p2-1] == c);
}

function count(contents, check) {
    let re = /([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)/;
    return contents.split("\n")
        .map(l => l.match(re))
        .filter(m => m)
        .filter(m => check(
            parseInt(m[1]),
            parseInt(m[2]),
            m[3],
            m[4])
        ).length;
}

const contents = fs.readFileSync("input.txt", "utf8")
console.log(`Part1. Count = ${count(contents, check1)}`);
console.log(`Part2. Count = ${count(contents, check2)}`);
