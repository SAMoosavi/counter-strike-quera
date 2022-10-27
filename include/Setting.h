//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_SETTING_H
#define COUNTER_STRIKE_QUERA_SETTING_H

#include "Gun.h"
#include "Guns.h"
#include <string>

using std::string;

class Setting {
public:
    inline static int get_start_money();

    inline static int get_max_money();

    inline static int get_won_money();

    inline static int get_lose_money();

    inline static string get_end_time();

    inline static string get_time_buy_gun();

    inline static string get_time_add_player();

    inline static const Gun *get_start_gun();

    inline static int get_max_member_team();

private:
    static const int START_MONEY = 1000;
    static const int MAX_MONEY = 10000;
    static const int WON_MONEY = 2700;
    static const int LOSE_MONEY = 2400;
    static const int MAX_MEMBER_TEAM = 10;
    static const string END_TIME;
    static const string TIME_BUY_GUN;
    static const string TIME_ADD_PLAYER;
    static const Gun *START_GUN;

    Setting() = default;
};

const string Setting::END_TIME = "02:15:000";
const string Setting::TIME_BUY_GUN = "00:45:000";
const string Setting::TIME_ADD_PLAYER = "00:03:000";

const Gun *Setting::START_GUN = Guns::get_gun("knife", GlobalVariable::access_level::setting);

#include "../src/Setting.h"

#endif // COUNTER_STRIKE_QUERA_SETTING_H
