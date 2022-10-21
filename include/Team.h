//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_TEAM_H
#define COUNTER_STRIKE_QUERA_TEAM_H

#include <string>
#include <map>
#include <vector>
#include "Gun.h"
#include "Player.h"
#include "Guns.h"

using std::map;
using std::vector;
using std::string;

class Team {
public:
    explicit Team(GlobalVariable::access_level accessLevel) : ACCESS_LEVEL(accessLevel) {}

    void add_player(const string &name, const Time &time);

    bool has_player(const string &name) const;

    Player *get_player(const string &name) const;

    void new_round();

    vector<Player *> get_score_board() const;

    bool has_live() const;

    void won() const;

    void lose() const;

    int get_live_num() const;

    ~Team();

private:
    map<string, Player *> players;
    const GlobalVariable::access_level ACCESS_LEVEL;
};

#include "../src/Team.h"

#endif //COUNTER_STRIKE_QUERA_TEAM_H
