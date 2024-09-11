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
    explicit Player(string name, const Time &time, Gun::access_level accessLevel);

    void reset();

    void add_kill(Gun::type_gun type);

    bool shut(int health_);

    void buy_gun(const string &name);

    int get_health() const;

    int get_money() const;

    int get_kills() const;

    int get_killed() const;

    bool is_live() const;

    void won();

    void lose();

    bool has_gun(Gun::type_gun type) const;

    GunPointer  get_gun(Gun::type_gun type) const;

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
    map<Gun::type_gun, GunPointer > guns;
    const Gun::access_level ACCESS_LEVEL;
    const Time TIME;

    void can_bye(const GunPointer & gun) const;

    void add_money(int _money);
};

#endif
