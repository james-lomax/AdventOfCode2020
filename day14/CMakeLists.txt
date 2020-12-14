cmake_minimum_required (VERSION 3.8)

project ("day14")

set (CMAKE_CXX_STANDARD 17)

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

# Add source to this project's executable.
add_executable (day14 "day14.cpp")
target_link_libraries(day14 ${CONAN_LIBS})
