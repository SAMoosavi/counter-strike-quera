use std::rc::Rc;

use super::Team;
use crate::game_time::GameTime;
use crate::gun::{Gun, Guns, TypeOfGun};
use crate::player::Player;
use crate::setting::Setting;

fn fill_setting_for_create_player() -> Setting {
    let gun = Gun::new("knife".to_string(), 100, 10, 20, TypeOfGun::Knife);
    let mut setting = Setting::default();

    setting.default_gun = Some(Rc::new(gun));
    setting.default_money_of_player = 1000;
    setting.won_team_money = 2700;
    setting.lose_team_money = 2400;
    setting.max_time_buy = Some(GameTime::new(0, 45, 0));
    setting.did_time_of_player = Some(GameTime::new(0, 3, 0));
    setting
}

#[test]
pub fn test_default() {
    let team = Team::new("team".to_string());
    assert_eq!(team.guns, Box::new(Guns::new()));
    assert_eq!(team.players, vec!());
}

#[test]
pub fn test_add_player_should_be_return_error_when_does_not_set_max_number_of_players_in_setting() {
    let setting = fill_setting_for_create_player();
    let mut team = Team::new("team".to_string());
    let time = GameTime::new(0, 0, 1);

    let result = team.add_player("Player", &time, &setting);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "the maximum number of players doesn't set!"
    );
}

#[test]
pub fn test_add_player_should_be_return_error_when_team_is_full() {
    let mut setting = fill_setting_for_create_player();
    let mut team = Team::new("team".to_string());
    let time = GameTime::new(0, 0, 1);
    setting.max_money_of_player = 1;
    team.add_player("Player 1", &time, &setting).unwrap();

    let result = team.add_player("Player 2", &time, &setting);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "the team is full!");
}

#[test]
pub fn test_add_player_should_be_return_error_when_player_exists_with_same_name() {
    let mut setting = fill_setting_for_create_player();
    let mut team = Team::new("team".to_string());
    let time = GameTime::new(0, 0, 1);
    let name = "Player";
    setting.max_money_of_player = 2;
    team.add_player(name, &time, &setting).unwrap();

    let result = team.add_player(name, &time, &setting);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "you are already in this game");
}

#[test]
pub fn test_add_player_added_to_least_of_players() {
    let mut setting = fill_setting_for_create_player();
    let mut team = Team::new("team".to_string());
    setting.max_money_of_player = 2;
    let name = "Player";
    let time = GameTime::new(0, 0, 1);

    let result = team.add_player(name, &time, &setting);

    assert!(result.is_ok());
    assert_eq!(team.players.len(), 1);
    assert_eq!(
        *team.players[0].borrow(),
        Player::new(name.to_string(), time, &setting).unwrap()
    );
}
