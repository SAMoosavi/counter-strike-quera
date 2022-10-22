//
// Created by moosavi on 10/19/22.
//

#include "../include/Player.h"
#include "../include/HelperFunctions.h"

Player::Player(std::string name, const Time &time, GlobalVariable::access_level accessLevel) :
        NAME(std::move(name)),
        TIME(time),
        ACCESS_LEVEL(accessLevel) {
    this->guns[Setting::get_start_gun()->get_type()] = Setting::get_start_gun();
    if ((time % Time(Setting::get_end_time())) > Time(Setting::get_time_add_player()))
        this->health = 0;
}

void Player::reset() {
    this->health = 100;
    this->guns[Setting::get_start_gun()->get_type()] = Setting::get_start_gun();
}

bool Player::shut(int health_) {
    if (!this->is_live())
        throw Error("Player is not live!");
    this->health -= health_;
    if (this->health <= 0) {
        this->killed++;
        this->health = 0;
        this->guns.clear();
    }
    return !this->health;
}

void Player::buy_gun(const string &name) {
    auto gun = Guns::get_gun(name, this->ACCESS_LEVEL);
    this->can_bye(gun);
    this->money -= gun->get_price();
    this->guns[gun->get_type()] = gun;
}

void Player::can_bye(Gun *gun) const {
    if (this->guns.count(gun->get_type())) {
        throw Error("you have a " + HelperFunctions::type_gun_enum_to_string(gun->get_type()));
    }

    if (gun->get_price() > this->money) {
        throw Error("no enough money");
    }
}

void Player::add_kill(GlobalVariable::type_gun type) {
    if (!this->has_gun(type))
        throw Error("you have no gun named" + HelperFunctions::type_gun_enum_to_string(type));
    this->kills++;
    this->add_money(this->guns[type]->get_money());
}

int Player::get_health() const { return this->health; }

int Player::get_money() const { return this->money; }

int Player::get_kills() const {
    return this->kills;
}

int Player::get_killed() const {
    return this->killed;
}

void Player::won() {
    this->add_money(Setting::get_won_money());
}

void Player::lose() {
    this->add_money(Setting::get_lose_money());
}

void Player::add_money(int _money) {
    this->money += _money;
    if (this->money > Setting::get_max_money()) {
        this->money = Setting::get_max_money();
    }
}

bool Player::is_live() const {
    return this->health;
}

bool Player::has_gun(GlobalVariable::type_gun type) const {
    return this->guns.count(type);
}

Gun *Player::get_gun(GlobalVariable::type_gun type) const {
    if (this->has_gun(type))
        return const_cast<Gun *>(this->guns.at(type));
    else
        return nullptr;
}

ostream &operator<<(ostream &output, const Player &player) {
    output << player.to_string();
    return output;
}

bool Player::operator<(const Player &other) const {
    if (this->kills < other.kills)
        return true;
    if (this->kills == other.kills) {
        if (this->killed > other.killed)
            return true;
        if (this->killed == other.killed)
            return this->TIME > other.TIME;
    }
    return false;
}

bool Player::operator>(const Player &other) const {
    return !(*this < other);
}

string Player::to_string() const {
    return string(this->NAME + " " + std::to_string(this->kills) + " " + std::to_string(this->killed));
}
