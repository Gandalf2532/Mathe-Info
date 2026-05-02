cmake_minimum_required(VERSION 3.10)

project(RSA C)

set(CMAKE_C_STANDARD 17)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${PROJECT_SOURCE_DIR}/bin")

set(SOURCE_FILES
    src/main.c
    src/randomizer/randomizer.c
    src/bigint/bigint.c
)

add_executable(RSA ${SOURCE_FILES})