#include "gtest/gtest.h"
#include "gmock/gmock.h"
#include "../src/Guns.h"

using testing::NotNull;

TEST(GunsTest, Accessable) {
    EXPECT_THAT(Guns::get_gun("AK", Gun::access_level::terrorist), NotNull());
}

TEST(GunsTest, NotAccessable) {
    EXPECT_ANY_THROW(Guns::get_gun("AK", Gun::access_level::counter_terrorist));
}

TEST(GunsTest, NotExit) {
    EXPECT_ANY_THROW(Guns::get_gun("Ak", Gun::access_level::terrorist));
}

TEST(GunsTest, AccessAll) {
    EXPECT_THAT(Guns::get_gun("knife", Gun::access_level::terrorist), NotNull());
    EXPECT_THAT(Guns::get_gun("knife", Gun::access_level::counter_terrorist), NotNull());
}

TEST(GunsTest, AccessParamIsAll) {
    EXPECT_ANY_THROW(Guns::get_gun("AK", Gun::access_level::all));
}