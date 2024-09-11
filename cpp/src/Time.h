#ifndef COUNTER_STRIKE_QUERA_TIME_H
#define COUNTER_STRIKE_QUERA_TIME_H

#include <string>
#include "Setting.h"

using std::string;

class Time {
public:
    explicit Time(const string &time, int round = 1);

    bool operator<(const Time &other) const;

    bool operator>(const Time &other) const;

    bool operator<(const string &other) const;

    bool operator>(const string &other) const;

    bool operator==(const Time &other) const;

    bool operator==(const string &other) const;

    Time operator*(int i) const;

    Time operator%(const Time &other) const;

    Time operator+(const Time &other) const;

private:
    long long int Milliseconds{};

    static long long int correct_str_to_milliseconds(const string &time);

    static bool str_is_correct(const string &time);

    explicit Time(long long int time);
};

#endif
