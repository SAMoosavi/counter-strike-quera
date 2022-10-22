//
// Created by moosavi on 10/22/22.
//

#include "../include/HelperFunctions.h"

GlobalVariable::team HelperFunctions::string_to_team_enum(const std::string &team) {
    if (team == "Counter-Terrorist")
        return GlobalVariable::team::Counter_Terrorist;
    else if (team == "Terrorist")
        return GlobalVariable::team::Terrorist;
    else
        throw Error("Invalid team named " + team);
}

GlobalVariable::type_gun HelperFunctions::string_to_type_gun_enum(const string &gunType) {
    if (gunType == "heavy")
        return GlobalVariable::type_gun::heavy;
    else if (gunType == "pistol")
        return GlobalVariable::type_gun::pistol;
    else if (gunType == "knife")
        return GlobalVariable::type_gun::knife;
    else
        throw Error("Invalid gun type " + gunType);
}

string HelperFunctions::team_enum_to_string(GlobalVariable::team team) {
    if (team == GlobalVariable::team::Counter_Terrorist)
        return "Counter-Terrorist";
    else if (team == GlobalVariable::team::Terrorist)
        return "Terrorist";
    else
        throw Error("Invalid team enum ");
}

string HelperFunctions::type_gun_enum_to_string(GlobalVariable::type_gun gunType) {
    if (gunType == GlobalVariable::type_gun::heavy)
        return "heavy";
    else if (gunType == GlobalVariable::type_gun::pistol)
        return "pistol";
    else if (gunType == GlobalVariable::type_gun::knife)
        return "knife";
    else
        throw Error("Invalid gun type enum");
}
