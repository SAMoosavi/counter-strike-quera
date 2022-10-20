//
// Created by moosavi on 10/20/22.
//

#ifndef COUNTER_STRIKE_QUERA_TIME_H
#define COUNTER_STRIKE_QUERA_TIME_H

#include <string>
#include "../Setting.h"


using std::string;

class Time {
public:
    explicit Time(const string &time, int round = 1) ;

    explicit Time(long long int time) { this->Milliseconds = time; }

    bool operator<(const Time &other) const;

    bool operator>(const Time &other) const;

    bool operator<(const string &other) const;

    bool operator>(const string &other) const;

    bool operator==(const Time &other) const;

    bool operator==(const string &other) const;

    Time operator*(int i) const;

private:
    long long int Milliseconds;

    static long long int correct_str_to_milliseconds(const string &time);

    static bool str_is_correct(const string &time);
};

#include "../src/Time.h"

#endif //COUNTER_STRIKE_QUERA_TIME_H
