//
// Created by moosavi on 10/19/22.
//

#include "gtest/gtest.h"
#include "gmock/gmock.h"
#include "../include/Gun.h"

using ::testing::Eq;

class GunTest : public ::testing::Test {
protected:
    void SetUp() override {
        this->gun = new Gun("AK", 2700, 31, 100, GlobalVariable::type_gun::heavy,
                            GlobalVariable::access_level::terrorist);
    }

    Gun *gun{};
};

TEST_F(GunTest,GetPrice){
    EXPECT_THAT(this->gun->get_price(),Eq(2700));
}

TEST_F(GunTest,GetHealth){
    EXPECT_THAT(this->gun->get_health(),Eq(31));
}

TEST_F(GunTest,GetMoney){
    EXPECT_THAT(this->gun->get_money(),Eq(100));
}

TEST_F(GunTest,GetType){
    EXPECT_THAT(this->gun->get_type(),Eq(GlobalVariable::type_gun::heavy));
}

TEST_F(GunTest, GetAccessLevel){
    EXPECT_THAT(this->gun->get_access_level(),Eq(GlobalVariable::access_level::terrorist));
}