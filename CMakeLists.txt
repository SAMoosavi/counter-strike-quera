cmake_minimum_required(VERSION 3.23)
project(Counter_Strike_Quera)

set(CMAKE_CXX_STANDARD 14)

set(SOURCES
        main.cpp
        src/Gun.cpp
        GlobalVariable.h
        src/Player.cpp
        Setting.h
        src/Team.cpp
        src/Terrorist.cpp
        src/CounterTerrorist.cpp)

add_executable(Counter_Strike_Quera ${SOURCES})

target_include_directories(Counter_Strike_Quera
        PRIVATE
        ${PROJECT_SOURCE_DIR}/include
        )


add_subdirectory(test)
