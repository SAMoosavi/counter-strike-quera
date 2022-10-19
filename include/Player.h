//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_PLAYER_H
#define COUNTER_STRIKE_QUERA_PLAYER_H

#include <string>
#include <vector>
#include <utility>
#include "../Setting.h"
#include "Gun.h"

using std::string;
using std::vector;

class Player {
public:
    explicit Player(string name, Gun *knife) : NAME(std::move(name)), KNIFE(knife) {}

private:
    const string NAME;
    int health = 100;
    int money = Setting::START_MONEY;
    int kills = 0;
    int killed = 0;
    vector<Gun *> guns;
    const Gun *KNIFE;
};

#endif //COUNTER_STRIKE_QUERA_PLAYER_H
