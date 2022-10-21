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
    this->life = (int) this->players.size();
}

vector<Player *> Team::get_score_board() const {
    vector<Player *> result_players;
    for (const auto &player: this->players)
        result_players.push_back(player.second);
    std::sort(result_players.begin(), result_players.end());
    return result_players;
}

bool Team::has_life() const {
    return (bool) this->life;
}

Team::~Team() {
    for (const auto &player: this->players)
        delete player.second;
    this->players.clear();
}
