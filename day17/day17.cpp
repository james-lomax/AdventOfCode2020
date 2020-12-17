#include <iostream>

#include <string>
#include <vector>
#include <regex>
#include <unordered_map>
#include <array>

#include <cassert>
#include <cstdint>

#include <boost/filesystem/string_file.hpp>
#include <boost/algorithm/string.hpp>
#include <boost/functional/hash.hpp>

using namespace std;

/**
 * Iterates directions, starting at -1, -1, ... and ending at 1, 1, ....
 * Skips, 0, 0, ... (as this isn't a direction)
 * Returns true if there are more directions and false if there are no more
 * 
 * Modified dir in place. Will reset dir to -1, -1, ... when finished.
 */
template <size_t N>
bool next_direction(array<int, N>& dir)
{
    bool carry = true;
    for (auto i = 0; i < N && carry; i++)
    {
        carry = false;
        dir[i]++;

        if (dir[i] > 1)
        {
            dir[i] = -1;
            carry = true;
        }
    }

    array<int, N> base;
    base.fill(0);
    if (base == dir)
    {
        return next_direction(dir);
    }
    else
    {
        return !carry;
    }
}

template<size_t N>
array<int, N> operator+(const array<int, N>& a, const array<int, N>& b)
{
    array<int, N> o;
    for (auto i = 0; i < N; i++)
    {
        o[i] = a[i] + b[i];
    }
    return o;
}

namespace std
{
    template <size_t N>
    struct hash<array<int, N>>
    {
        std::size_t operator()(const array<int, N>& k) const
        {
            size_t acc = 0;
            for (size_t i = 0; i < N; i++)
            {
                boost::hash_combine(acc, k[i]);
            }
            return acc;
        }
    };
}

template<size_t N>
class ConwayGrid
{
public:
    typedef array<int, N> Point;

    ConwayGrid(vector<vector<bool>>& grid);

    void step();

    size_t count_active();

private:
    unordered_map<Point, bool> m_alive;
    unordered_map<Point, int> m_count;

    void inc_count_at(Point p);

    /// Iterate all active cells and increment counts surrounding them
    void increment_step();

    /// Iterate the counts and update the alive map
    void progress_step();

};

template<size_t N>
ConwayGrid<N>::ConwayGrid(vector<vector<bool>>& grid)
{
    for (auto y = 0; y < grid.size(); y++)
    {
        for (auto x = 0; x < grid.size(); x++)
        {
            if (grid[y][x])
            {
                Point p;
                p.fill(0);
                p[0] = x;
                p[1] = y;
                m_alive[p] = true;
            }
        }
    }
}

template<size_t N>
void ConwayGrid<N>::step()
{
    increment_step();
    progress_step();
}

template<size_t N>
size_t ConwayGrid<N>::count_active()
{
    size_t count = 0;
    for (auto& it : m_alive)
    {
        if (it.second)
        {
            count++;
        }
    }
    return count;
}

template<size_t N>
void ConwayGrid<N>::inc_count_at(Point p)
{
    if (m_count.count(p) > 0)
    {
        m_count[p]++;
    }
    else
    {
        m_count[p] = 1;
    }
}

template<size_t N>
void ConwayGrid<N>::increment_step()
{
    m_count.clear();
    for (auto& it : m_alive)
    {
        if (it.second)
        {
            // Increment all surrounding cells
            Point dir;
            dir.fill(-1);
            do
            {
                inc_count_at(it.first + dir);
            } while (next_direction(dir));

            // Lonesome cubes need a 0 to kill them
            if (m_count.count(it.first) == 0)
            {
                m_count[it.first] = 0;
            }
        }
    }
}

template<size_t N>
void ConwayGrid<N>::progress_step()
{
    for (auto& it_c : m_count)
    {
        if (m_alive.count(it_c.first) > 0 && m_alive[it_c.first])
        {
            m_alive[it_c.first] = it_c.second == 2 || it_c.second == 3;
        }
        else
        {
            m_alive[it_c.first] = it_c.second == 3;
        }
    }

    // Purge dead cells
    auto it = begin(m_alive);
    while (it != end(m_alive))
    {
        if (it->second)
        {
            it++;
        }
        else
        {
            it = m_alive.erase(it);
        }
    }
}

vector<vector<bool>> parse(string contents)
{
    vector<vector<bool>> out;

    vector<string> lines;
    boost::split(lines, contents, boost::is_any_of("\n\r"));

    for (auto& line : lines)
    {
        auto l = boost::algorithm::trim_copy(line);
        if (l.length() > 0)
        {
            vector<bool> row;
            for (char c : l)
            {
                row.push_back(c == '#');
            }
            out.push_back(row);
        }
    }

    return out;
}

template<size_t N>
void step_until(ConwayGrid<N>& con, size_t n)
{
    for (auto i = 0; i < n; i++)
    {
        con.step();
    }
}

void test()
{
    const char* sample = R"(.#.
..#
###)";

    auto grid = parse(string(sample));
    assert(!grid[0][0]);
    assert(grid[2][2]);

    ConwayGrid<3> con(grid);
    assert(con.count_active() == 5);
    step_until(con, 6);
    assert(con.count_active() == 112);

    ConwayGrid<4> con4(grid);
    step_until(con4, 6);
    assert(con4.count_active() == 848);
}

void main()
{
    test();

    string contents;
    boost::filesystem::load_string_file("../../../../input.txt", contents);
    auto grid = parse(contents);

    ConwayGrid<3> con(grid);
    step_until(con, 6);
    auto p1 = con.count_active();
    cout << "Part 1 = " << p1 << endl;

    ConwayGrid<4> con4(grid);
    step_until(con4, 6);
    auto p2 = con4.count_active();
    cout << "Part 2 = " << p2 << endl;
}