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
    inline Gun(string name, int price, int health, int money, GlobalVariable::type_gun type,
               GlobalVariable::access_level accessLevel);

    inline int get_price() const;

    inline GlobalVariable::type_gun get_type() const;

    inline GlobalVariable::access_level get_access_level() const;

    inline int get_money() const;

    inline int get_health() const;

    inline string get_name() const;

private:
    const GlobalVariable::access_level ACCESS_LEVEL;
    const GlobalVariable::type_gun TYPE;
    const string NAME;
    const int PRICE;
    const int HEALTH;
    const int MONEY;
};

#include "../src/Gun.h"

#endif // COUNTER_STRIKE_QUERA_GUN_H
