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
    Gun(string name, int price, int health, int money, GlobalVariable::type_gun type,
        GlobalVariable::access_level accessLevel);

    int get_price() const;

    GlobalVariable::type_gun get_type() const;

    GlobalVariable::access_level get_access_level() const;

    int get_money() const;

    int get_health() const;

    string get_name() const;

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
