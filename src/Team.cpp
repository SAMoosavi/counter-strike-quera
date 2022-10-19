//
// Created by moosavi on 10/19/22.
//
#include "../include/Team.h"

void Team::add_player(const string &name) {
    this->players[name] = (new Player(name, this->guns["knife"]));
}