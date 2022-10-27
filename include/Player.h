//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_PLAYER_H
#define COUNTER_STRIKE_QUERA_PLAYER_H

#include <string>
#include <map>
#include <iostream>
#include "Setting.h"
#include "Time.h"
#include "Gun.h"

using std::map;
using std::ostream;
using std::string;

class Player {
public:
    inline explicit Player(string name, const Time &time, GlobalVariable::access_level accessLevel);

    inline void reset();

    inline void add_kill(GlobalVariable::type_gun type);

    inline bool shut(int health_);

    inline void buy_gun(const string &name);

    inline int get_health() const;

    inline int get_money() const;

    inline int get_kills() const;

    inline int get_killed() const;

    inline bool is_live() const;

    inline void won();

    inline void lose();

    inline bool has_gun(GlobalVariable::type_gun type) const;

    inline Gun *get_gun(GlobalVariable::type_gun type) const;

    inline friend ostream &operator<<(ostream &output, const Player &player);

    inline string to_string() const;

    inline bool operator<(const Player &other) const;

    inline bool operator>(const Player &other) const;

private:
    const string NAME;
    int health = 100;
    int money = Setting::get_start_money();
    int kills = 0;
    int killed = 0;
    map<GlobalVariable::type_gun, const Gun *> guns;
    const GlobalVariable::access_level ACCESS_LEVEL;
    const Time TIME;

    inline void can_bye(Gun *gun) const;

    inline void add_money(int _money);
};

#include "../src/Player.h"

#endif // COUNTER_STRIKE_QUERA_PLAYER_H
