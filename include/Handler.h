//
// Created by moosavi on 10/20/22.
//

#ifndef COUNTER_STRIKE_QUERA_HANDLER_H
#define COUNTER_STRIKE_QUERA_HANDLER_H

#include <string>
#include "Time.h"
#include "Team.h"

using std::string;

class Handler {
public:
    inline string add_user(const string &username, GlobalVariable::team team, const string &time);

    inline int get_money(const string &username) const;

    inline int get_health(const string &username) const;

    inline string tap(const string &attacker, const string &attacked, GlobalVariable::type_gun type) const;

    inline string buy(const string &username, const string &gunName, const string &time) const;

    inline string score_board() const;

    inline string new_round();

    inline ~Handler();

private:
    int round = 1;
    Team *terrorist_class = new Team(GlobalVariable::access_level::terrorist);
    Team *counter_terrorist_class = new Team(GlobalVariable::access_level::counter_terrorist);

    inline Player *find_player(const string &name) const;

    inline bool has_player(const string &name) const;

    inline static string time_score_board_to_string(vector<Player *> team_score_board);
};

#include "../src/Handler.h"

#endif // COUNTER_STRIKE_QUERA_HANDLER_H
