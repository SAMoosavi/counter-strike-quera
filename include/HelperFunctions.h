//
// Created by moosavi on 10/21/22.
//

#ifndef COUNTER_STRIKE_QUERA_HELPERFUNCTIONS_H
#define COUNTER_STRIKE_QUERA_HELPERFUNCTIONS_H

#include "../GlobalVariable.h"
#include <string>

using std::string;

class HelperFunctions {
public:
    static GlobalVariable::team string_to_team_enum(const string &team);

    static GlobalVariable::type_gun string_to_type_gun_enum(const string &gunType);

    static string team_enum_to_string(GlobalVariable::team team);

    static string type_gun_enum_to_string(GlobalVariable::type_gun gunType);
};

#include "../src/HelperFunctions.h"

#endif //COUNTER_STRIKE_QUERA_HELPERFUNCTIONS_H
