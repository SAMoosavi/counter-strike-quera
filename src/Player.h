//
// Created by moosavi on 10/19/22.
//

#include "../include/Player.h"

void Player::reset() {
    this->health = 100;
}

bool Player::shut(int health) {
    if(!this->is_live())
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
    this->guns.push_back(gun);
}

void Player::can_bye(Gun *gun) const {
    for (auto playerGun: this->guns) {
        if (gun->get_type() == playerGun->get_type()) {
            throw "you have a" + (gun->get_type() == GlobalVariable::type_gun::heavy) ? "heavy" : "pistol";
        }
    }

    if (gun->get_price() > this->money) {
        throw "no enough money";
    }
}

void Player::add_kill(int money) {
    this->kills++;
}

int Player::get_health() const { return this->health; }

int Player::get_money() const { return this->money; }

int Player::get_kills() const {
    return this->kills;
}

int Player::get_killed() const {
    return this->killed;
}

string Player::get_time() const {
    return this->TIME;
}

void Player::won() {
    this->add_money(Setting::WON_MONEY);
}

void Player::lose() {
    this->add_money(Setting::LOSE_MONEY);
}

void Player::add_money(int money) {
    this->money += money;
    if (this->money > Setting::MAX_MONEY) {
        this->money = Setting::MAX_MONEY;
    }
}

bool Player::is_live() const {
    return this->health;
}
