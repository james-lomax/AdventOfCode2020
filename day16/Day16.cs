using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace day16
{
    struct Rule
    {
        public string Name;
        public int R1Min;
        public int R1Max;
        public int R2Min;
        public int R2Max;

        public bool IsInRange(int n)
        {
            if (n >= R1Min && n <= R1Max)
                return true;
            else if (n >= R2Min && n <= R2Max)
                return true;
            else
                return false;
        }
    }

    class TicketInfo
    {
        public List<Rule> Rules;
        public List<int> MyTicket;
        public List<List<int>> Others;

        public TicketInfo(List<Rule> rules, List<int> myticket, List<List<int>> others)
        {
            Rules = rules;
            MyTicket = myticket;
            Others = others;
        }

        public int ScanErrorRate()
        {
            return Others
                .SelectMany(ns => ns)
                .Where(n => Rules.Where(r => r.IsInRange(n)).Count() == 0)
                .Sum();
        }

        public List<string> FindTicketDesc()
        {
            // Each field starts being any possibility
            List<List<Rule>> options = MyTicket.Select(_ => new List<Rule>(Rules)).ToList();

            var tickets = new List<List<int>>(Others);
            tickets.Add(MyTicket);

            // Reduce possibilities by finding and removing invalid fields
            for (var fieldIdx = 0; fieldIdx < options.Count; fieldIdx++)
            {
                var opts = options[fieldIdx];
                foreach (var ticket in tickets)
                {
                    for (var ruleIdx = 0; ruleIdx < opts.Count; ruleIdx++)
                    {
                        if (!opts[ruleIdx].IsInRange(ticket[fieldIdx]))
                        {
                            opts.RemoveAt(ruleIdx);
                            ruleIdx--;
                        }
                    }
                }
            }

            // Repeatedly take "lonely" fields until we have them all
            List<string> descs = Enumerable.Repeat<string>(null, Rules.Count).ToList();
            while (descs.Where(d => d == null).Count() > 0)
            {
                // Find a "lonely" rule
                int optIdx = options.FindIndex(o => o.Count == 1);
                string field = options[optIdx][0].Name;

                // Add to the descs
                descs[optIdx] = field;

                // Remove trace of the field from the options
                foreach (var optf in options)
                {
                    optf.RemoveAll(r => r.Name == field);
                }
            }

            return descs;
        }

        public void DiscardInvalidTickets()
        {
            Others = Others.Where(ticket => 
                ticket.Where(n => 
                    Rules.Where(r => 
                        r.IsInRange(n)
                    ).Count() == 0
               ).Count() == 0).ToList();
        }

        public static TicketInfo Parse(string infolist)
        {
            var subs = infolist.Replace("\r", "").Split("\n\n");
            Debug.Assert(subs.Length == 3);
            var rules = subs[0];
            var yourticket = subs[1];
            var othertickets = subs[2];

            var rl = ParseRules(rules);

            var ytks = ParseTickets(yourticket);
            Debug.Assert(ytks.Count == 1);

            var otks = ParseTickets(othertickets);

            return new TicketInfo(rl, ytks[0], otks);
        }

        static List<List<int>> ParseTickets(string tickets)
        {
            var rx = new Regex(@"(\d+)");
            var lines = tickets.Split("\n");
            return lines.Select(line => rx.Matches(line)
                .Select(m => Int32.Parse(m.Value)).ToList())
                .Where(lines => lines.Count > 0)
                .ToList();
        }

        static List<Rule> ParseRules(string rules)
        {
            var rx = new Regex(@" *([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)");
            return rx.Matches(rules).Select(m => new Rule
            {
                Name = m.Groups[1].Value,
                R1Min = Int32.Parse(m.Groups[2].Value),
                R1Max = Int32.Parse(m.Groups[3].Value),
                R2Min = Int32.Parse(m.Groups[4].Value),
                R2Max = Int32.Parse(m.Groups[5].Value)
            }).ToList();
        }
    }

    class Day16
    {
        static void Test1()
        {
            var sample = @"class: 1-3 or 5-7
                row: 6-11 or 33-44
                seat: 13-40 or 45-50

                your ticket:
                7,1,14

                nearby tickets:
                7,3,47
                40,4,50
                55,2,20
                38,6,12";

            var info = TicketInfo.Parse(sample);
            Debug.Assert(info.MyTicket.Count == 3);
            Debug.Assert(info.MyTicket[0] == 7);
            Debug.Assert(info.MyTicket[2] == 14);

            Debug.Assert(info.Rules.Count == 3);
            Debug.Assert(info.Rules[0].Name == "class");
            Debug.Assert(info.Rules[0].R1Min == 1);

            Debug.Assert(info.Others.Count == 4);

            Debug.Assert(info.ScanErrorRate() == 71);

            info.DiscardInvalidTickets();
            Debug.Assert(info.Others.Count == 1);
        }

        static void Test2()
        {
            var sample = @"class: 0-1 or 4-19
                row: 0-5 or 8-19
                seat: 0-13 or 16-19

                your ticket:
                11,12,13

                nearby tickets:
                3,9,18
                15,1,5
                5,14,9";

            var info = TicketInfo.Parse(sample);
            info.DiscardInvalidTickets();
            var descs = info.FindTicketDesc();
            Debug.Assert(descs.Count == 3);
            Debug.Assert(descs[0] == "row");
            Debug.Assert(descs[1] == "class");
            Debug.Assert(descs[2] == "seat");
        }

        static void Main(string[] args)
        {
            Test1();
            Test2();

            var contents = File.ReadAllText("input.txt");
            var info = TicketInfo.Parse(contents);
            int p1 = info.ScanErrorRate();
            Console.WriteLine("Part 1 scan error rate = " + p1);

            info.DiscardInvalidTickets();
            var descs = info.FindTicketDesc();
            ulong product = 1;
            for (var i = 0; i < descs.Count; i++)
            {
                if (descs[i].StartsWith("departure"))
                {
                    product *= (ulong)info.MyTicket[i];
                }
            }

            Console.WriteLine("Part 2 product = " + product);
        }
    }
}
