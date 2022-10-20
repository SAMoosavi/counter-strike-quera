//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_COUNTER_TERRORIST_H
#define COUNTER_STRIKE_QUERA_COUNTER_TERRORIST_H

#include "Team.h"

class CounterTerrorist : public Team {
public:
    static CounterTerrorist *create_class();
private:
    static CounterTerrorist *ct;
    CounterTerrorist() : Team(GlobalVariable::access_level::counter_terrorist) {}
};

#include "../src/CounterTerrorist.h"

#endif //COUNTER_STRIKE_QUERA_COUNTER_TERRORIST_H
