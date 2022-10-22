//
// Created by moosavi on 10/19/22.
//

#include "../include/Gun.h"

Gun::Gun(std::string name, int price, int health, int money, GlobalVariable::type_gun type,
         GlobalVariable::access_level accessLevel) :
        NAME(std::move(name)),
        PRICE(price),
        HEALTH(health),
        MONEY(money),
        TYPE(type),
        ACCESS_LEVEL(accessLevel) {}

int Gun::get_price() const { return this->PRICE; }

GlobalVariable::type_gun Gun::get_type() const { return this->TYPE; }

GlobalVariable::access_level Gun::get_access_level() const { return this->ACCESS_LEVEL; }

int Gun::get_money() const { return this->MONEY; }

int Gun::get_health() const { return this->HEALTH; }

string Gun::get_name() const { return this->NAME; }
