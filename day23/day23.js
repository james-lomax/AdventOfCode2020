const assert = require('assert');

Array.prototype.equals = function (arr) {
    return this.length == arr.length && this.every((u, i) => u === arr[i]);
}

function parse(contents) {
    return contents.split("").map((i) => parseInt(i));
}

assert(parse("1234").equals([1,2,3,4]));

function intoLinkedList(nums) {
    let m = new Array(nums.length+1).fill(0);
    for (let i = 0; i < nums.length; i++) {
        m[nums[i]] = nums[(i + 1) % nums.length];
    }
    return m;
}

function intoNumsAfter1(ll) {
    let nums = [];
    let cur = 1;
    while (nums.length < ll.length-2) {
        cur = ll[cur];
        nums.push(cur);
    }
    return nums;
}

function step(cur, ll) {
    let low = 1;
    let high = ll.length - 1;

    // Collect three cups following current
    let next = ll[cur];
    let picked = [];
    while (picked.length < 3) {
        picked.push(next);
        next = ll[next];
    }

    // Skip those three nums
    ll[cur] = next;

    let dest = cur - 1;
    while (picked.indexOf(dest) >= 0 || dest < low) {
        if (--dest < low) {
            dest = high;
        }
    }

    // console.log(`Cur = ${cur}`);
    // console.log(`Picked = ${picked.join(" ")}`);
    // console.log(`Dest = ${dest}`);

    // Re-insert after dest
    let end = ll[dest];
    ll[dest] = picked[0];
    ll[picked[2]] = end;

    return ll[cur];
}

function runfor(numStr, count) {
    let nums = parse(numStr);
    let ll = intoLinkedList(nums);
    let cur = nums[0];
    for (let i = 0; i < count; i++) {
        cur = step(cur, ll);
    }

    return intoNumsAfter1(ll).join("");
}

let n1 = intoLinkedList(parse("389125467"));
assert(n1.equals([0, 2, 5, 8, 6, 4, 7, 3, 9, 1]));
assert.strictEqual(runfor("389125467", 10), "92658374");
assert.strictEqual(runfor("389125467", 100), "67384529");

console.log("Part 1 = " + runfor("853192647", 100));


function part2(numStr) {
    let nums = parse(numStr);

    let ll = intoLinkedList(nums);
    
    // Extend to 1 mil
    ll[nums[nums.length-1]] = ll.length;
    for (let i = ll.length; i < 1000000; i++) {
        ll.push(i+1);
    }
    ll.push(nums[0]);

    // Step 10 mil times
    let cur = nums[0];
    for (let i = 0; i < 10000000; i++) {
        cur = step(cur, ll);
    }

    // Find the numbers after 1
    let n1 = ll[1];
    let n2 = ll[n1];

    console.log(`${n1} and ${n2}`);

    return n1*n2;
}

assert.strictEqual(part2("389125467"), 149245887792);

let p2 = part2("853192647");
console.log(`Part 2 = ${p2}`);
