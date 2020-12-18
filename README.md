# [Advent of Code 2020](https://adventofcode.com/2020)

| Day | Language(s) | Theme |
|-----|-------------|-------|
|   1 | Rust        | Combinations |
|   2 | JavaScript  | Counting |
|   3 | JavaScript  | 2D grid |
|   4 | Python      | Boring rules |
|   5 | D           | Binary search |
|   6 | D           | Sets |
|   7 | Rust        | Tree traversal |
|   8 | C++         | Virtual Machine |
|   9 | C++         | Churn the numbers |
|  10 | C#          | Combinatorial |
|  11 | C#          | Rules and 2D grids |
|  12 | Go          | Manhatten ships |
|  13 | C           | Mod arithmetic |
|  14 | C++         | Bit manipulation |
|  15 | C++         | Memory game |
|  16 | C#          | Rules and deduction |
|  17 | C++         | Conways Game (3/4D) |
|  18 | Rust        | Parsing/Evaluation |

## Building C++ projects (with conan+CMake)

Use conan for easy boost installation. See [conan 'getting started'](https://docs.conan.io/en/latest/getting_started.html).

Having just installed conan, you will need to run:

```
conan profile new default --detect
```

To build a C++ day:

```
cd dayXX
mkdir build && cd build
conan install .. [-s build_type=Debug]
cmake ..
cmake --build .
./dayXX
```

Using visual studio, generate once, then configure conan libs with something like:

```
cd out/build/x64-Debug
conan install ../../../ -s build_type=Debug
```

Regenerate CMake in IDE and away!
