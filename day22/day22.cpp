#include <iostream>

#include <string>
#include <deque>
#include <unordered_set>

#include <cassert>
#include <cstdint>

#include <boost/regex.hpp>
#include <boost/filesystem/string_file.hpp>
#include <boost/algorithm/string.hpp>
#include <boost/functional/hash.hpp>

using namespace std;

struct Twople
{
    deque<int> a;
    deque<int> b;

    bool operator==(const Twople& other) const
    {
        return a == other.a && b == other.b;
    }
};

namespace std
{
    template <>
    struct hash<Twople>
    {
        std::size_t operator()(const Twople& k) const
        {
            size_t acc = 0;
            for (auto x : k.a)
            {
                boost::hash_combine(acc, x);
            }
            for (auto x : k.b)
            {
                boost::hash_combine(acc, x);
            }
            return acc;
        }
    };
}


void parse_numbers(string& s, deque<int>& deck)
{
    boost::regex num_pat("[0-9]+");
    boost::sregex_iterator j = boost::sregex_iterator(s.begin(), s.end(), num_pat);
    for (; j != boost::sregex_iterator(); ++j)
    {
        boost::smatch jm = *j;
        int i = stoi(jm[0].str());
        deck.push_back(i);
    }
}

void parse(string contents, deque<int>& deck_1, deque<int>& deck_2)
{
    boost::regex player_pat("Player 1:(.*)Player 2:(.*)");

    boost::smatch m;
    if (boost::regex_search(contents, m, player_pat))
    {
        if (m.size() >= 3)
        {
            parse_numbers(m[1].str(), deck_1);
            parse_numbers(m[2].str(), deck_2);
        }
        else
        {
            cerr << "Somehow the match happened but the size is < 3???" << endl;
            exit(EXIT_FAILURE);
        }
    }
    else
    {
        cerr << "Regex did not find a match!" << endl;
        exit(EXIT_FAILURE);
    }
}


void step_once(deque<int>& deck_1, deque<int>& deck_2)
{
    int n1 = deck_1.front(); 
    deck_1.pop_front();
    int n2 = deck_2.front(); 
    deck_2.pop_front();

    if (n1 > n2)
    {
        // Player 1 wins this round
        deck_1.push_back(n1);
        deck_1.push_back(n2);
    }
    else if (n2 > n1)
    {
        // Player 2 wins this round
        deck_2.push_back(n2);
        deck_2.push_back(n1);
    }
    else
    {
        cerr << "Two numbers the same..." << endl;
        exit(EXIT_FAILURE);
    }
}

void play_til_win(deque<int>& deck_1, deque<int>& deck_2)
{
    while (deck_1.size() > 0 && deck_2.size() > 0)
    {
        step_once(deck_1, deck_2);
    }
}

int score_winner(deque<int>& deck_1, deque<int>& deck_2)
{
    deque<int>* winner;
    if (deck_1.size() > 0)
    {
        winner = &deck_1;
    }
    else
    {
        winner = &deck_2;
    }

    int count = winner->size();
    int sum = 0;
    for (auto i = winner->begin(); i != winner->end(); ++i)
    {
        sum += *i * count;
        count--;
    }

    return sum;
}

int p1(deque<int> deck_1, deque<int> deck_2)
{
    play_til_win(deck_1, deck_2);
    return score_winner(deck_1, deck_2);
}

deque<int> slice(deque<int>& in, size_t count)
{
    deque<int> out;
    size_t n = 0;
    for (auto i = in.begin(); i != in.end() && n < count; ++i)
    {
        out.push_back(*i);
        n++;
    }
    return out;
}

// If score_out != nullptr, calculate the score of the winner
// Return true if deck_1 wins, false if deck_2 wins
bool r_game(deque<int> deck_1, deque<int> deck_2, int* score_out = nullptr)
{
    unordered_set<Twople> seen;

    while (deck_1.size() > 0 && deck_2.size() > 0)
    {
        Twople state = { deck_1, deck_2 };
        if (seen.count(state) > 0)
        {
            // Happened before, quit early in favour of p1
            if (score_out != nullptr)
            {
                *score_out = score_winner(deck_1, deck_2);
            }
            return true;
        }
        seen.insert(state);

        int n1 = deck_1.front();
        deck_1.pop_front();
        int n2 = deck_2.front();
        deck_2.pop_front();

        bool d1_win;

        if (deck_1.size() >= n1 && deck_2.size() >= n2)
        {
            d1_win = r_game(slice(deck_1, n1), slice(deck_2, n2));
        }
        else
        {
            assert(n1 != n2);
            d1_win = n1 > n2;
        }

        if (d1_win)
        {
            deck_1.push_back(n1);
            deck_1.push_back(n2);
        }
        else
        {
            deck_2.push_back(n2);
            deck_2.push_back(n1);
        }
    }

    if (score_out != nullptr)
    {
        *score_out = score_winner(deck_1, deck_2);
    }

    return deck_1.size() > 0;
}

int p2(deque<int> deck_1, deque<int> deck_2)
{
    int score;
    r_game(deck_1, deck_2, &score);
    return score;
}

void test()
{
    const char* sample = R"(Player 1: 9 2 6 3 1 Player 2: 5 8 4 7 10)";

    deque<int> deck_1, deck_2;
    parse(string(sample), deck_1, deck_2);

    assert(deck_1.size() == 5);
    assert(deck_1.back() == 1);
    assert(deck_1.front() == 9);
    assert(deck_2.size() == 5);
    assert(deck_2.back() == 10);
    assert(deck_2.front() == 5);

    assert(p1(deck_1, deck_2) == 306);

    assert(p2(deck_1, deck_2) == 291);

    Twople t = { deck_1, deck_2 };
    unordered_set<Twople> s;
    s.insert(t);
    assert(s.count(t) == 1);
}

void main()
{
    test();

    string contents;
    boost::filesystem::load_string_file("../../../../input.txt", contents);

    deque<int> deck_1, deck_2;
    parse(contents, deck_1, deck_2);
    cout << "Part 1 score = " << p1(deck_1, deck_2) << endl;
    cout << "Part 2 score = " << p2(deck_1, deck_2) << endl;
}