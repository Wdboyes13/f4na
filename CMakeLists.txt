cmake_minimum_required(VERSION 3.5)
project(f4na)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED YES)

file(GLOB SRCS CONFIGURE_DEPENDS ${CMAKE_SOURCE_DIR}/src/*.cc)

add_executable(${CMAKE_PROJECT_NAME} ${SRCS})
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/include)