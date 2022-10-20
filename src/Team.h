//
// Created by moosavi on 10/19/22.
//
#include "../include/Team.h"

void Team::add_player(const string &name) {
    if(this->players.size() == 10)
        throw "this team is full";
    this->players[name] = new Player(name, Guns::get_gun("knife", this->ACCESS_LEVEL));
}