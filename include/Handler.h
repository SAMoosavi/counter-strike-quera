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
    void add_user(const string &username, GlobalVariable::team team, const string &time);

    int get_money(const string &username) const;

    int get_health(const string &username) const;

    void tap(const string &attacker, const string &attacked, GlobalVariable::type_gun type) const;

    void buy(const string &username, const string &gunName, const string &time) const;

    void score_board() const;

    void new_round();

    ~Handler();

private:
    int round = 0;
    Team *terrorist_class = new Team(GlobalVariable::access_level::terrorist);
    Team *counter_terrorist_class = new Team(GlobalVariable::access_level::counter_terrorist);

    Player *find_player(const string &name) const;

    bool has_player(const string &name) const;
};

#include "../src/Handler.h"

#endif //COUNTER_STRIKE_QUERA_HANDLER_H
