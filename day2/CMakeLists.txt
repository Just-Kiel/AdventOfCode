cmake_minimum_required(VERSION 3.13)

# give a name to the project
project(day2)

# file to compile and name of the executable
add_executable(day2 src/main.cpp)

# compilation options
target_compile_features(day2 PRIVATE cxx_std_17) # use at least c++ 11
# target_compile_options(simpleProject PRIVATE -Wall -O2)   # specify some compilation flags

