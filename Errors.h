//
// Created by moosavi on 10/21/22.
//

#ifndef COUNTER_STRIKE_QUERA_ERRORS_H
#define COUNTER_STRIKE_QUERA_ERRORS_H

#include <exception>

#include <string>
#include <utility>

using std::string;

class Error : public std::exception {
public:
    explicit Error(string message) : MESSAGE(std::move(message)) {}

    string get_error() const { return this->MESSAGE; }

private:
    const string MESSAGE;
};

#endif //COUNTER_STRIKE_QUERA_ERRORS_H
