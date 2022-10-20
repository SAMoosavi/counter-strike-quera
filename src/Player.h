//
// Created by moosavi on 10/19/22.
//

#include "../include/Player.h"

void Player::reset() {
    this->health = 100;
    this->guns[Setting::get_start_gun()->get_name()] = Setting::get_start_gun();
}

bool Player::shut(int health) {
    if (!this->is_live())
        throw "Player is not live!";
    this->health -= health;
    if (this->health <= 0) {
        this->killed++;
        this->health = 0;
        this->guns.clear();
    }
    return !this->health;
}

void Player::bye_gun(Gun *gun) {
    this->can_bye(gun);
    this->money -= gun->get_price();
    this->guns[gun->get_name()] = gun;
}

void Player::can_bye(Gun *gun) const {
    for (const auto &playerGun: this->guns) {
        if (gun->get_type() == playerGun.second->get_type()) {
            throw "you have a" + (gun->get_type() == GlobalVariable::type_gun::heavy) ? "heavy" : "pistol";
        }
    }

    if (gun->get_price() > this->money) {
        throw "no enough money";
    }
}

void Player::add_kill(const string &name) {
    if (!this->has_gun(name))
        throw "you have no gun named" + name;
    this->kills++;
    this->add_money(this->guns[name]->get_money());
}

int Player::get_health() const { return this->health; }

int Player::get_money() const { return this->money; }

int Player::get_kills() const {
    return this->kills;
}

int Player::get_killed() const {
    return this->killed;
}

Time Player::get_time() const {
    return this->TIME;
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

bool Player::has_gun(const string &name) const {
    return this->guns.count(name);
}
