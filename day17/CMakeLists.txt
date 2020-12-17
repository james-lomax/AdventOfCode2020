cmake_minimum_required (VERSION 3.8)

project ("day17")

set (CMAKE_CXX_STANDARD 17)

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

# Add source to this project's executable.
add_executable (day17 "day17.cpp")
target_link_libraries(day17 ${CONAN_LIBS})
