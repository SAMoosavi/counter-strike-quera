#ifndef COUNTER_STRIKE_QUERA_LOGGER_H
#define COUNTER_STRIKE_QUERA_LOGGER_H

#include <string>
#include <iostream>

using std::string;

class Logger {
public:
    static void log_successes(const string &message);

    static void log_successes(long long int message);

    static void log_error(const string &message);
};


#endif
