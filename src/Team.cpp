//
// Created by moosavi on 10/19/22.
//
#include <utility>

#include "../include/Team.h"

void Team::add_player(string name) {
    this->players[name] = (new Player(std::move(name), this->guns["knife"]));
}