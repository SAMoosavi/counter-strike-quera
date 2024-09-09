#include "Player.h"
#include "Gun.h"
#include "Error.h"

Player::Player(std::string name, const Time &time, Gun::access_level accessLevel) :
        NAME(std::move(name)),
        TIME(time),
        ACCESS_LEVEL(accessLevel) {
    guns[Setting::get_start_gun()->get_type()] = Setting::get_start_gun();
    if ((time % Time(Setting::get_end_time())) > Time(Setting::get_time_add_player()))
        health = 0;
}

void Player::reset() {
    health = 100;
    guns[Setting::get_start_gun()->get_type()] = Setting::get_start_gun();
}

bool Player::shut(int health_) {
    if (!is_live())
        throw Error("Player is not live!");
    health -= health_;
    if (health <= 0) {
        killed++;
        health = 0;
        guns.clear();
    }
    return !health;
}

void Player::buy_gun(const string &name) {
    auto gun = Guns::get_gun(name, ACCESS_LEVEL);
    can_bye(gun);
    money -= gun->get_price();
    guns[gun->get_type()] = gun;
}

void Player::can_bye(const GunPointer & gun) const {
    if (guns.count(gun->get_type())) {
        throw Error("you have a " + Gun::type_gun_enum_to_string(gun->get_type()));
    }

    if (gun->get_price() > money) {
        throw Error("no enough money");
    }
}

void Player::add_kill(Gun::type_gun type) {
    if (!has_gun(type))
        throw Error("you have no Gun named" + Gun::type_gun_enum_to_string(type));
    kills++;
    add_money(guns[type]->get_money());
}

int Player::get_health() const { return health; }

int Player::get_money() const { return money; }

int Player::get_kills() const {
    return kills;
}

int Player::get_killed() const {
    return killed;
}

void Player::won() {
    add_money(Setting::get_won_money());
}

void Player::lose() {
    add_money(Setting::get_lose_money());
}

void Player::add_money(int _money) {
    money += _money;
    if (money > Setting::get_max_money()) {
        money = Setting::get_max_money();
    }
}

bool Player::is_live() const {
    return health;
}

bool Player::has_gun(Gun::type_gun type) const {
    return guns.count(type);
}

GunPointer  Player::get_gun(Gun::type_gun type) const {
    if (has_gun(type))
        return guns.at(type);
    else
        return nullptr;
}

ostream &operator<<(ostream &output, const Player &player) {
    output << player.to_string();
    return output;
}

bool Player::operator<(const Player &other) const {
    if (kills < other.kills)
        return true;
    if (kills == other.kills) {
        if (killed > other.killed)
            return true;
        if (killed == other.killed)
            return TIME > other.TIME;
    }
    return false;
}

bool Player::operator>(const Player &other) const {
    return !(*this < other);
}

string Player::to_string() const {
    return string(NAME + " " + std::to_string(kills) + " " + std::to_string(killed));
}
