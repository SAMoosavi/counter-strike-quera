//
// Created by moosavi on 10/20/22.
//

#include "../include/Time.h"
#include "../Errors.h"

#include <regex>

using std::regex;

Time::Time(const string &time, int round) {
    if (round == 1)
        this->Milliseconds = Time::correct_str_to_milliseconds(time);
    else
        this->Milliseconds =
                (Time(Setting::get_end_time()) * (round - 1)).Milliseconds +
                Time::correct_str_to_milliseconds(time);
}

bool Time::operator<(const Time &other) const { return this->Milliseconds < other.Milliseconds; }

bool Time::operator<(const string &other) const {
    return this->Milliseconds < Time::correct_str_to_milliseconds(other);
}

bool Time::operator>(const Time &other) const { return this->Milliseconds > other.Milliseconds; }

bool Time::operator>(const string &other) const {
    return this->Milliseconds > Time::correct_str_to_milliseconds(other);
}

long long int Time::correct_str_to_milliseconds(const std::string &time) {
    if (!Time::str_is_correct(time))
        throw Error("Time::correct_str_to_milliseconds() failed: " + std::string(time));
    long long int minute = stoll(time.substr(0, 2));
    long long int second = stoll(time.substr(3, 2));
    long long int millisecond = stoll(time.substr(6, 3));

    return minute * 60 * 1000 + second * 1000 + millisecond;
}

bool Time::str_is_correct(const std::string &time) {
    regex timePattern(R"(\d\d:\d\d:\d\d\d)");
    return regex_match(time, timePattern);
}

bool Time::operator==(const Time &other) const {
    return this->Milliseconds == other.Milliseconds;
}

bool Time::operator==(const string &other) const {
    return this->Milliseconds == Time::correct_str_to_milliseconds(other);
}

Time Time::operator*(int i) const {
    return Time(this->Milliseconds * i);
}

Time Time::operator%(const Time &other) const {
    return Time(this->Milliseconds % other.Milliseconds);
}

Time Time::operator+(const Time &other) const {
    return Time(this->Milliseconds + other.Milliseconds);
}

Time::Time(long long int time) : Milliseconds(time) {}
