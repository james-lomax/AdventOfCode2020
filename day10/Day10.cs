using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text.RegularExpressions;
using System.IO;

class Day10
{
    static List<ulong> ParseAndSortInts(string contents)
    {
        var rx = new Regex(@"[0-9]+");
        var nums = rx.Matches(contents).Select(m => UInt64.Parse(m.Groups[0].Value)).ToList();
        nums.Sort();
        return nums;
    }

    static ulong BuiltInRating(List<ulong> sorted)
    {
        return sorted.Last() + 3;
    }

    static Dictionary<ulong, ulong> ChainDistribution(List<ulong> sorted)
    {
        var occurs = new Dictionary<ulong, ulong>();
        occurs[1] = 0;
        occurs[2] = 0;
        occurs[3] = 0;

        ulong cur = 0;
        foreach (ulong n in sorted)
        {
            ulong diff = n - cur;
            Debug.Assert(diff >= 1 && diff <= 3);
            occurs[diff] += 1;
            cur = n;
        }

        return occurs;
    }

    static ulong CountArrangements(List<ulong> sorted)
    {
        var nums = new List<ulong>(sorted);
        nums.Insert(0, 0);

        // The idea is essentially to build a tree from the BuiltInRating to 0 and count the number of ways to get there
        var reach = nums.ToDictionary(x => x, x => (ulong)0);

        var toVisit = new SortedSet<ulong>();
        var builtin = BuiltInRating(nums);
        toVisit.Add(builtin);
        reach[builtin] = 1;

        while (toVisit.Count > 0)
        {
            var visiting = toVisit.Max;
            toVisit.Remove(visiting);

            var rch = reach[visiting];

            foreach (var n in nums)
            {
                var diff = visiting - n;
                if (diff >= 1 && diff <= 3)
                {
                    // We can reach `n` from `visiting`
                    reach[n] += rch;
                    toVisit.Add(n);
                }
            }
        }

        return reach[0];
    }

    static ulong Part1(List<ulong> sorted)
    {
        var nums = new List<ulong>(sorted);
        nums.Add(BuiltInRating(nums));
        var occurs = ChainDistribution(nums);
        return occurs[1] * occurs[3];
    }

    static void Main(string[] args)
    {
        Test1();
        Test2();

        string contents = File.ReadAllText("input.txt");
        var nums = ParseAndSortInts(contents);
        var p1 = Part1(nums);
        Console.WriteLine("Part 1: " + p1);
        var p2 = CountArrangements(nums);
        Console.WriteLine("Part 2: " + p2);
    }

    static void Test1()
    {
        string sample = @"16
                10
                15
                5
                1
                11
                7
                19
                6
                12
                4";

        var nums = ParseAndSortInts(sample);
        var builtin = BuiltInRating(nums);
        Debug.Assert(builtin == 22);
        var np = new List<ulong>(nums);
        np.Add(builtin);
        var occurs = ChainDistribution(np);
        Debug.Assert(occurs[1] == 7);
        Debug.Assert(occurs[3] == 5);

        var na = CountArrangements(nums);
        Debug.Assert(na == 8);
    }

    static void Test2()
    {
        string sample = "28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3";
        var nums = ParseAndSortInts(sample);
        Debug.Assert(Part1(nums) == 220);
        var na = CountArrangements(nums);
        Debug.Assert(na == 19208);
    }
}
