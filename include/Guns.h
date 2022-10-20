//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_GUNS_H
#define COUNTER_STRIKE_QUERA_GUNS_H

#include "Gun.h"
#include <map>

using std::map;

class Guns {
public:
    static Gun * get_gun(string name, GlobalVariable::access_level accessLevel);
private:
    static map<string, Gun *> guns ;
    Guns() = default;
};

#include "../src/Guns.h"

#endif //COUNTER_STRIKE_QUERA_GUNS_H
