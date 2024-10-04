use std::rc::Rc;

use super::Team;
use crate::game_time::GameTime;
use crate::gun::{Gun, Guns, TypeOfGun};
use crate::player::Player;
use crate::setting::Setting;

fn fill_setting_for_create_player() {
    let gun = Gun::new("knife".to_string(), 100, 10, 20, TypeOfGun::Knife);
    Setting::set_default_gun(&Rc::new(gun)).unwrap();
    Setting::set_default_money_of_player(1000).unwrap();
    Setting::set_won_team_money(2700).unwrap();
    Setting::set_lose_team_money(2400).unwrap();
    Setting::set_max_time_buy(&GameTime::new(0, 45, 0)).unwrap();
    Setting::set_did_time_of_player(&GameTime::new(0, 3, 0)).unwrap();

}

#[test]
pub fn test_default() {
    let team = Team::new("team".to_string());
    assert_eq!(team.guns, Box::new(Guns::new()));
    assert_eq!(team.players, vec!());
}

#[test]
pub fn test_add_player_should_be_return_error_when_does_not_set_max_number_of_players_in_setting() {
    Setting::reset();
    fill_setting_for_create_player();
    let mut team = Team::new("team".to_string());
    let time = GameTime::new(0, 0, 1);

    let result = team.add_player("Player", &time);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "the maximum number of players doesn't set!"
    );
    Setting::reset();
}

#[test]
pub fn test_add_player_should_be_return_error_when_team_is_full() {
    Setting::reset();
    fill_setting_for_create_player();
    let mut team = Team::new("team".to_string());
    let time = GameTime::new(0, 0, 1);
    Setting::set_max_money_of_player(1).unwrap();
    team.add_player("Player 1", &time).unwrap();

    let result = team.add_player("Player 2", &time);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "the team is full!");
    Setting::reset();
}

#[test]
pub fn test_add_player_should_be_return_error_when_player_exists_with_same_name() {
    Setting::reset();
    fill_setting_for_create_player();
    let mut team = Team::new("team".to_string());
    let time = GameTime::new(0, 0, 1);
    let name = "Player";
    Setting::set_max_money_of_player(2).unwrap();
    team.add_player(name, &time).unwrap();

    let result = team.add_player(name, &time);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "you are already in this game");
    Setting::reset();
}

#[test]
pub fn test_add_player_added_to_least_of_players() {
    Setting::reset();
    fill_setting_for_create_player();
    let mut team = Team::new("team".to_string());
    Setting::set_max_money_of_player(2).unwrap();
    let name = "Player";
    let time = GameTime::new(0, 0, 1);

    let result = team.add_player(name, &time);

    assert!(result.is_ok());
    assert_eq!(team.players.len(), 1);
    assert_eq!(
        *team.players[0].borrow(),
        Player::new(name.to_string(), time).unwrap()
    );
    Setting::reset();
}
