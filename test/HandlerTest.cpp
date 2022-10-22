//
// Created by moosavi on 10/21/22.
//

#include "../include/Handler.h"
#include <gtest/gtest.h>
#include <gmock/gmock.h>

using std::to_string;
using testing::Eq;

string to_string_three_char(int num) {
    string numStr = to_string(num);
    for (int i = 0; i < 4 - numStr.size(); i++)
        numStr = string("0" + numStr);
    return numStr;
}

class HandlerTest : public testing::Test {
protected:
    void SetUp() override {
        this->handler = new Handler();
        for (int i = 1; i < Setting::get_max_member_team(); ++i) {
            handler->add_user("T" + to_string(i), GlobalVariable::team::Terrorist,
                              "00:00:" + to_string_three_char(i));

            handler->add_user("CT" + to_string(i), GlobalVariable::team::Counter_Terrorist,
                              "00:00:" + to_string_three_char(i));
        }
    }

    void TearDown() override {
        delete this->handler;
    }

    Handler *handler{};
};

TEST_F(HandlerTest, AddUser) {
    ASSERT_NO_THROW(handler->add_user("T0", GlobalVariable::team::Terrorist, "00:00:001"));

    ASSERT_THAT(handler->add_user("CT0", GlobalVariable::team::Counter_Terrorist, "00:00:001"),
                Eq("this user added to Counter-Terrorist"));

    ASSERT_ANY_THROW(handler->add_user("T0", GlobalVariable::team::Counter_Terrorist, "00:00:001"));
    ASSERT_ANY_THROW(handler->add_user("T" + std::to_string(Setting::get_max_member_team() + 1),
                                       GlobalVariable::team::Terrorist, "00:00:001"));
}

TEST_F(HandlerTest, GetMoney) {
    ASSERT_THAT(this->handler->get_money("T1"), Eq(Setting::get_start_money()));
}

TEST_F(HandlerTest, GetHealth) {
    ASSERT_THAT(this->handler->get_health("T1"), Eq(100));
}

TEST_F(HandlerTest, Tap) {
    EXPECT_NO_THROW(this->handler->tap("T1", "CT1", GlobalVariable::type_gun::knife));
    EXPECT_THAT(this->handler->tap("T1", "CT2", GlobalVariable::type_gun::knife), Eq("nice shot"));
    EXPECT_ANY_THROW(this->handler->tap("T1", "CT1", GlobalVariable::type_gun::heavy));
    EXPECT_ANY_THROW(this->handler->tap("T1", "T2", GlobalVariable::type_gun::knife));
    EXPECT_ANY_THROW(this->handler->tap("CT0", "CT1", GlobalVariable::type_gun::knife));
    EXPECT_ANY_THROW(this->handler->tap("T", "CT1", GlobalVariable::type_gun::knife));

    for (int i = 0; i < 100 / Setting::get_start_gun()->get_health(); ++i)
        this->handler->tap("T1", "CT1", GlobalVariable::type_gun::knife);

    EXPECT_ANY_THROW(this->handler->tap("CT1", "T1", GlobalVariable::type_gun::knife));
}

TEST_F(HandlerTest, Buy) {
    EXPECT_NO_THROW(this->handler->buy("T1", "Revolver", "00:01:000"));
    EXPECT_THAT(this->handler->buy("T2", "Revolver", "00:01:000"), Eq("I hope you can use it"));
    EXPECT_ANY_THROW(this->handler->buy("T1", "AK", "00:01:000"));
    EXPECT_ANY_THROW(this->handler->buy("T1", "UPS-S", "00:01:000"));
    EXPECT_ANY_THROW(this->handler->buy("T1", "Revolver", "00:50:000"));

    for (int i = 0; i <= 100 / Guns::get_gun("Revolver", GlobalVariable::access_level::terrorist)->get_health(); ++i)
        this->handler->tap("T1", "CT1", GlobalVariable::type_gun::pistol);

    EXPECT_ANY_THROW(this->handler->buy("CT1", "UPS-S", "00:00:001"));
}

TEST_F(HandlerTest, NewRound) {
    ASSERT_THAT(this->handler->new_round(), Eq("Counter-Terrorist won"));
    ASSERT_THAT(this->handler->get_money("T2"), Eq(Setting::get_start_money() + Setting::get_lose_money()));
    ASSERT_THAT(this->handler->get_money("CT2"), Eq(Setting::get_start_money() + Setting::get_won_money()));
    for (int i = 1; i < Setting::get_max_member_team(); ++i)
        for (int j = 0; j <= 100 / Setting::get_start_gun()->get_health(); ++j)
            this->handler->tap("T1", "CT" + to_string(i), GlobalVariable::type_gun::knife);

    ASSERT_THAT(this->handler->new_round(), Eq("Terrorist won"));
    ASSERT_THAT(this->handler->get_money("T2"),
                Eq(Setting::get_start_money() + Setting::get_lose_money() + Setting::get_won_money()));
    ASSERT_THAT(this->handler->get_money("CT2"),
                Eq(Setting::get_start_money() + Setting::get_won_money() + Setting::get_lose_money()));
}


