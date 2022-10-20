//
// Created by moosavi on 10/19/22.
//

#ifndef COUNTER_STRIKE_QUERA_GLOBALVARIABLE_H
#define COUNTER_STRIKE_QUERA_GLOBALVARIABLE_H

class GlobalVariable {
public:
    enum type_gun {
        heavy, pistol, knife
    };
    enum access_level {
        setting, terrorist, counter_terrorist, all
    };

    enum team {
        Terrorist, Counter_Terrorist
    };
};

#endif //COUNTER_STRIKE_QUERA_GLOBALVARIABLE_H
