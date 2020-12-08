#include <cstdint>
#include <cassert>

#include <iostream>
#include <vector>
#include <fstream>
#include <exception>
#include <string>
#include <sstream>
#include <set>

using namespace std;

enum class Op
{
	NOP,
	ACC,
	JMP
};

struct OpCode
{
	Op op;
	int32_t val;
};

bool is_whitespace(char c)
{
	return c == '\n' || c == '\t' || c == '\r' || c == ' ';
}

string strip(string& s)
{
	size_t start, end;
	for (start = 0; start < s.length() && is_whitespace(s[start]); start++)
	{ }

	for (end = s.length() - 1; end >= start && is_whitespace(s[end]); end--)
	{ }

	return s.substr(start, end - start + 1);
}

/** Split into list of lines and trim whitespace */
vector<string> splitlines(string& contents)
{
	vector<string> lines;

	size_t lastpos = 0;
	size_t delimpos = 0;
	while ((delimpos = contents.find('\n', lastpos + 1)) != string::npos)
	{
		string line = strip(contents.substr(lastpos, delimpos - lastpos));
		if (line.length() > 0)
		{
			lines.push_back(line);
		}
		lastpos = delimpos;
	}

	string line = strip(contents.substr(lastpos, contents.length() - lastpos));
	if (line.length() > 0)
	{
		lines.push_back(line);
	}

	return lines;
}

vector<OpCode> parse(string& instructions)
{
	vector<OpCode> ops;
	vector<string> lines = splitlines(instructions);

	for (string& line : lines)
	{
		Op op;

		if (line.compare(0, 3, "nop") == 0)
		{
			op = Op::NOP;
		}
		else if (line.compare(0, 3, "acc") == 0)
		{
			op = Op::ACC;
		}
		else if (line.compare(0, 3, "jmp") == 0)
		{
			op = Op::JMP;
		}
		else
		{
			throw std::runtime_error("Bad operator");
		}

		auto val_s = line.substr(5, line.length() - 5);
		int32_t val = stoi(val_s);
		if (line[4] == '-')
		{
			val *= -1;
		}

		ops.push_back(OpCode{ op, val });
	}

	return ops;
}

struct Machine
{
	vector<OpCode> ops;
	size_t pc;
	int32_t acc;

	Machine(vector<OpCode>& ops)
		: ops(ops)
		, pc(0)
		, acc(0)
	{ }

	bool terminated()
	{
		return pc >= ops.size();
	}

	void step()
	{
		if (terminated())
		{
			return;
		}

		switch (ops[pc].op)
		{
		case Op::JMP:
			pc += ops[pc].val;
			break;
		case Op::ACC:
			acc += ops[pc].val;
		case Op::NOP:
		default:
			pc += 1;
			break;
		}
	}
};

void run_to_loop(Machine& machine)
{
	set<size_t> pc_seen;

	while (pc_seen.count(machine.pc) == 0 && !machine.terminated())
	{
		pc_seen.insert(machine.pc);
		machine.step();
	}
}

int32_t fix_program(vector<OpCode>& ops)
{
	for (size_t i = 0; i < ops.size(); i++)
	{
		if (ops[i].op == Op::JMP || ops[i].op == Op::NOP)
		{
			Machine m(ops);
			m.ops[i].op = ops[i].op == Op::JMP ? Op::NOP : Op::JMP;
			run_to_loop(m);

			if (m.terminated())
			{
				return m.acc;
			}
		}
	}
}

void test_sample()
{
	const char* sample = R"(
		nop +0
		acc +1
		jmp +4
		acc +3
		jmp -3
		acc -99
		acc +1
		jmp -4
		acc +6)";

	auto ops = parse(std::string(sample));
	assert(ops[0].op == Op::NOP);
	assert(ops[2].op == Op::JMP);
	assert(ops[2].val == 4);
	assert(ops[4].val == -3);

	Machine m(ops);
	run_to_loop(m);
	assert(!m.terminated());
	assert(m.acc == 5);

	assert(fix_program(ops) == 8);
}

int main()
{
	test_sample();

	std::ifstream infile("input.txt");
	assert(infile.is_open());

	stringstream buffer;
	buffer << infile.rdbuf();
	std::string contents = buffer.str();
	auto ops = parse(contents);
	assert(ops.size() > 0);

	Machine m(ops);
	run_to_loop(m);

	cout << "Part 1: " << m.acc << endl;

	cout << "Part 2: " << fix_program(ops) << endl;
	return 0;
}
