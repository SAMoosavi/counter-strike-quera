//
// Created by moosavi on 10/19/22.
//

#include "../include/Player.h"

void Player::reset() {
    this->health = 100;
}

bool Player::shut(int health) {
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
