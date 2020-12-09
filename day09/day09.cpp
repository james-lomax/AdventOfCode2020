#include <iostream>

#include <string>
#include <vector>
#include <regex>
#include <algorithm>
#include <numeric>

#include <cassert>
#include <cstdint>

#include <fstream>
#include <sstream>

#include <boost/filesystem/string_file.hpp>

using namespace std;

vector<uint64_t> parse_nums(string contents)
{
    vector<uint64_t> nums;
    regex r("[0-9]+");

    sregex_iterator i = sregex_iterator(contents.begin(), contents.end(), r);
    for (; i != sregex_iterator(); ++i)
    {
        smatch m = *i;
        string d = m.str();
        nums.push_back(stoull(d));
    }

    return nums;
}

bool check_rule(vector<uint64_t>& nums, size_t i, size_t previous)
{
    for (auto j = i - previous; j < i; j++)
    {
        for (auto k = j + 1; k < i; k++)
        {
            if (nums[j] + nums[k] == nums[i])
            {
                return true;
            }
        }
    }
    return false;
}

uint64_t rule_breaker(vector<uint64_t>& nums, size_t previous)
{
    for (auto i = previous; i < nums.size(); i++)
    {
        if (!check_rule(nums, i, previous))
        {
            return nums[i];
        }
    }
    return 0;
}

uint64_t p2(vector<uint64_t>& nums, uint64_t rule_breaker)
{
    for (size_t len = 2; len < nums.size(); len++)
    {
        for (auto i = nums.begin(); i != nums.end() - len; i++)
        {
            auto sum = accumulate(i, i + len, (uint64_t)0);
            if (sum == rule_breaker)
            {
                auto min = min_element(i, i + len);
                auto max = max_element(i, i + len);
                return *min + *max;
            }
        }
    }
    assert(false);
    return 0;
}

void test_sample()
{
    const char* sample = R"(
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576
        )";
    
    auto nums = parse_nums(string(sample));
    auto rb = rule_breaker(nums, 5);
    assert(rb == 127);

    assert(p2(nums, rb) == 62);

}

void main()
{
    test_sample();

    string contents;
    boost::filesystem::load_string_file("input.txt", contents);
    auto nums = parse_nums(contents);
    auto rb = rule_breaker(nums, 25);
    cout << "Part 1: " << rb << endl;
    cout << "Part 2: " << p2(nums, rb) << endl;
}