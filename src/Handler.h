//
// Created by moosavi on 10/20/22.
//

#include "../include/Handler.h"
#include "../include/Logger.h"
#include "../include/HelperFunctions.h"

using std::to_string;

Player *Handler::find_player(const std::string &name) const {
    if (this->terrorist_class->has_player(name))
        return this->terrorist_class->get_player(name);

    if (this->counter_terrorist_class->has_player(name))
        return this->counter_terrorist_class->get_player(name);

    throw Error("invalid username");
}

bool Handler::has_player(const std::string &name) const {
    return this->terrorist_class->has_player(name) || this->counter_terrorist_class->has_player(name);
}

string Handler::add_user(const std::string &name, GlobalVariable::team team, const string &time) {
    if (this->has_player(name))
        throw Error("you are already in this game");

    auto time_of_add = Time(time, this->round);

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

    string msg =
            "this user added to " +
            string((team == GlobalVariable::team::Terrorist) ? "Terrorist" : "Counter-Terrorist");

    return msg;
}

int Handler::get_money(const std::string &username) const {
    return this->find_player(username)->get_money();
}

int Handler::get_health(const std::string &username) const {
    return this->find_player(username)->get_health();
}

string
Handler::tap(const std::string &attacker, const std::string &attacked, const GlobalVariable::type_gun type) const {
    auto attacker_player = this->find_player(attacker);
    auto attacked_player = this->find_player(attacked);

    if (!attacker_player->is_live()) throw Error("attacker is dead");
    if (!attacked_player->is_live()) throw Error("attacked is dead");

    if (!attacker_player->has_gun(type)) throw Error("no such gun");

    if (!(this->terrorist_class->has_player(attacker) ^ this->terrorist_class->has_player(attacked)))
        throw Error("friendly fire");

    if (attacked_player->shut(attacker_player->get_gun(type)->get_health())) { attacker_player->add_kill(type); }

    return "nice shot";
}

string Handler::buy(const string &username, const string &gunName, const string &time) const {
    auto player = this->find_player(username);

    if (!player->is_live()) throw Error("deads can not buy");

    if (Time(time) > Time(Setting::get_time_buy_gun())) throw Error("you are out of time");

    player->buy_gun(gunName);

    return "I hope you can use it";
}

string Handler::score_board() const {
    string msg = ":Counter-Terrorist-Players\n";
    auto counter_terrorist_score_board = this->counter_terrorist_class->get_score_board();
    for (int i = 0; i < counter_terrorist_score_board.size(); ++i) {
        msg += to_string(i + 1) + " " + counter_terrorist_score_board[i]->to_string() + "\n";
    }
    msg += ":Terrorist-Players\n";
    auto terrorist_score_board = this->terrorist_class->get_score_board();
    for (int i = 0; i < terrorist_score_board.size(); ++i) {
        msg += to_string(i + 1) + " " + terrorist_score_board[i]->to_string() + "\n";
    }

    return msg;
}

string Handler::new_round() {
    string msg;
    this->round++;

    if ((!this->counter_terrorist_class->has_live()) && this->terrorist_class->has_live()) {
        msg = "Terrorist won";
        this->terrorist_class->won();
        this->counter_terrorist_class->lose();
    } else {
        msg = "Counter-Terrorist won";
        this->terrorist_class->lose();
        this->counter_terrorist_class->won();
    }

    this->terrorist_class->new_round();
    this->counter_terrorist_class->new_round();

    return msg;
}

Handler::~Handler() {
    delete this->terrorist_class;
    delete this->counter_terrorist_class;
}
