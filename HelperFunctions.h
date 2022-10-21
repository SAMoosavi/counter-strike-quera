//
// Created by moosavi on 10/21/22.
//

#ifndef COUNTER_STRIKE_QUERA_HELPERFUNCTIONS_H
#define COUNTER_STRIKE_QUERA_HELPERFUNCTIONS_H

#include "GlobalVariable.h"
#include <string>

using std::string;

class HelperFunctions {
public:
    static GlobalVariable::team string_to_team_enum(const string &team) {
        if (team == "Counter-Terrorist")
            return GlobalVariable::team::Counter_Terrorist;
        else if (team == "Terrorist")
            return GlobalVariable::team::Terrorist;
        else
            throw "Invalid team named " + team;
    }

    static GlobalVariable::type_gun string_to_type_gun_enum(const string &gunType) {
        if (gunType == "heavy")
            return GlobalVariable::type_gun::heavy;
        else if (gunType == "pistol")
            return GlobalVariable::type_gun::pistol;
        else if (gunType == "knife")
            return GlobalVariable::type_gun::knife;
        else
            throw "Invalid gun type " + gunType;
    }
};

#endif //COUNTER_STRIKE_QUERA_HELPERFUNCTIONS_H
