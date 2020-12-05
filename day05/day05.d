import std.regex;
import std.stdio;
import std.algorithm.iteration;

/// Get the ID of the seat from a boarding pass (of binary encoding)
int seat_id(string pass)
{
    static auto pat = regex(r"^[FB]{7}[LR]{3}$");
    assert(!matchFirst(pass, pat).empty);

    auto cMin = 0;
    auto cMax = 128*8 - 1;

    foreach (i, c; pass) {
        const auto sep = cMax - cMin;
        const auto extra = sep % 2;

        const auto half = cMin + (sep - extra) / 2;
        if (c == 'F' || c == 'L') {
            cMax = half;
        } else {
            cMin = half + extra;
        }
    }

    assert(cMin == cMax);
    return cMin;
}

void main()
{
    import std.algorithm.comparison : max;
    import std.algorithm.sorting : sort;
    import std.file : readText;
    import std.string : strip;
    import std.array : array;

    string contents = readText("input.txt");
    int[] passes = contents.splitter("\n")
        .map!(s => s.strip())
        .filter!(s => s.length > 0)
        .map!(s => seat_id(s))
        .array();

    int mx = passes.reduce!((acc, id) => max(acc, id));
    writeln("Max = ", mx);

    passes.sort!((a, b) => a < b);
    int last = passes[0];
    foreach (_, n; passes) {
        if (n - last == 2) {
            writeln("Missing seat = ", last + 1);
            break;
        }
        
        last = n;
    }
}

unittest
{
    assert(seat_id("FBFBBFFRLR") == 357);
    assert(seat_id("BFFFBBFRRR") == 567);
    assert(seat_id("FFFBBBFRRR") == 119);
    assert(seat_id("BBFFBBFRLL") == 820);
}