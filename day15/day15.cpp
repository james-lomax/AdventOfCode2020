#include <iostream>
#include <vector>
#include <map>
#include <unordered_map>
#include <array>
#include <cassert>
#include <chrono>

using namespace std;

template <class MapType = map<uint32_t, uint32_t>>
class Game
{
public:
    Game(vector<uint32_t> nums);

    uint32_t step();

    uint32_t step_til(uint32_t count);

    uint32_t last_num() const
    {
        return last;
    }

private:
    MapType seen;
    uint32_t count;
    uint32_t last;

};

template <class MapType>
Game<MapType>::Game(vector<uint32_t> nums)
{
    for (uint32_t i = 0; i < nums.size() - 1; i++)
    {
        seen[nums[i]] = i + 1;
    }
    count = nums.size();
    last = nums.back();
}

template <class MapType>
uint32_t Game<MapType>::step()
{
    if (seen.count(last) > 0)
    {
        uint32_t last_idx = seen[last];
        uint32_t elapsed = count - last_idx;
        
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

template <class MapType>
uint32_t Game<MapType>::step_til(uint32_t target)
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

template<class MapType>
void p2_time(string map_t_name, vector<uint32_t>& nums)
{
    Game<MapType> game(nums);
    auto start = chrono::high_resolution_clock::now();
    uint32_t p2 = game.step_til(30000000);
    auto stop = chrono::high_resolution_clock::now();
    auto duration = chrono::duration_cast<chrono::milliseconds>(stop - start); 
    cout << "Part 2 = " << p2 << " (with " << map_t_name << " in " << duration.count() << "ms)" << endl;
}

class HugeArrayMap
{
    const static uint32_t UNSEEN = (uint32_t)-1;

public:
    HugeArrayMap()
        : a(new array<uint32_t, 32000000>)
    {
        a->fill(UNSEEN);
    }
    
    ~HugeArrayMap()
    {
        delete a;
    }

    uint32_t& operator[](uint32_t n)
    {
        return (*a)[n];
    }

    uint32_t count(uint32_t n)
    {
        return (*a)[n] == UNSEEN ? 0 : 1;
    }

private:
    array<uint32_t, 32000000>* a;
};

void main()
{
    test();

    std::vector<uint32_t> nums = { 12,1,16,3,11,0 };
    Game game(nums);
    uint32_t p1 = game.step_til(2020);
    cout << "Part 1 = " << p1 << endl;

    p2_time<map<uint32_t, uint32_t>>("std::map", nums);
    p2_time<unordered_map<uint32_t, uint32_t>>("std::unordered_map", nums);
    p2_time<HugeArrayMap>("HugeArrayMap", nums);
}