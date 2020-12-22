cmake_minimum_required (VERSION 3.8)

project ("day22")

set (CMAKE_CXX_STANDARD 17)

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

# Add source to this project's executable.
add_executable (day22 "day22.cpp")
target_link_libraries(day22 ${CONAN_LIBS})
