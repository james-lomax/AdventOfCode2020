using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace day11
{
    enum Cell
    {
        None,
        FreeSeat,
        OccupiedSeat
    }

    class Grid
    {
        public enum Algorithm
        { 
            Part1Adjacent,
            Part2LineOfSight
        }

        private Cell[,] SeatGrid;
        public int Width { get; private set; }
        public int Height { get; private set; }
        public bool Stable { get; private set; }
        public Algorithm AlgoMode { get; private set; }

        public Grid(Cell[,] grid, Algorithm algo)
        {
            this.SeatGrid = grid;
            Width = grid.GetLength(0);
            Height = grid.GetLength(1);
            Stable = false;
            AlgoMode = algo;
        }

        public Cell Get(int x, int y)
        {
            if (x >= Width || x < 0 || y >= Height || y < 0)
                return Cell.None;
            return SeatGrid[x, y];
        }

        public void RunUntilStable()
        {
            while (!Stable)
            {
                Step();
            }
        }

        public void Step()
        {
            var next = new Cell[Width, Height];

            for (var x = 0; x < Width; x++)
            {
                for (var y = 0; y < Height; y++)
                {
                    next[x, y] = Next(x, y);
                }
            }

            if (SeatGrid.Cast<Cell>().SequenceEqual(next.Cast<Cell>()))
            {
                Stable = true;
            }

            SeatGrid = next;
        }

        public Cell Next(int x, int y)
        {
            switch (Get(x, y))
            {
                case Cell.None:
                    return Cell.None;
                case Cell.FreeSeat:
                    return CountSeatsIncident(x, y) == 0 ? Cell.OccupiedSeat : Cell.FreeSeat;
                case Cell.OccupiedSeat:
                    return CountSeatsIncident(x, y) >= GetSeatTolerance() ? Cell.FreeSeat : Cell.OccupiedSeat;
                default:
                    Debug.Assert(false);
                    return Cell.None;
            }
        }

        public int GetSeatTolerance()
        {
            return AlgoMode == Algorithm.Part2LineOfSight ? 5 : 4;
        }

        public int CountSeatsIncident(int cx, int cy)
        {
            int count = 0;
            for (var dx = -1; dx <= 1; dx++)
            {
                for (var dy = -1; dy <= 1; dy++)
                {
                    if ((dx != 0 || dy != 0) && CheckDirection(cx, cy, dx, dy))
                    {
                        count++;
                    }
                }
            }

            return count;
        }

        private bool CheckDirection(int ox, int oy, int dx, int dy)
        {
            switch (AlgoMode)
            {
                case Algorithm.Part1Adjacent:
                    return Get(ox + dx, oy + dy) == Cell.OccupiedSeat;
                case Algorithm.Part2LineOfSight:
                    return ProjectHit(ox, oy, dx, dy);
                default:
                    Debug.Assert(false);
                    return false;
            }
        }

        private bool ProjectHit(int ox, int oy, int dx, int dy)
        {
            int x = ox;
            int y = oy;

            while (x <= Width && x >= 0 && y <= Width && y >= 0)
            {
                x += dx;
                y += dy;

                var c = Get(x, y);
                if (c == Cell.OccupiedSeat)
                {
                    return true;
                }
                else if (c == Cell.FreeSeat)
                {
                    return false;
                }
            }

            return false;
        }

        public int CountOccupied()
        {
            return SeatGrid.Cast<Cell>().Where(c => c == Cell.OccupiedSeat).Count();
        }

        public void DebugPrint()
        {
            Console.WriteLine(">>>");
            for (var y = 0; y < Height; y++)
            {
                for (var x = 0; x < Width; x++)
                {
                    Console.Write(FmtCell(Get(x, y)));
                }
                Console.WriteLine();
            }
            Console.WriteLine("<<<");
        }

        public static Grid Parse(string contents, Algorithm algo = Algorithm.Part1Adjacent)
        {
            var rowPat = new Regex(@"[L\.#]+");
            var rows = rowPat.Matches(contents)
                .Select(m => m.Value.Select(ParseCell).ToList())
                .ToList();

            var width = rows.Max(x => x.Count);
            var height = rows.Count;
            var grid = new Cell[width, height];
            for (var y = 0; y < height; y++)
            {
                var row = rows[y];
                Debug.Assert(row.Count == width);
                for (var x = 0; x < width; x++)
                {
                    grid[x, y] = row[x];
                }
            }

            return new Grid(grid, algo);
        }

        private static char FmtCell(Cell c)
        {
            switch (c)
            {
                case Cell.None:
                    return '.';
                case Cell.FreeSeat:
                    return 'L';
                case Cell.OccupiedSeat:
                    return '#';
                default:
                    return '?';
            }
        }

        private static Cell ParseCell(char c)
        {
            switch (c)
            {
                case '.':
                    return Cell.None;
                case 'L':
                    return Cell.FreeSeat;
                case '#':
                    return Cell.OccupiedSeat;
                default:
                    Debug.Assert(false);
                    return Cell.None;
            }
        }
    }

    class Day11
    {
        static void Test1()
        {
            string sample = 
              @"L.LL.LL.LL
                LLLLLLL.LL
                L.L.L..L..
                LLLL.LL.LL
                L.LL.LL.LL
                L.LLLLL.LL
                ..L.L.....
                LLLLLLLLLL
                L.LLLLLL.L
                L.LLLLL.LL";

            var grid = Grid.Parse(sample);
            Debug.Assert(grid.Get(0, 0) == Cell.FreeSeat);
            Debug.Assert(grid.Get(1, 0) == Cell.None);

            grid.Step();
            Debug.Assert(grid.Get(0, 0) == Cell.OccupiedSeat);

            grid.RunUntilStable();
            var o = grid.CountOccupied();
            Debug.Assert(o == 37);

            var grid2 = Grid.Parse(sample, Grid.Algorithm.Part2LineOfSight);
            grid2.RunUntilStable();
            Debug.Assert(grid2.CountOccupied() == 26);
        }

        static void Test2()
        {
            string sample1 = @".......#.
                ...#.....
                .#.......
                .........
                ..#L....#
                ....#....
                .........
                #........
                ...#.....";

            Debug.Assert(Grid.Parse(sample1, Grid.Algorithm.Part2LineOfSight).CountSeatsIncident(3, 4) == 8);

            string sample2 = @".##.##.
                #.#.#.#
                ##...##
                ...L...
                ##...##
                #.#.#.#
                .##.##.";

            Debug.Assert(Grid.Parse(sample2, Grid.Algorithm.Part2LineOfSight).CountSeatsIncident(3, 3) == 0);

            string sample3 =
              @".............
                .L.L.#.#.#.#.
                .............";

            Debug.Assert(Grid.Parse(sample3, Grid.Algorithm.Part2LineOfSight).CountSeatsIncident(1, 1) == 0);
        }

        static void Main(string[] args)
        {
            Test2();
            Test1();

            string contents = File.ReadAllText("input.txt");
            
            var grid = Grid.Parse(contents);
            grid.RunUntilStable();
            var p1 = grid.CountOccupied();
            Console.WriteLine("Part1: Occupied = " + p1);

            var grid2 = Grid.Parse(contents, Grid.Algorithm.Part2LineOfSight);
            grid2.RunUntilStable();
            var p2 = grid2.CountOccupied();
            Console.WriteLine("Part2: Occupied = " + p2);
        }
    }
}
