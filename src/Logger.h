//
// Created by moosavi on 10/20/22.
//

#include "../include/Logger.h"

using std::cout;
using std::endl;

void Logger::log_successes(const std::string &message) {
    cout << message << endl;
}

void Logger::log_error(const string &message) {
    cout << message << endl;
}

void Logger::log_successes(long long int message) {
    cout << std::to_string(message) << endl;
}
