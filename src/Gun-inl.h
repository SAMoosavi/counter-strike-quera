#ifndef COUNTER_STRIKE_QUERA_GUN_INL_H
#define COUNTER_STRIKE_QUERA_GUN_INL_H

#include "Gun.h"
#include "Error.h"

Gun::Gun(std::string name, int price, int health, int money, type_gun type,
         access_level accessLevel)
: NAME(std::move(name))
, PRICE(price)
, HEALTH(health)
, MONEY(money)
, TYPE(type)
, ACCESS_LEVEL(accessLevel)
{}

int Gun::get_price() const {
    return PRICE;
}

auto Gun::get_type() const -> type_gun {
    return TYPE;
}

auto Gun::get_access_level() const -> access_level {
    return ACCESS_LEVEL;
}

int Gun::get_money() const {
    return MONEY;
}

int Gun::get_health() const {
    return HEALTH;
}

string Gun::get_name() const {
    return NAME;
}

auto Gun::string_to_team_enum(const std::string &team_str) -> type_team {
    if (team_str == "Counter-Terrorist")
        return type_team::Counter_Terrorist;
    else if (team_str == "Terrorist")
        return type_team::Terrorist;
    else
        throw Error("Invalid type_team named " + team_str);
}

auto Gun::string_to_type_gun_enum(const string &gunType) -> type_gun {
    if (gunType == "heavy")
        return type_gun::heavy;
    else if (gunType == "pistol")
        return type_gun::pistol;
    else if (gunType == "knife")
        return type_gun::knife;
    else
        throw Error("Invalid gun type " + gunType);
}

string Gun::team_enum_to_string(type_team team) {
    switch (team) {
        case type_team::Counter_Terrorist:
            return "Counter-Terrorist";
        case type_team::Terrorist:
            return "Terrorist";
        default:
            throw Error("Invalid type_team enum ");
    }
}

string Gun::type_gun_enum_to_string(type_gun gunType) {
    switch (gunType) {
        case type_gun::heavy:
            return "heavy";
        case type_gun::pistol:
            return "pistol";
        case type_gun::knife:
            return "knife";
        default:
            throw Error("Invalid gun type enum");
    }
}

#endif
