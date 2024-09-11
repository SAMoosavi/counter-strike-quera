#ifndef COUNTER_STRIKE_QUERA_GUNS_INL_H
#define COUNTER_STRIKE_QUERA_GUNS_INL_H

#include "Guns.h"
#include "Error.h"
#include <map>


const GunPointer& Guns::get_gun(const string &name, Gun::access_level accessLevel) {
    const static std::map<string, GunPointer > guns = {
            {"AK",           (std::make_shared<Gun>("AK", 2700, 31, 100, Gun::type_gun::heavy, Gun::access_level::terrorist))},
            {"AWP",          (std::make_shared<Gun>("AWP", 4300, 110, 50, Gun::type_gun::heavy, Gun::access_level::all))},
            {"M4A1",         {std::make_shared<Gun>("M4A1", 2700, 29, 100, Gun::type_gun::heavy, Gun::access_level::counter_terrorist)}},
            {"Desert-Eagle", (std::make_shared<Gun>("Desert-Eagle", 600, 53, 175, Gun::type_gun::pistol, Gun::access_level::counter_terrorist))},
            {"UPS-S",        (std::make_shared<Gun>("UPS-S", 300, 13, 225, Gun::type_gun::pistol, Gun::access_level::counter_terrorist))},
            {"Revolver",     (std::make_shared<Gun>("Revolver", 600, 51, 150, Gun::type_gun::pistol, Gun::access_level::terrorist))},
            {"Glock-18",     (std::make_shared<Gun>("Glock-18", 300, 11, 200, Gun::type_gun::pistol, Gun::access_level::terrorist))},
            {"knife",        (std::make_shared<Gun>("knife", 0, 43, 500, Gun::type_gun::knife, Gun::access_level::all))}
    };

    if (accessLevel == Gun::access_level::all)
        throw Error("How are you?");
    else if (!guns.count(name))
        throw Error("Gun don't exist");
    else if (accessLevel == Gun::access_level::setting
            || guns.at(name)->get_access_level() == Gun::access_level::all
            || guns.at(name)->get_access_level() == accessLevel)
        return guns.at(name);
    else
        throw Error("invalid category Gun");
}
#endif

