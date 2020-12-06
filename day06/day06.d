import std.stdio;
import std.algorithm.iteration;
import std.algorithm;
import std.array;
import std.string;
import std.file;

// THis whole thing would be easier with sets... Why doesnt D have a standard set container?

char[] set_union(char[] a, char[] b)
{
    // Wtf D... Why is filter arbitrarily convert my data type char to dchar?
    return a ~ b.filter!(c => !a.canFind(c)).map!(x => cast(char)x).array();
}

char[] set_intersection(char[] a, char[] b)
{
    return a.filter!(c => b.canFind(c)).map!(x => cast(char)x).array();
}

/// Split the file into groups of people, in turn each being a group of characters
char[][][] groups(string contents)
{
    return contents.splitter("\n\n")
        .map!(g => g.splitter("\n")
                    .filter!(s => s.length > 0)
                    .map!(s => s.strip().dup).array())
        .array();
}

/// Part 1 count (i.e. letters anyone has)
ulong count_p1(char[][][] groups)
{
    char[] empty;
    return groups
        .map!(group => reduce!(set_union)(empty, group).length)
        .sum();
}

/// Part 2 count (letters everyone has per group)
ulong count_p2(char[][][] groups)
{
    return groups
        .map!(group => reduce!(set_intersection)(group[0], group[1..group.length]).length)
        .sum();
}

void main()
{
    string contents = readText("input.txt");
    writeln("Part 1: ", contents.groups().count_p1());
    writeln("Part 2: ", contents.groups().count_p2());
}

unittest
{
    string contents = "abc

a
b
c

ab
ac

a
a
a
a

b";

    assert(contents.groups().count_p1() == 11);
    assert(contents.groups().count_p2() == 6);
}