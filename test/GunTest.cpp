//
// Created by moosavi on 10/19/22.
//

#include "gtest/gtest.h"
#include "gmock/gmock.h"
#include "../src/Gun.h"

using ::testing::Eq;

class GunTest : public ::testing::Test {
protected:
    void SetUp() override {
        gun = new Gun("AK", 2700, 31, 100, Gun::type_gun::heavy,
                      Gun::access_level::terrorist);
    }

    GunPointer  gun{};
};

TEST_F(GunTest, GetPrice) {
    EXPECT_THAT(gun->get_price(), Eq(2700));
}

TEST_F(GunTest, GetHealth) {
    EXPECT_THAT(gun->get_health(), Eq(31));
}

TEST_F(GunTest, GetMoney) {
    EXPECT_THAT(gun->get_money(), Eq(100));
}

TEST_F(GunTest, GetType) {
    EXPECT_THAT(gun->get_type(), Eq(Gun::type_gun::heavy));
}

TEST_F(GunTest, GetAccessLevel) {
    EXPECT_THAT(gun->get_access_level(), Eq(Gun::access_level::terrorist));
}

TEST_F(GunTest, GetName) {
    EXPECT_THAT(gun->get_name(), Eq("AK"));
}