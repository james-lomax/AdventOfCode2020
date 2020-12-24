const assert = require('assert');

Array.prototype.equals = function (arr) {
    return this.length == arr.length && this.every((u, i) => u === arr[i]);
}

function parse(contents) {
    return contents.split("").map((i) => parseInt(i));
}

assert(parse("1234").equals([1,2,3,4]));

function move(lastNums) {
    const low = 1;
    const high = 9;
    let current = lastNums[0];
    let nums = lastNums.slice();
    let picked = nums.splice(1, 3);

    let dest = current - 1;
    while (nums.indexOf(dest) < 0) {
        if (dest-- < low) {
            dest = high;
        }
    }

    //console.log(`Picked = ${picked.join(" ")}`);
    //console.log(`Dest = ${dest} @ ${nums.indexOf(dest)}`);

    nums.splice(nums.indexOf(dest) + 1, 0, ...picked);
    nums.splice(0, 1);  // Move current to back
    nums.push(current);

    return nums;
}

function runfor(numStr, count) {
    let nums = parse(numStr);
    for (let i = 0; i < count; i++) {
        nums = move(nums);
    }

    let startIndex = nums.indexOf(1);
    let out = nums.slice(startIndex + 1).concat(nums.slice(0, startIndex));
    return out.join("");
}

assert.strictEqual(runfor("389125467", 10), "92658374");
assert.strictEqual(runfor("389125467", 100), "67384529");

console.log("Part 1 = " + runfor("853192647", 100));
