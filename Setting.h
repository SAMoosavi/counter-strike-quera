//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_SETTING_H
#define COUNTER_STRIKE_QUERA_SETTING_H


#include "include/Gun.h"
#include "include/Guns.h"
#include <string>

using std::string;

class Setting {
public:
    static auto get_start_money() { return Setting::START_MONEY; }

    static auto get_max_money() { return Setting::MAX_MONEY; }

    static auto get_won_money() { return Setting::WON_MONEY; }

    static auto get_lose_money() { return Setting::LOSE_MONEY; }

    static auto get_start_time() { return Setting::START_TIME; }

    static auto get_end_time() { return Setting::END_TIME; }

    static auto get_time_buy_gun() { return Setting::TIME_BUY_GUN; }

    static auto get_time_add_player() { return Setting::TIME_ADD_PLAYER; }

    static auto get_start_gun() { return Setting::START_GUN; }

    static auto get_max_member_team() { return Setting::MAX_MEMBER_TEAM; }

private:
    static const int START_MONEY = 1000;
    static const int MAX_MONEY = 10000;
    static const int WON_MONEY = 2700;
    static const int LOSE_MONEY = 2400;
    static const int MAX_MEMBER_TEAM = 10;
    static const string START_TIME;
    static const string END_TIME;
    static const string TIME_BUY_GUN;
    static const string TIME_ADD_PLAYER;
    static const Gun *START_GUN;

    Setting() = default;
};

const string Setting::START_TIME = "00:00:000";
const string Setting::END_TIME = "02:15:000";
const string Setting::TIME_BUY_GUN = "00:45:000";
const string Setting::TIME_ADD_PLAYER = "00:03:000";

const Gun *Setting::START_GUN = Guns::get_gun("knife", GlobalVariable::access_level::setting);

#endif //COUNTER_STRIKE_QUERA_SETTING_H
