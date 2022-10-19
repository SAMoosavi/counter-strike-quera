//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_TEAM_H
#define COUNTER_STRIKE_QUERA_TEAM_H

#include <vector>
#include "Gun.h"
#include "Player.h"

using std::vector;

class Team {
public:
protected:
    vector<Player *> players;
    vector<Gun *> guns;
    int life = 0;
};

#endif //COUNTER_STRIKE_QUERA_TEAM_H
