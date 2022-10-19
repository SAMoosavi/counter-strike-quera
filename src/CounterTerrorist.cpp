//
// Created by moosavi on 10/19/22.
//

#include "../include/CounterTerrorist.h"

void CounterTerrorist::create_gun() {
    this->guns["AK"] = (new Gun("AK", 2700, 31, 100, GlobalVariable::type_gun::heavy));
    this->guns["AWP"] = (new Gun("AWP", 4300, 110, 50, GlobalVariable::type_gun::heavy));

    this->guns["Revolver"] = (new Gun("Revolver", 600, 51, 150, GlobalVariable::type_gun::pistol));
    this->guns["Glock-18"] = (new Gun("Glock-18", 300, 11, 200, GlobalVariable::type_gun::pistol));

    this->guns["knife"] = (new Gun("knife", 0, 43, 500, GlobalVariable::type_gun::knife));
}
