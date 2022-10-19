//
// Created by moosavi on 10/19/22.
//

#include "../include/Guns.h"

map<string, Gun *> Guns::guns = {
        {"AK",           (new Gun("AK", 2700, 31, 100, GlobalVariable::type_gun::heavy, GlobalVariable::access_level::terrorist))},
        {"AWP",          (new Gun("AWP", 4300, 110, 50, GlobalVariable::type_gun::heavy, GlobalVariable::access_level::all))},
        {"Desert-Eagle", (new Gun("Desert-Eagle", 600, 53, 175, GlobalVariable::type_gun::pistol, GlobalVariable::access_level::counter_terrorist))},
        {"UPS-S",        (new Gun("UPS-S", 300, 13, 225, GlobalVariable::type_gun::pistol, GlobalVariable::access_level::counter_terrorist))},
        {"Revolver",     (new Gun("Revolver", 600, 51, 150, GlobalVariable::type_gun::pistol, GlobalVariable::access_level::terrorist))},
        {"Glock-18",     (new Gun("Glock-18", 300, 11, 200, GlobalVariable::type_gun::pistol, GlobalVariable::access_level::terrorist))},
        {"knife",        (new Gun("knife", 0, 43, 500, GlobalVariable::type_gun::knife, GlobalVariable::access_level::all))}
};


Gun * Guns::get_gun(string name, GlobalVariable::access_level accessLevel) {
    if(!Guns::guns.count(name))
        throw "invalid category gun";
    else if(Guns::guns[name]->get_access_level() == GlobalVariable::access_level::all) return  Guns::guns[name];
    else if (Guns::guns[name]->get_access_level() == accessLevel) return Guns::guns[name];
    else throw "invalid category gun";
}