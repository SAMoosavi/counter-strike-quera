//
// Created by moosavi on 10/19/22.
//

#include "../include/CounterTerrorist.h"

CounterTerrorist *CounterTerrorist::create_class() {
    if (CounterTerrorist::ct == nullptr) return new CounterTerrorist();
    return CounterTerrorist::ct;
}