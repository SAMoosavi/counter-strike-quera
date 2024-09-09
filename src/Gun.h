#ifndef COUNTER_STRIKE_QUERA_GUN_H
#define COUNTER_STRIKE_QUERA_GUN_H

#include <string>
#include <utility>
#include <memory>

using std::string;

class Gun
{
public:
    enum type_gun
    {
        heavy,
        pistol,
        knife
    };
    enum access_level
    {
        setting,
        terrorist,
        counter_terrorist,
        all
    };

    enum type_team
    {
        Terrorist,
        Counter_Terrorist
    };

    inline Gun(string name, int price, int health, int money, type_gun type, access_level accessLevel);

    inline int get_price() const;

    inline type_gun get_type() const;

    inline access_level get_access_level() const;

    inline int get_money() const;

    inline int get_health() const;

    inline string get_name() const;

    inline static type_team string_to_team_enum(const string &team);

    inline static type_gun string_to_type_gun_enum(const string &gunType);

    inline static string team_enum_to_string(type_team team);

    inline static string type_gun_enum_to_string(type_gun gunType);

private:
    const access_level ACCESS_LEVEL;
    const type_gun TYPE;
    const string NAME;
    const int PRICE;
    const int HEALTH;
    const int MONEY;
};

using GunPointer = std::shared_ptr<Gun>;

#include "Gun-inl.h"

#endif
