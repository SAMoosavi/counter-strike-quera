//
// Created by moosavi on 10/20/22.
//

#include "../include/Handler.h"
#include "../include/Logger.h"
#include "../HelperFunctions.h"

Player *Handler::find_player(const std::string &name) const {
    if (this->terrorist_class->has_player(name))
        return this->terrorist_class->get_player(name);

    if (this->counter_terrorist_class->has_player(name))
        return this->counter_terrorist_class->get_player(name);

    throw Error("invalid username");
}

bool Handler::has_player(const std::string &name) const {
    return this->terrorist_class->has_player(name) && this->counter_terrorist_class->has_player(name);
}

void Handler::add_user(const std::string &name, GlobalVariable::team team, const string &time) {
// check has player
    if (this->has_player(name))
        throw Error("you are already in this game");
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
            throw Error("Unsupported variable type: " + HelperFunctions::team_enum_to_string(team));
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

void Handler::tap(const std::string &attacker, const std::string &attacked, const GlobalVariable::type_gun type) const {
    auto attacker_player = this->find_player(attacker);
    auto attacked_player = this->find_player(attacked);

    if (!attacker_player->is_live()) throw Error("attacker is dead");
    if (!attacked_player->is_live()) throw Error("attacked is dead");

    if (!attacker_player->has_gun(type)) throw Error("no such gun");

    if (this->terrorist_class->has_player(attacker) ^ this->terrorist_class->has_player(attacked))
        throw Error("friendly fire");

    Logger::log_successes("nice shot");

    if (attacked_player->shut(attacker_player->get_gun(type)->get_health()))
        attacker_player->add_kill(type);
}

void Handler::buy(const string &username, const string &gunName, const string &time) const {
    auto player = this->find_player(username);

    if (!player->is_live()) throw Error("deads can not buy");

    if (Time(time) > Time(Setting::get_time_buy_gun())) throw Error("you are out of time");

    player->buy_gun(gunName);

    Logger::log_successes("I hope you can use it");
}

void Handler::score_board() const {
    Logger::log_successes(":Counter-Terrorist-Players");
    auto counter_terrorist_score_board = this->counter_terrorist_class->get_score_board();
    for (int i = 0; i < counter_terrorist_score_board.size(); ++i) {
        Logger::log_successes(std::to_string(i + 1) + " " + counter_terrorist_score_board[i]->to_string());
    }
    Logger::log_successes(":Terrorist-Players");
    auto terrorist_score_board = this->terrorist_class->get_score_board();
    for (int i = 0; i < terrorist_score_board.size(); ++i) {
        Logger::log_successes(std::to_string(i + 1) + " " + terrorist_score_board[i]->to_string());
    }
}

void Handler::new_round() {
    this->round++;

    if ((!this->counter_terrorist_class->has_life()) && this->terrorist_class->has_life()) {
        Logger::log_successes("Terrorist won");
        this->terrorist_class->won();
        this->counter_terrorist_class->lose();
    } else {
        Logger::log_successes("Counter-Terrorist won");
        this->terrorist_class->lose();
        this->counter_terrorist_class->won();
    }

    this->terrorist_class->new_round();
    this->counter_terrorist_class->new_round();
}
