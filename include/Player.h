//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_PLAYER_H
#define COUNTER_STRIKE_QUERA_PLAYER_H

#include <string>
#include <map>
#include <iostream>
#include "../Setting.h"
#include "Time.h"
#include "Gun.h"

using std::string;
using std::map;
using std::ostream;

class Player {
public:
    explicit Player(string name, const Time &time, GlobalVariable::access_level accessLevel) :
            NAME(std::move(name)),
            TIME(time),
            ACCESS_LEVEL(accessLevel) { this->guns[Setting::get_start_gun()->get_type()] = Setting::get_start_gun(); }

    void reset();

    void add_kill(GlobalVariable::type_gun type);

    bool shut(int health);

    void bye_gun(const string &name);

    int get_health() const;

    int get_money() const;

    int get_kills() const;

    int get_killed() const;

    Time get_time() const;

    bool is_live() const;

    void won();

    void lose();

    bool has_gun(GlobalVariable::type_gun type) const;

    Gun *get_gun(GlobalVariable::type_gun type) const;

    friend ostream &operator<<(ostream &output, const Player &player);

    string to_string() const;

    bool operator<(const Player &other) const;

    bool operator>(const Player &other) const;

private:
    const string NAME;
    int health = 100;
    int money = Setting::get_start_money();
    int kills = 0;
    int killed = 0;
    map<GlobalVariable::type_gun, const Gun *> guns;
    const GlobalVariable::access_level ACCESS_LEVEL;
    const Time TIME;

    void can_bye(Gun *gun) const;

    void add_money(int _money);
};

#include "../src/Player.h"

#endif //COUNTER_STRIKE_QUERA_PLAYER_H
