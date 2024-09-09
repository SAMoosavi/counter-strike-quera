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
foreach (TEST_FILE ${TEST_FILES})
        get_filename_component(TEST_NAME ${TEST_FILE} NAME_WE)
        add_executable(${TEST_NAME} ${TEST_FILE})
        target_link_libraries(${TEST_NAME} gtest_main gmock_main)
endforeach ()