//
// Created by moosavi on 10/20/22.
//

#ifndef COUNTER_STRIKE_QUERA_TIME_H
#define COUNTER_STRIKE_QUERA_TIME_H

#include <string>
#include "Setting.h"

using std::string;

class Time {
public:
    inline explicit Time(const string &time, int round = 1);

    inline bool operator<(const Time &other) const;

    inline bool operator>(const Time &other) const;

    inline bool operator<(const string &other) const;

    inline bool operator>(const string &other) const;

    inline bool operator==(const Time &other) const;

    inline bool operator==(const string &other) const;

    inline Time operator*(int i) const;

    inline Time operator%(const Time &other) const;

    inline Time operator+(const Time &other) const;

private:
    long long int Milliseconds{};

    inline static long long int correct_str_to_milliseconds(const string &time);

    inline static bool str_is_correct(const string &time);

    inline explicit Time(long long int time);
};

#include "../src/Time.h"

#endif // COUNTER_STRIKE_QUERA_TIME_H
