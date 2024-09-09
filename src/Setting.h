#ifndef COUNTER_STRIKE_QUERA_SETTING_H
#define COUNTER_STRIKE_QUERA_SETTING_H

#include "Gun.h"
#include "Guns.h"
#include <string>

using std::string;

class Setting
{
public:
    static int get_start_money();

    static int get_max_money();

    static int get_won_money();

    static int get_lose_money();

    static string get_end_time();

    static string get_time_buy_gun();

    static string get_time_add_player();

    static const GunPointer & get_start_gun();

    static int get_max_member_team();

private:
    static const int START_MONEY = 1000;
    static const int MAX_MONEY = 10000;
    static const int WON_MONEY = 2700;
    static const int LOSE_MONEY = 2400;
    static const int MAX_MEMBER_TEAM = 10;
    static const string END_TIME;
    static const string TIME_BUY_GUN;
    static const string TIME_ADD_PLAYER;
    static const GunPointer & START_GUN;

    Setting() = default;
};

#endif
