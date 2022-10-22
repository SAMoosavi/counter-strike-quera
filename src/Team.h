//
// Created by moosavi on 10/19/22.
//
#include "../include/Team.h"

Team::Team(GlobalVariable::access_level accessLevel) : ACCESS_LEVEL(accessLevel) {}


void Team::add_player(const string &name, const Time &time) {
    if (this->players.size() == Setting::get_max_member_team())
        throw Error("this team is full");
    this->players[name] = new Player(name, time, this->ACCESS_LEVEL);
}

bool Team::has_player(const std::string &name) const { return this->players.count(name); }

Player *Team::get_player(const std::string &name) const {
    if (this->players.count(name))
        return this->players.at(name);
    throw Error("invalid username");
}

void Team::new_round() {
    for (auto &player: this->players) {
        player.second->reset();
    }
}

vector<Player *> Team::get_score_board() const {
    vector<Player *> result_players;
    for (const auto &player: this->players)
        result_players.push_back(player.second);
    std::sort(result_players.begin(), result_players.end());
    return result_players;
}

bool Team::has_live() const {
    return this->get_live_num() > 0;
}

Team::~Team() {
    for (const auto &player: this->players)
        delete player.second;
    this->players.clear();
}

void Team::won() const {
    for (auto &player: this->players)
        player.second->won();
}

void Team::lose() const {
    for (auto &player: this->players)
        player.second->lose();
}

int Team::get_live_num() const {
    int liveNum = 0;
    for (auto &player: this->players)
        if (player.second->is_live()) liveNum++;
    return liveNum;
}
