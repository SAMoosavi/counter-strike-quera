//
// Created by moosavi on 10/20/22.
//

#ifndef COUNTER_STRIKE_QUERA_LOGGER_H
#define COUNTER_STRIKE_QUERA_LOGGER_H

#include <string>
#include <iostream>

using std::string;

class Logger {
public:
    inline static void log_successes(const string &message);

    inline static void log_successes(long long int message);

    inline static void log_error(const string &message);
};

#include "../src/Logger.h"

#endif // COUNTER_STRIKE_QUERA_LOGGER_H
