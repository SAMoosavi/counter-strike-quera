//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_GUN_H
#define COUNTER_STRIKE_QUERA_GUN_H

#include <string>
#include <utility>
#include "../GlobalVariable.h"

using std::string;

class Gun {
public:
    Gun(
            string name,
            int price,
            int health,
            int money,
            GlobalVariable::type_gun type,
            GlobalVariable::access_level accessLevel
    ) :
            NAME(std::move(name)),
            PRICE(price),
            HEALTH(health),
            MONEY(money),
            TYPE(type),
            ACCESS_LEVEL(accessLevel) {}

    int get_price() const { return this->PRICE; }

    GlobalVariable::type_gun get_type() const { return this->TYPE; }

    GlobalVariable::access_level get_access_level() const { return this->ACCESS_LEVEL; }

    int get_money() const { return this->MONEY; }

    int get_health() const { return this->HEALTH; }

private:
    const GlobalVariable::access_level ACCESS_LEVEL;
    const GlobalVariable::type_gun TYPE;
    const string NAME;
    const int PRICE;
    const int HEALTH;
    const int MONEY;
};

#include "../src/Gun.h"

#endif //COUNTER_STRIKE_QUERA_GUN_H
