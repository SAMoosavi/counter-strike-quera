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
    static const int START_MONEY = 1000;
    static const int MAX_MONEY = 10000;
    static const int WON_MONEY = 2700;
    static const int LOSE_MONEY = 2400;
    static const Time START_TIME;
    static const Time END_TIME;
    static const Gun *START_GUN;
private:
    Setting() = default;
};

const Time Setting::START_TIME = Time("00:00:000");
const Time Setting::END_TIME = Time("02:15:000");
const Gun *Setting::START_GUN = Guns::get_gun("knife", GlobalVariable::access_level::setting);

#endif //COUNTER_STRIKE_QUERA_SETTING_H
