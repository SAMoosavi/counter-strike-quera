#include "Setting.h"

const string Setting::END_TIME = "02:15:000";
const string Setting::TIME_BUY_GUN = "00:45:000";
const string Setting::TIME_ADD_PLAYER = "00:03:000";

const GunPointer & Setting::START_GUN = Guns::get_gun("knife", Gun::access_level::setting);

int Setting::get_start_money()
{
    return Setting::START_MONEY;
}

int Setting::get_max_money()
{
    return Setting::MAX_MONEY;
}

int Setting::get_won_money()
{
    return Setting::WON_MONEY;
}

int Setting::get_lose_money()
{
    return Setting::LOSE_MONEY;
}

string Setting::get_end_time()
{
    return Setting::END_TIME;
}

string Setting::get_time_buy_gun()
{
    return Setting::TIME_BUY_GUN;
}

string Setting::get_time_add_player()
{
    return Setting::TIME_ADD_PLAYER;
}

const GunPointer & Setting::get_start_gun()
{
    return Setting::START_GUN;
}

int Setting::get_max_member_team()
{
    return Setting::MAX_MEMBER_TEAM;
}
