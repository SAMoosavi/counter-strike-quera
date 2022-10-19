cmake_minimum_required(VERSION 3.23)
project(Counter_Strike_Quera)

set(CMAKE_CXX_STANDARD 14)

set(SOURCES
        main.cpp
        src/Gun.cpp)

add_executable(Counter_Strike_Quera ${SOURCES})

target_include_directories(Counter_Strike_Quera
        PRIVATE
        ${PROJECT_SOURCE_DIR}/include
        )


add_subdirectory(test)
