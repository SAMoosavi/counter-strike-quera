//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_PLAYER_H
#define COUNTER_STRIKE_QUERA_PLAYER_H

#include <string>
#include <map>
#include "../Setting.h"
#include "Time.h"
#include "Gun.h"

using std::string;
using std::map;

class Player {
public:
    explicit Player(string name, const Time &time) : NAME(std::move(name)), TIME(time) {}

    void reset();

    void add_kill(const string& name);

    bool shut(int health);

    void bye_gun(Gun *gun);

    int get_health() const;

    int get_money() const;

    int get_kills() const;

    int get_killed() const;

    Time get_time() const;

    bool is_live() const;

    void won();

    void lose();

    bool has_gun(const string& name) const;
private:
    const string NAME;
    int health = 100;
    int money = Setting::START_MONEY;
    int kills = 0;
    int killed = 0;
    map<string ,Gun *> guns;
    const Time TIME;

    void can_bye(Gun *gun) const;

    void add_money(int _money);
};

#include "../src/Player.h"

#endif //COUNTER_STRIKE_QUERA_PLAYER_H
