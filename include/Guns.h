//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_GUNS_H
#define COUNTER_STRIKE_QUERA_GUNS_H

#include "Gun.h"
#include <map>

using std::map;

class Guns {
private:
    map<string, Gun *> guns = {
            {"AK",           (new Gun("AK", 2700, 31, 100, GlobalVariable::type_gun::heavy, GlobalVariable::access_level::terrorist))},
            {"AWP",          (new Gun("AWP", 4300, 110, 50, GlobalVariable::type_gun::heavy, GlobalVariable::access_level::all))},
            {"Desert-Eagle", (new Gun("Desert-Eagle", 600, 53, 175, GlobalVariable::type_gun::pistol, GlobalVariable::access_level::counter_terrorist))},
            {"UPS-S",        (new Gun("UPS-S", 300, 13, 225, GlobalVariable::type_gun::pistol, GlobalVariable::access_level::counter_terrorist))},
            {"Revolver",     (new Gun("Revolver", 600, 51, 150, GlobalVariable::type_gun::pistol, GlobalVariable::access_level::terrorist))},
            {"Glock-18",     (new Gun("Glock-18", 300, 11, 200, GlobalVariable::type_gun::pistol, GlobalVariable::access_level::terrorist))},
            {"knife",        (new Gun("knife", 0, 43, 500, GlobalVariable::type_gun::knife, GlobalVariable::access_level::all))}
    };
};

#endif //COUNTER_STRIKE_QUERA_GUNS_H
