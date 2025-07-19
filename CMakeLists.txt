cmake_minimum_required(VERSION 3.21)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_VISIBILITY_PRESET hidden)

project(ServerShared VERSION 1.0.0)

add_library(${PROJECT_NAME} INTERFACE
    schema/generated/game.capnp.cpp
    schema/generated/main.capnp.cpp
    schema/generated/shared.capnp.cpp
)

target_include_directories(${PROJECT_NAME} INTERFACE schema/generated)
