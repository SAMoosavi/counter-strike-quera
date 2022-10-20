//
// Created by moosavi on 10/20/22.
//

#include "../include/Handler.h"
#include "../include/Logger.h"

Player *Handler::find_player(const std::string &name) const {
    if (this->terrorist_class->has_player(name))
        return this->terrorist_class->get_player(name);

    if (this->counter_terrorist_class->has_player(name))
        return this->counter_terrorist_class->get_player(name);

    throw "invalid username";
}

bool Handler::has_player(const std::string &name) const {
    return this->terrorist_class->has_player(name) && this->counter_terrorist_class->has_player(name);
}

void Handler::add_user(const std::string &name, GlobalVariable::team team, const string &time) const {
// check has player
    if (this->has_player(name))
        throw "you are already in this game";
// set time
    auto time_of_add = Time(time, this->round);
// add player
    switch (team) {
        case GlobalVariable::team::Terrorist:
            this->terrorist_class->add_player(name, time_of_add);
            break;
        case GlobalVariable::team::Counter_Terrorist:
            this->counter_terrorist_class->add_player(name, time_of_add);
            break;
        default:
            throw "Unsupported variable type: " + team;
    }
// log successes
    string msg =
            "this user added to " +
            string((team == GlobalVariable::team::Terrorist) ? "Terrorist" : "Counter-Terrorist");

    Logger::log_successes(msg);
}

void Handler::get_money(const std::string &username) const {
//    get money
    int money = this->find_player(username)->get_money();
//    log success
    Logger::log_successes(std::to_string(money));
}

void Handler::get_health(const std::string &username) const {
    //    get health
    int health = this->find_player(username)->get_health();
//    log success
    Logger::log_successes(std::to_string(health));
}
