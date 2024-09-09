#ifndef COUNTER_STRIKE_QUERA_GUNS_H
#define COUNTER_STRIKE_QUERA_GUNS_H

#include "Gun.h"

class Guns {
public:
    inline static const GunPointer& get_gun(const string &name, Gun::access_level accessLevel);

private:
    Guns() = default;
};

#include "Guns-inl.h"

#endif
