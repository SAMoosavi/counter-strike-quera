//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_PLAYER_H
#define COUNTER_STRIKE_QUERA_PLAYER_H

#include <string>
#include "../Setting.h"
#include "Gun.h"

using std::string;

class Player {
public:
    Player(string name) : NAME(name) {}

private:
    const string NAME;
    int health = 100;
    int money = Setting::START_MONEY;
    int kills = 0;
    int killed = 0;
    Gun *guns[3];
};

#endif //COUNTER_STRIKE_QUERA_PLAYER_H
