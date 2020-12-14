#include <iostream>

#include <string>
#include <vector>
#include <regex>
#include <map>

#include <cassert>
#include <cstdint>

#include <boost/filesystem/string_file.hpp>
#include <boost/algorithm/string.hpp>

using namespace std;

typedef uint64_t u64;

constexpr u64 BITMASK_U36 = 0xFFFFFFFFF;

enum class InsKind
{
    MASK,
    MEM
};

struct Instruction
{
    InsKind kind;
    u64 a;
    u64 b;
};

Instruction parse_mask(string maskstr)
{
    u64 mask = 0;
    u64 val = 0;
    for (char c : maskstr)
    {
        val <<= 1;
        mask <<= 1;

        if (c == '1')
        {
            val |= 1;
            mask |= 1;
        }
        else if (c == '0')
        {
            mask |= 1;
        }
    }
    
    // Verify value bits are masked
    assert((val & ~mask) == 0);
    return { InsKind::MASK, mask, val };
}

vector<Instruction> parse_instructions(string contents)
{
    regex mask_pat(R"(mask = ([01X]+))");
    regex mem_pat(R"(mem\[([0-9]+)\] = ([0-9]+))");

    vector<string> lines;
    boost::split(lines, contents, boost::is_any_of("\n"));

    vector<Instruction> ins;

    for (auto& line : lines)
    {
        smatch m;
        if (regex_match(line, m, mask_pat))
        {
            string mask = m[1];
            ins.push_back(parse_mask(mask));
        }
        else if (regex_match(line, m, mem_pat))
        {
            u64 a = stoull(m[1]) & BITMASK_U36;
            u64 b = stoull(m[2]) & BITMASK_U36;
            ins.push_back({ InsKind::MEM, a, b });
        }
        else
        {
            // Ignore line
        }
    }

    return ins;
}

u64 mask_val(Instruction maskins, u64 val)
{
    assert(maskins.kind == InsKind::MASK);
    u64 mask = maskins.a;
    u64 altv = maskins.b;
    return (val & ~mask) | altv;
}

u64 summem(map<u64, u64>& memory)
{
    u64 sum = 0;
    for (auto const& x : memory)
    {
        sum += x.second;
    }
    return sum;
}

u64 p1_exec(vector<Instruction>& program)
{
    map<u64, u64> memory;

    Instruction cmask = { InsKind::MEM, 0, 0 };
    for (auto& ins : program)
    {
        if (ins.kind == InsKind::MASK)
        {
            cmask = ins;
        }
        else
        {
            // Check we've already seen a mask
            assert(cmask.kind == InsKind::MASK);
            memory[ins.a] = mask_val(cmask, ins.b);
        }
    }

    return summem(memory);
}

/** 
 * Find the next value against a mask,
 * E.g. if the mask is 0b101 then the values are:
 * 0b0, 0b1, 0b100, 0b101, repeat
 *
 * Basically performs addition in a loop over bits
 */
u64 next_masked(u64 mask, u64 val)
{
    u64 carry = 1;
    u64 out = 0;

    u64 inbit = 0;
    u64 outbit = 0;

    for (u64 i = 0; i < sizeof(u64) * 8; i++)
    {
        inbit = (val >> i) & 1;
        if (((mask >> i) & 1) == 1)
        {
            outbit = inbit ^ carry;
            carry = inbit & carry;
        }
        else
        {
            outbit = inbit;
        }
        out |= outbit << i;
    }

    return out;
}

u64 p2_exec(vector<Instruction>& program)
{
    map<u64, u64> memory;

    Instruction cmask = { InsKind::MEM, 0, 0 };
    for (auto& ins : program)
    {
        if (ins.kind == InsKind::MASK)
        {
            cmask = ins;
        }
        else
        {
            // Check we've already seen a mask
            assert(cmask.kind == InsKind::MASK);
            u64 floatmask = ~(cmask.a) & BITMASK_U36;
            u64 overwrite = cmask.b;
            u64 start_addr = ins.a | overwrite;
            u64 addr = start_addr;

            do
            {
                memory[addr] = ins.b;
                addr = next_masked(floatmask, addr);
            } while (addr != start_addr);
        }
    }

    return summem(memory);
    return 0;
}

void test()
{
    {
        const char* sample = R"(mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0)";

        auto ins = parse_instructions(string(sample));
        assert(ins.size() == 4);
        assert(ins[0].kind == InsKind::MASK);
        assert(ins[0].a == 0b1000010);
        assert(ins[0].b == 0b1000000);
        assert(ins[1].kind == InsKind::MEM);
        assert(ins[1].a == 8);
        assert(ins[1].b == 11);

        assert(mask_val(ins[0], 11) == 73);
        assert(mask_val(ins[0], 101) == 101);

        assert(p1_exec(ins) == 165);
    }

    {
        assert(next_masked(0b101, 0) == 0b1);
        assert(next_masked(0b101, 0b1) == 0b100);
        assert(next_masked(0b101, 0b101) == 0);
        assert(next_masked(0b101, 0b111) == 0b10);

        const char* sample = R"(mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1)";

        auto program = parse_instructions(string(sample));
        assert(p2_exec(program) == 208);
    }
}

void main()
{
    test();

    string contents;
    boost::filesystem::load_string_file("input.txt", contents);
    auto program = parse_instructions(contents);

    auto p1 = p1_exec(program);
    cout << "Part 1 = " << p1 << endl;

    auto p2 = p2_exec(program);
    cout << "Part 2 = " << p2 << endl;
}