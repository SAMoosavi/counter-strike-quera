//
// Created by moosavi on 10/19/22.
//
#include "../include/Team.h"

void Team::add_player(const string &name, const Time &time) {
    if (this->players.size() == Setting::get_max_member_team())
        throw "this team is full";
    this->players[name] = new Player(name, time);
}

bool Team::has_player(const std::string &name) const { return this->players.count(name); }