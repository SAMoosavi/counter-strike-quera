//
// Created by moosavi on 10/21/22.
//

#include "../include/Team.h"
#include <gtest/gtest.h>
#include <gmock/gmock.h>

using std::to_string;
using std::string;
using testing::NotNull;
using testing::Eq;

class TeamTest : public testing::Test {
protected:
    void SetUp() override {
        this->team = new Team(GlobalVariable::access_level::terrorist);
    }

    void TearDown() override {
        delete this->team;
    }

    Team *team{};
};


TEST_F(TeamTest, AddPlayer) {
    ASSERT_NO_THROW(this->team->add_player("0", Time("00:01:000")));
    int muxMemberTeam = Setting::get_max_member_team();
    for (int i = 1; i < muxMemberTeam; ++i)
        ASSERT_NO_THROW(this->team->add_player(to_string(i), Time("00:00:001") * i));
    ASSERT_ANY_THROW(this->team->add_player(to_string(muxMemberTeam), Time("00:00:001") * muxMemberTeam));
}

TEST_F(TeamTest, HasPlayer) {
    int muxMemberTeam = Setting::get_max_member_team();
    for (int i = 0; i < muxMemberTeam - 2; ++i)
        this->team->add_player(to_string(i), Time("00:00:001") * i);
    ASSERT_TRUE(this->team->has_player("0"));
    ASSERT_FALSE(this->team->has_player("-1"));
}

TEST_F(TeamTest, GetPlayer) {
    this->team->add_player("0", Time("00:00:001"));
    ASSERT_THAT(this->team->get_player("0"), NotNull());
    ASSERT_ANY_THROW(this->team->get_player("-1"));
}

TEST_F(TeamTest, HasLive) {
    this->team->add_player("0", Time("00:00:001"));
    ASSERT_TRUE(this->team->has_live());
    this->team->get_player("0")->shut(100);
    ASSERT_FALSE(this->team->has_live());
    this->team->add_player("1", Time(Setting::get_time_add_player()) + Time("00:00:001"));
    ASSERT_FALSE(this->team->has_live());
}

TEST_F(TeamTest, Won) {
    this->team->add_player("0", Time("00:00:001"));
    this->team->won();
    EXPECT_THAT(this->team->get_player("0")->get_money(), Eq(Setting::get_start_money() + Setting::get_won_money()));
}

TEST_F(TeamTest, Lose) {
    this->team->add_player("0", Time("00:00:001"));
    this->team->lose();
    EXPECT_THAT(this->team->get_player("0")->get_money(), Eq(Setting::get_start_money() + Setting::get_lose_money()));
}

TEST_F(TeamTest, GetLiveNum) {
    ASSERT_THAT(this->team->get_live_num(), Eq(0));
    this->team->add_player("0", Time("00:00:001"));
    ASSERT_THAT(this->team->get_live_num(), Eq(1));
    this->team->add_player("1", Time(Setting::get_time_add_player()) + Time("00:00:001"));
    ASSERT_THAT(this->team->get_live_num(), Eq(1));
}
