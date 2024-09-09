#ifndef COUNTER_STRIKE_QUERA_ERROR_H
#define COUNTER_STRIKE_QUERA_ERROR_H

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

#endif
