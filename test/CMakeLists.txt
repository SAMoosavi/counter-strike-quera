# 'Google_test' is the subproject name
project(Google_tests)

# 'lib' is the folder with Google Test sources
add_subdirectory(lib/googletest-main)
include_directories(${gtest_SOURCE_DIR}/include ${gtest_SOURCE_DIR})
include_directories(${gmock_SOURCE_DIR}/include ${gmock_SOURCE_DIR})

set(TEST_FILES
        GunTest.cpp
        GunsTest.cpp
        PlayerTest.cpp
        TimeTest.cpp
        TeamTest.cpp
        HandlerTest.cpp
        )

# 'Google_Tests_run' is the target name
add_executable(Google_Tests_run ${TEST_FILES})
target_link_libraries(Google_Tests_run gtest gtest_main)