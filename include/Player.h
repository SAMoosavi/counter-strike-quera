//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_PLAYER_H
#define COUNTER_STRIKE_QUERA_PLAYER_H

#include <string>
#include <utility>
#include <vector>
#include <utility>
#include "../Setting.h"
#include "Gun.h"

using std::string;
using std::vector;

class Player {
public:
    explicit Player(string name, string time) : NAME(std::move(name)), TIME(std::move(time)) {}

    void reset();

    void add_kill(int money);

    bool shut(int health);

    void bye_gun(Gun *gun);

    int get_health() const;

    int get_money() const;

    int get_kills() const;

    int get_killed() const;

    string get_time() const;

    bool is_live() const;

    void won();

    void lose();

private:
    const string NAME;
    int health = 100;
    int money = Setting::START_MONEY;
    int kills = 0;
    int killed = 0;
    vector<Gun *> guns;
    const string TIME;

    void can_bye(Gun *gun) const;

    void add_money(int money);
};

#endif //COUNTER_STRIKE_QUERA_PLAYER_H
