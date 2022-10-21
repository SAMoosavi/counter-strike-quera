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
    for (int i = 0; i < 3 - numStr.size(); i++)
        numStr = string("0" + numStr);
    return numStr;
}

class HandlerTest : public testing::Test {
protected:
    void SetUp() override {
        this->handler = new Handler();
    }

    void TearDown() override {
        delete this->handler;
    }

    void add_user() {
        for (int i = 0; i <= Setting::get_max_member_team(); ++i) {
            handler->add_user("T" + to_string(i), GlobalVariable::team::Terrorist,
                              "00:00:" + to_string_three_char(i));

            handler->add_user("CT" + to_string(i), GlobalVariable::team::Counter_Terrorist,
                              "00:00:" + to_string_three_char(i));
        }
    }

    Handler *handler{};
};

TEST_F(HandlerTest, AddUser) {
    ASSERT_NO_THROW(handler->add_user("0", GlobalVariable::team::Terrorist, "00:00:001"));
    ASSERT_ANY_THROW(handler->add_user("0", GlobalVariable::team::Counter_Terrorist, "00:00:001"));
    for (int i = 1; i <= Setting::get_max_member_team(); ++i)
        ASSERT_NO_THROW(handler->add_user(std::to_string(i), GlobalVariable::team::Counter_Terrorist, "00:00:001"));
    ASSERT_ANY_THROW(handler->add_user(std::to_string(Setting::get_max_member_team() + 1),
                                       GlobalVariable::team::Counter_Terrorist, "00:00:001"));
}

TEST_F(HandlerTest, GetMoney) {
    this->add_user();
    ASSERT_THAT(this->handler->get_money("T0"), Eq(Setting::get_start_money()));
}

TEST_F(HandlerTest, GetHealth) {
    this->add_user();
    ASSERT_THAT(this->handler->get_health("T0"), Eq(100));
}

TEST_F(HandlerTest, Tap) {
    this->add_user();
    EXPECT_NO_THROW(this->handler->tap("T0", "CT1", GlobalVariable::type_gun::knife));
    EXPECT_ANY_THROW(this->handler->tap("T0", "CT1", GlobalVariable::type_gun::heavy));
    EXPECT_ANY_THROW(this->handler->tap("T0", "T1", GlobalVariable::type_gun::knife));
    EXPECT_ANY_THROW(this->handler->tap("CT0", "CT1", GlobalVariable::type_gun::knife));
    EXPECT_ANY_THROW(this->handler->tap("T", "CT1", GlobalVariable::type_gun::knife));

    for (int i = 0; i < 100 / Setting::get_start_gun()->get_health(); ++i)
        this->handler->tap("T0", "CT1", GlobalVariable::type_gun::knife);

    EXPECT_ANY_THROW(this->handler->tap("CT1", "T1", GlobalVariable::type_gun::knife));
}

TEST_F(HandlerTest, Buy) {
    EXPECT_NO_THROW(this->handler->buy("T0", "Revolver", "00:01:000"));
    EXPECT_ANY_THROW(this->handler->buy("T0", "AK", "00:01:000"));
    EXPECT_ANY_THROW(this->handler->buy("T0", "UPS-S", "00:01:000"));
    EXPECT_ANY_THROW(this->handler->buy("T0", "Revolver", "00:50:000"));

    for (int i = 0; i < 100 / Guns::get_gun("Revolver", GlobalVariable::access_level::terrorist)->get_health(); ++i)
        this->handler->tap("T0", "CT1", GlobalVariable::type_gun::pistol);

    EXPECT_ANY_THROW(this->handler->buy("CT1", "UPS-S", "00:00:001"));
}

TEST_F(HandlerTest, NewRound) {
    this->handler->new_round();
    ASSERT_THAT(this->handler->get_money("T0"),Eq(Setting::get_start_money() + Setting::get_lose_money()));
    ASSERT_THAT(this->handler->get_money("CT0"),Eq(Setting::get_start_money() + Setting::get_won_money()));
}


