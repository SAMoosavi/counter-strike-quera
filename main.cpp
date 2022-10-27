#include <iostream>
#include <string>
#include "include/Handler.h"
#include "HelperFunctions.h"
#include "Logger.h"
#include "Errors.h"

using std::string;
using std::cin;

int main() {
    int round;
    cin >> round;
    auto handler = new Handler();
    string order, time;
    int orderNumber;
    for (int i = 0; i < round; ++i) {
        cin >> order >> orderNumber;
        for (int j = 0; j < orderNumber; ++j) {
            cin >> order;
            try {
                if (order == "ADD-USER") {
                    string username, team;
                    cin >> username >> team >> time;
                    Logger::log_successes(handler->add_user(username, HelperFunctions::string_to_team_enum(team), time));
                } else if (order == "GET-MONEY") {
                    string username;
                    cin >> username >> time;
                    Logger::log_successes(handler->get_money(username));
                } else if (order == "GET-HEALTH") {
                    string username;
                    cin >> username >> time;
                    Logger::log_successes(handler->get_health(username));
                } else if (order == "TAP") {
                    string attacker, attacked, gunType;
                    cin >> attacker >> attacked >> gunType >> time;
                    Logger::log_successes(
                            handler->tap(attacker, attacked, HelperFunctions::string_to_type_gun_enum(gunType)));
                } else if (order == "BUY") {
                    string username, gunName;
                    cin >> username >> gunName >> time;
                    Logger::log_successes(handler->buy(username, gunName, time));
                } else if (order == "SCORE-BOARD") {
                    cin >> time;
                    Logger::log_successes(handler->score_board());
                }
            }
            catch (const Error &error) {
                Logger::log_error(error.get_error());
            }
        }
        Logger::log_successes(handler->new_round());
    }
}
