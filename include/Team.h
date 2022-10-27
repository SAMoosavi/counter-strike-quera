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
using std::string;
using std::vector;

class Team {
public:
    inline explicit Team(GlobalVariable::access_level accessLevel);

    inline void add_player(const string &name, const Time &time);

    inline bool has_player(const string &name) const;

    inline Player *get_player(const string &name) const;

    inline void new_round();

    inline vector<Player *> get_score_board() const;

    inline bool has_live() const;

    inline void won() const;

    inline void lose() const;

    inline int get_live_num() const;

    inline ~Team();

private:
    map<string, Player *> players;
    const GlobalVariable::access_level ACCESS_LEVEL;
};

#include "../src/Team.h"

#endif // COUNTER_STRIKE_QUERA_TEAM_H
