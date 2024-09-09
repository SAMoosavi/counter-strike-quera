#include "gtest/gtest.h"
#include "gmock/gmock.h"
#include "../src/Player.h"

using testing::IsNull;
using testing::Eq;

class PlayerTest : public ::testing::Test {
protected:
    void SetUp() override {
        this->player = new Player("aa", TIME, this->ACCESS_LEVEL);
    }

    Player *player{};
    Time TIME = Time("00:01:000");
    const Gun::access_level ACCESS_LEVEL = Gun::access_level::terrorist;
};

TEST_F(PlayerTest, ByeGunNotEnoughMoney) {
    EXPECT_ANY_THROW(this->player->buy_gun("AK"));
}

TEST_F(PlayerTest, ByeGuns) {
    this->player->won();
    this->player->won();
    EXPECT_NO_THROW(this->player->buy_gun("Revolver"));
    EXPECT_NO_THROW(this->player->buy_gun("AWP"));
}

TEST_F(PlayerTest, ByeGunHaveAType) {
    this->player->buy_gun("Revolver");
    EXPECT_ANY_THROW(this->player->buy_gun("Glock-18"));
}

TEST_F(PlayerTest, GetMoneyStart) {
    EXPECT_THAT(this->player->get_money(), Eq(Setting::get_start_money()));
}

TEST_F(PlayerTest, GetMoneyAfterBye) {
    this->player->buy_gun("Revolver");
    EXPECT_THAT(this->player->get_money(), Eq(Setting::get_start_money() - 600));
}

TEST_F(PlayerTest, AddKills) {
    this->player->add_kill(Gun::type_gun::knife);
    EXPECT_THAT(this->player->get_money(), Eq(1500));
    EXPECT_THAT(this->player->get_kills(), Eq(1));
}

TEST_F(PlayerTest, AddMoneyAfterMaxMoney) {
    for (int i = 0; i < Setting::get_max_money() / Guns::get_gun("knife", this->ACCESS_LEVEL)->get_money() + 1; ++i)
        this->player->add_kill(Gun::type_gun::knife);
    EXPECT_THAT(this->player->get_money(), Eq(Setting::get_max_money()));
}

TEST_F(PlayerTest, Wod) {
    this->player->won();
    EXPECT_THAT(this->player->get_money(), Eq(Setting::get_start_money() + Setting::get_won_money()));
}

TEST_F(PlayerTest, Lose) {
    this->player->lose();
    EXPECT_THAT(this->player->get_money(), Eq(Setting::get_start_money() + Setting::get_lose_money()));
}

TEST_F(PlayerTest, Shut) {
    EXPECT_FALSE(this->player->shut(50));
    EXPECT_TRUE(this->player->is_live());
    EXPECT_THAT(this->player->get_killed(), Eq(0));
    EXPECT_THAT(this->player->get_health(), Eq(50));
    EXPECT_TRUE(this->player->shut(50));
    EXPECT_FALSE(this->player->is_live());
    EXPECT_THAT(this->player->get_killed(), Eq(1));
    EXPECT_THAT(this->player->get_health(), Eq(0));

    EXPECT_ANY_THROW(this->player->shut(10));
    EXPECT_FALSE(this->player->is_live());
    EXPECT_THAT(this->player->get_killed(), Eq(1));
    EXPECT_THAT(this->player->get_health(), Eq(0));
}

TEST_F(PlayerTest, Reset) {
    this->player->shut(50);
    EXPECT_NO_THROW(this->player->buy_gun("Revolver"));
    this->player->reset();
    EXPECT_THAT(this->player->get_health(), Eq(100));
    EXPECT_THAT(this->player->get_money(), Eq(Setting::get_start_money() - 600));
    EXPECT_ANY_THROW(this->player->buy_gun("Revolver"));
}

TEST_F(PlayerTest, HasGun) {
    EXPECT_TRUE(this->player->has_gun(Gun::type_gun::knife));
    EXPECT_FALSE(this->player->has_gun(Gun::type_gun::heavy));
    for (int i = 0; i < Setting::get_max_money() / Guns::get_gun("knife", this->ACCESS_LEVEL)->get_money() + 1; ++i)
        this->player->add_kill(Gun::type_gun::knife);
    this->player->buy_gun("AWP");
    EXPECT_TRUE(this->player->has_gun(Gun::type_gun::heavy));
    this->player->shut(110);
    EXPECT_FALSE(this->player->has_gun(Gun::type_gun::heavy));
    EXPECT_FALSE(this->player->has_gun(Gun::type_gun::knife));
    this->player->reset();
}

TEST_F(PlayerTest, GetGun) {
    this->player->buy_gun("Revolver");
    EXPECT_THAT(this->player->get_gun(Gun::type_gun::pistol)->get_name(), Eq("Revolver"));
    EXPECT_THAT(this->player->get_gun(Gun::type_gun::heavy), IsNull());
}

