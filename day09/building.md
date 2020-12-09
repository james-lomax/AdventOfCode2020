See [conan 'getting started'](https://docs.conan.io/en/latest/getting_started.html)

```
[conan profile new default --detect]

mkdir build && cd build
conan install .. [-s build_type=Debug]
cmake ..
cmake --build .
```