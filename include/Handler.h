//
// Created by moosavi on 10/20/22.
//

#ifndef COUNTER_STRIKE_QUERA_HANDLER_H
#define COUNTER_STRIKE_QUERA_HANDLER_H

#include <string>
#include "Time.h"
#include "Terrorist.h"
#include "CounterTerrorist.h"

using std::string;

class Handler {
public:
    void add_user(const string &username, GlobalVariable::team team, const string &time) const;

    void get_money(const string &username) const;

    void get_health(const string &username) const;

private:
    int round = 0;
    Terrorist *terrorist_class = Terrorist::create_class();
    CounterTerrorist *counter_terrorist_class = CounterTerrorist::create_class();

    Player *find_player(const string &name) const;

    bool has_player(const string &name) const;
};

#include "../src/Handler.h"

#endif //COUNTER_STRIKE_QUERA_HANDLER_H
