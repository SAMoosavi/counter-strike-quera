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
            throw Error("Invalid team named " + team);
    }

    static GlobalVariable::type_gun string_to_type_gun_enum(const string &gunType) {
        if (gunType == "heavy")
            return GlobalVariable::type_gun::heavy;
        else if (gunType == "pistol")
            return GlobalVariable::type_gun::pistol;
        else if (gunType == "knife")
            return GlobalVariable::type_gun::knife;
        else
            throw Error("Invalid gun type " + gunType);
    }

    static string team_enum_to_string(GlobalVariable::team team) {
        if (team == GlobalVariable::team::Counter_Terrorist)
            return "Counter-Terrorist";
        else if (team == GlobalVariable::team::Terrorist)
            return "Terrorist";
        else
            throw Error("Invalid team enum ");
    }

    static string type_gun_enum_to_string(GlobalVariable::type_gun gunType) {
        if (gunType == GlobalVariable::type_gun::heavy)
            return "heavy";
        else if (gunType == GlobalVariable::type_gun::pistol)
            return "pistol";
        else if (gunType == GlobalVariable::type_gun::knife)
            return "knife";
        else
            throw Error("Invalid gun type enum");
    }
};

#endif //COUNTER_STRIKE_QUERA_HELPERFUNCTIONS_H
