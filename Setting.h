//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_SETTING_H
#define COUNTER_STRIKE_QUERA_SETTING_H

#include "include/Time.h"
#include "include/Gun.h"
#include "include/Guns.h"

class Setting {
public:
    static auto get_start_money() { return Setting::START_MONEY; }

    static auto get_max_money() { return Setting::MAX_MONEY; }

    static auto get_won_money() { return Setting::WON_MONEY; }

    static auto get_lose_money() { return Setting::LOSE_MONEY; }

    static auto get_start_time() { return Setting::START_TIME; }

    static auto get_end_time() { return Setting::END_TIME; }

    static auto get_start_gun() { return Setting::START_GUN; }

private:
    static const int START_MONEY = 1000;
    static const int MAX_MONEY = 10000;
    static const int WON_MONEY = 2700;
    static const int LOSE_MONEY = 2400;
    static const Time START_TIME;
    static const Time END_TIME;
    static const Gun *START_GUN;

    Setting() = default;
};

const Time Setting::START_TIME = Time("00:00:000");
const Time Setting::END_TIME = Time("02:15:000");
const Gun *Setting::START_GUN = Guns::get_gun("knife", GlobalVariable::access_level::setting);

#endif //COUNTER_STRIKE_QUERA_SETTING_H
