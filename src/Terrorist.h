//
// Created by moosavi on 10/19/22.
//

#include "../include/Terrorist.h"

Terrorist *Terrorist::create_class() {
    if (Terrorist::t == nullptr) return new Terrorist();
    return Terrorist::t;
}