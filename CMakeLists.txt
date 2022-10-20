cmake_minimum_required(VERSION 3.23)
project(Counter_Strike_Quera)

set(CMAKE_CXX_STANDARD 14)

set(SOURCES
        main.cpp
        Setting.h
        GlobalVariable.h
        src/Gun.h
        src/Player.h
        src/Team.h
        src/Terrorist.h
        src/CounterTerrorist.h
        src/Guns.h
        src/Time.h
        src/Handler.h
        )

add_executable(Counter_Strike_Quera ${SOURCES})

target_include_directories(Counter_Strike_Quera
        PRIVATE
        ${PROJECT_SOURCE_DIR}/include
        )


add_subdirectory(test)
