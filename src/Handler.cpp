#include "Handler.h"
#include "Gun.h"
#include "Error.h"

using std::to_string;

Player *Handler::find_player(const std::string &name) const {
    if (terrorist_class->has_player(name))
        return terrorist_class->get_player(name);

    if (counter_terrorist_class->has_player(name))
        return counter_terrorist_class->get_player(name);

    throw Error("invalid username");
}

bool Handler::has_player(const std::string &name) const {
    return terrorist_class->has_player(name) || counter_terrorist_class->has_player(name);
}

string Handler::add_user(const std::string &name, Gun::type_team team, const string &time) {
    if (has_player(name))
        throw Error("you are already in this game");

    auto time_of_add = Time(time, round);

    switch (team) {
        case Gun::type_team::Terrorist:
            terrorist_class->add_player(name, time_of_add);
            break;
        case Gun::type_team::Counter_Terrorist:
            counter_terrorist_class->add_player(name, time_of_add);
            break;
        default:
            throw Error("Unsupported variable type: " + Gun::team_enum_to_string(team));
    }

    string msg = "this user added to " + Gun::team_enum_to_string(team);

    return msg;
}

int Handler::get_money(const std::string &username) const {
    return find_player(username)->get_money();
}

int Handler::get_health(const std::string &username) const {
    return find_player(username)->get_health();
}

string
Handler::tap(const std::string &attacker, const std::string &attacked, const Gun::type_gun type) const {
    auto attacker_player = find_player(attacker);
    auto attacked_player = find_player(attacked);

    if (!attacker_player->is_live()) throw Error("attacker is dead");
    if (!attacked_player->is_live()) throw Error("attacked is dead");

    if (!attacker_player->has_gun(type)) throw Error("no such Gun");

    if (!(terrorist_class->has_player(attacker) ^ terrorist_class->has_player(attacked)))
        throw Error("friendly fire");

    if (attacked_player->shut(attacker_player->get_gun(type)->get_health())) { attacker_player->add_kill(type); }

    return "nice shot";
}

string Handler::buy(const string &username, const string &gunName, const string &time) const {
    auto player = find_player(username);

    if (!player->is_live()) throw Error("deads can not buy");

    if (Time(time) > Time(Setting::get_time_buy_gun())) throw Error("you are out of Time");

    player->buy_gun(gunName);

    return "I hope you can use it";
}

string Handler::time_score_board_to_string(vector<Player *> team_score_board) {
    string msg;
    for (int i = 0; i < team_score_board.size(); ++i) {
        msg += to_string(i + 1) + " " + team_score_board[i]->to_string() + "\n";
    }
    return msg;
}

string Handler::score_board() const {
    string msg = ":Counter-Terrorist-Players\n";
    msg += Handler::time_score_board_to_string(counter_terrorist_class->get_score_board());
    msg += ":Terrorist-Players\n";
    msg += Handler::time_score_board_to_string(terrorist_class->get_score_board());
    return msg;
}

string Handler::new_round() {
    string msg;
    round++;

    if ((!counter_terrorist_class->has_live()) && terrorist_class->has_live()) {
        msg = "Terrorist won";
        terrorist_class->won();
        counter_terrorist_class->lose();
    } else {
        msg = "Counter-Terrorist won";
        terrorist_class->lose();
        counter_terrorist_class->won();
    }

    terrorist_class->new_round();
    counter_terrorist_class->new_round();

    return msg;
}

Handler::~Handler() {
    delete terrorist_class;
    delete counter_terrorist_class;
}
