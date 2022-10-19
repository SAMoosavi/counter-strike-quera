//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_GUN_H
#define COUNTER_STRIKE_QUERA_GUN_H

#include <string>
#include "../GlobalVariable.h"

using std::string;

class Gun {
public:
    Gun(
            string name,
            int price,
            int health,
            int money,
            GlobalVariable::type_gun type
    ) :
            NAME(name),
            PRICE(price),
            HEALTH(health),
            MONEY(money),
            TYPE(type) {}

private:
    const GlobalVariable::type_gun TYPE;
    const string NAME;
    const int PRICE;
    const int HEALTH;
    const int MONEY;
};

#endif //COUNTER_STRIKE_QUERA_GUN_H
