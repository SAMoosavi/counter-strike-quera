//
// Created by moosavi on 10/19/22.
//
#include "../include/Team.h"

void Team::add_player(const string &name, const Time &time) {
    if (this->players.size() == Setting::get_max_member_team())
        throw "this team is full";
    this->players[name] = new Player(name, time, this->ACCESS_LEVEL);
}

bool Team::has_player(const std::string &name) const { return this->players.count(name); }

Player *Team::get_player(const std::string &name) const {
    if (this->players.count(name))
        throw "invalid username";
    return this->players.at(name);
}

void Team::new_round() {
    for (auto &player: this->players) {
        player.second->reset();
    }
    this->life = this->players.size();
}