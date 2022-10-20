//
// Created by moosavi on 10/19/22.
//

#include "gtest/gtest.h"
#include "gmock/gmock.h"
#include "../include/Player.h"

#include "../include/Guns.h"
#include "../include/Time.h"

using testing::Eq;
using testing::IsTrue;
using testing::IsFalse;

class PlayerTest : public ::testing::Test {
protected:
    void SetUp() override {
        this->player = new Player("aa", TIME);
    }

    Player *player{};
    Time TIME = Time("00:10:000");
    const GlobalVariable::access_level ACCESS_LEVEL = GlobalVariable::access_level::terrorist;
};

TEST_F(PlayerTest, GetTime) {
    EXPECT_THAT(this->player->get_time(), Eq(this->TIME));
}

TEST_F(PlayerTest, ByeGunNotEnoughMoney) {
    EXPECT_ANY_THROW(this->player->bye_gun(Guns::get_gun("AK", this->ACCESS_LEVEL)));
}

TEST_F(PlayerTest, ByeGuns) {
    this->player->won();
    this->player->won();
    EXPECT_NO_THROW(this->player->bye_gun(Guns::get_gun("Revolver", this->ACCESS_LEVEL)));
    EXPECT_NO_THROW(this->player->bye_gun(Guns::get_gun("AWP", this->ACCESS_LEVEL)));
}

TEST_F(PlayerTest, ByeGunHaveAType) {
    this->player->bye_gun(Guns::get_gun("Revolver", this->ACCESS_LEVEL));
    EXPECT_ANY_THROW(this->player->bye_gun(Guns::get_gun("Glock-18", this->ACCESS_LEVEL)));
}

TEST_F(PlayerTest, GetMoneyStart) {
    EXPECT_THAT(this->player->get_money(), Eq(Setting::START_MONEY));
}

TEST_F(PlayerTest, GetMoneyAfterBye) {
    this->player->bye_gun(Guns::get_gun("Revolver", this->ACCESS_LEVEL));
    EXPECT_THAT(this->player->get_money(), Eq(Setting::START_MONEY - 600));
}

TEST_F(PlayerTest, AddKills) {
    this->player->add_kill(1000);
    EXPECT_THAT(this->player->get_money(), Eq(2000));
    EXPECT_THAT(this->player->get_kills(), Eq(1));
}

TEST_F(PlayerTest, AddMoneyAfterMaxMoney) {
    this->player->add_kill(Setting::MAX_MONEY);
    EXPECT_THAT(this->player->get_money(), Eq(Setting::MAX_MONEY));
}

TEST_F(PlayerTest, Wod) {
    this->player->won();
    EXPECT_THAT(this->player->get_money(), Eq(Setting::START_MONEY + Setting::WON_MONEY));
}

TEST_F(PlayerTest, Lose) {
    this->player->lose();
    EXPECT_THAT(this->player->get_money(), Eq(Setting::START_MONEY + Setting::LOSE_MONEY));
}

TEST_F(PlayerTest, Shut) {
    EXPECT_THAT(this->player->shut(50), IsFalse());
    EXPECT_THAT(this->player->is_live(), IsTrue());
    EXPECT_THAT(this->player->get_killed(), Eq(0));
    EXPECT_THAT(this->player->get_health(), Eq(50));
    EXPECT_THAT(this->player->shut(50), IsTrue());
    EXPECT_THAT(this->player->is_live(), IsFalse());
    EXPECT_THAT(this->player->get_killed(), Eq(1));
    EXPECT_THAT(this->player->get_health(), Eq(0));

    EXPECT_ANY_THROW(this->player->shut(10));
    EXPECT_THAT(this->player->is_live(), IsFalse());
    EXPECT_THAT(this->player->get_killed(), Eq(1));
    EXPECT_THAT(this->player->get_health(), Eq(0));
}

TEST_F(PlayerTest,Reset) {
    this->player->shut(50);
    EXPECT_NO_THROW(this->player->bye_gun(Guns::get_gun("Revolver", this->ACCESS_LEVEL)));
    this->player->reset();
    EXPECT_THAT(this->player->get_health(), Eq(100));
    EXPECT_THAT(this->player->get_money(), Eq(Setting::START_MONEY - 600));
    EXPECT_ANY_THROW(this->player->bye_gun(Guns::get_gun("Revolver", this->ACCESS_LEVEL)));
}

