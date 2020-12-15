#include <iostream>
#include <vector>
#include <map>
#include <cassert>

using namespace std;

class Game
{
public:
    Game(vector<size_t>&& nums);

    size_t step();

    size_t step_til(size_t count);

    size_t last_num() const
    {
        return last;
    }

private:
    map<size_t, size_t> seen;
    size_t count;
    size_t last;

};

Game::Game(vector<size_t>&& nums)
{
    for (size_t i = 0; i < nums.size() - 1; i++)
    {
        seen[nums[i]] = i + 1;
    }
    count = nums.size();
    last = nums.back();
}

size_t Game::step()
{
    if (seen.count(last) > 0)
    {
        size_t last_idx = seen[last];
        size_t elapsed = count - last_idx;
        
        seen[last] = count;

        last = elapsed;
        count++;
    }
    else
    {
        seen[last] = count;
        last = 0;
        count++;
    }

    return last;
}

size_t Game::step_til(size_t target)
{
    while (count < target)
    {
        step();
    }
    return last;
}

void test()
{
    {
        Game game({ 0, 3, 6 });
        assert(game.step() == 0);
        assert(game.step() == 3);
        assert(game.step() == 3);
        assert(game.step() == 1);
        assert(game.step_til(2020) == 436);
    }

    {
        Game game({ 3, 1, 2 });
        assert(game.step_til(2020) == 1836);
        //assert(game.step_til(30000000) == 362);
    }
}

void main()
{
    test();

    Game game({ 12,1,16,3,11,0 });
    size_t p1 = game.step_til(2020);
    cout << "Part 1 = " << p1 << endl;

    size_t p2 = game.step_til(30000000);
    cout << "Part 2 = " << p2 << endl;
}