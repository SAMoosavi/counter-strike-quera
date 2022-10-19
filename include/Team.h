//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_TEAM_H
#define COUNTER_STRIKE_QUERA_TEAM_H

#include <string>
#include <map>
#include "Gun.h"
#include "Player.h"

using std::map;
using std::string;

class Team {
public:
    void add_player(string name);

protected:
    virtual void create_gun() = 0;

    map<string,Player *> players;
    map<string, Gun *> guns;
    int life = 0;
};

#endif //COUNTER_STRIKE_QUERA_TEAM_H
