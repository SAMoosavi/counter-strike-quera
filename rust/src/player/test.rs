use super::Player;
use crate::game_time::GameTime;
use crate::gun::{Gun, TypeOfGun};
use crate::setting::Setting;
use std::rc::Rc;

fn create_player() -> Player {
    let gun = Gun::new("knife".to_string(), 100, 10, 20, TypeOfGun::Knife);
    let mut setting = Setting::default();
    setting.default_gun = Some(Rc::new(gun));
    setting.default_money_of_player = 1000;
    setting.won_team_money = 2700;
    setting.lose_team_money = 2400;
    setting.max_time_buy = Some(GameTime::new(0, 45, 0));
    setting.did_time_of_player = Some(GameTime::new(0, 3, 0));

    let time = GameTime::new(0, 0, 10);
    Player::new("p1".to_string(), time, &setting).unwrap()
}

#[test]
pub fn new_player_when_get_a_gun_that_type_of_it_is_not_knife_should_be_return_error() {
    let setting = Setting::default();
    let time = GameTime::new(0, 0, 10);
    assert!(Player::new("p1".to_string(), time, &setting).is_err());
}

#[test]
pub fn new_player_when_get_a_gun_that_type_of_it_is_knife_should_be_return_ok() {
    let gun = Gun::new("knife".to_string(), 100, 10, 20, TypeOfGun::Knife);
    let mut setting = Setting::default();
    setting.default_gun = Some(Rc::new(gun));
    setting.default_money_of_player = 1000;
    setting.won_team_money = 2700;
    setting.lose_team_money = 2400;
    setting.max_time_buy = Some(GameTime::new(0, 45, 0));
    setting.did_time_of_player = Some(GameTime::new(0, 3, 0));

    let time = GameTime::new(0, 0, 10);
    assert!(Player::new("p1".to_string(), time, &setting).is_ok());
}

#[test]
pub fn shut_did_player() {
    let mut player: Player = create_player();
    player.health = 0;
    let result = player.shut(10);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "p1 did!".to_string());
}

#[test]
pub fn player_should_be_live_when_its_health_has_more_then_shut() {
    let mut player: Player = create_player();
    let result = player.shut(10);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 90);
    assert_eq!(player.health, 90);
    assert_eq!(player.death, 0);
}

#[test]
pub fn player_should_be_dead_when_its_health_has_lese_then_shut() {
    let mut player: Player = create_player();
    player.health = 5;
    let result = player.shut(10);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
    assert_eq!(player.health, 0);
    assert_eq!(player.death, 1);
}

#[test]
pub fn player_can_not_buy_gun_when_does_not_have_enough_money() {
    let mut player: Player = create_player();
    player.money = 10;

    let gun = Rc::new(Gun::new(
        "new gun".to_string(),
        100,
        10,
        20,
        crate::gun::TypeOfGun::Heavy,
    ));

    let result = player.buy_gun(gun);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "no enough money");
}

#[test]
pub fn player_can_not_buy_gun_when_exist_its_type() {
    let mut player: Player = create_player();
    player.money = 1000;

    let heavy_gun_1 = Rc::new(Gun::new(
        "heavy gun 1".to_string(),
        100,
        10,
        20,
        crate::gun::TypeOfGun::Heavy,
    ));

    let heavy_gun_2 = Rc::new(Gun::new(
        "heavy gun 2".to_string(),
        100,
        10,
        20,
        crate::gun::TypeOfGun::Heavy,
    ));

    let result = player.buy_gun(heavy_gun_1);

    assert!(result.is_ok());

    let result = player.buy_gun(heavy_gun_2);
    assert!(result.is_err());

    assert_eq!(result.unwrap_err(), "you have a heavy");
}

#[test]
pub fn player_can_buy_gun_when_does_not_have_its_type_and_enough_money() {
    let mut player: Player = create_player();
    player.money = 1000;

    let gun = Rc::new(Gun::new(
        "heavy gun 1".to_string(),
        100,
        10,
        20,
        crate::gun::TypeOfGun::Heavy,
    ));

    let result = player.buy_gun(gun.clone());

    assert!(result.is_ok());
    assert_eq!(player.money, 900);
    assert_eq!(player.guns.get(&crate::gun::TypeOfGun::Heavy), Some(&gun));
}

#[test]
pub fn players_health_set_100_when_call_reset_function() {
    let mut player = create_player();
    player.health = 30;

    player.reset();
    assert_eq!(player.health, 100);
}

#[test]
pub fn the_add_kill_func_should_be_return_error_when_player_is_did() {
    let mut player = create_player();
    player.health = 0;

    let result = player.add_kill(&crate::gun::TypeOfGun::Knife);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "the p1 is did!");
}

#[test]
pub fn the_add_kill_func_should_be_return_error_when_player_does_not_have_this_type_of_gun() {
    let mut player = create_player();

    let result = player.add_kill(&crate::gun::TypeOfGun::Heavy);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "the p1 does not have heavy gun!");
}

#[test]
pub fn the_add_kill_func_should_be_add_kill_number_and_money() {
    let mut player = create_player();
    player.money = 1100;
    let gun = Rc::new(Gun::new(
        "heavy gun".to_string(),
        100,
        10,
        20,
        crate::gun::TypeOfGun::Heavy,
    ));

    player.buy_gun(gun.clone()).unwrap();

    let result = player.add_kill(&crate::gun::TypeOfGun::Heavy);

    assert!(result.is_ok());
    assert_eq!(player.kills, 1);
    assert_eq!(player.money, 1020);
}

#[test]
pub fn the_add_money_should_be_set_money_sum_of_current_money_and_added_money_when_the_sum_is_less_than_max_money_of_player(
) {
    let mut player = create_player();
    player.money = 100;
    let mut setting = Setting::default();
    setting.max_money_of_player = 1000;

    player.add_money(200, &setting);

    assert_eq!(player.money, 300);
}
#[test]
pub fn the_add_money_should_be_set_max_money_when_the_sum_is_more_than_max_money_of_player() {
    let mut player = create_player();
    player.money = 100;
    let mut setting = Setting::default();
    setting.max_money_of_player = 1000;

    player.add_money(1100, &setting);

    assert_eq!(player.money, 1000);
}

#[test]
pub fn test_cmp() {
    let gun = Gun::new("knife".to_string(), 100, 10, 20, TypeOfGun::Knife);
    let mut setting = Setting::default();
    setting.default_money_of_player = 1000;
    setting.default_gun = Some(Rc::new(gun));
    setting.won_team_money = 2700;
    setting.lose_team_money = 2400;
    setting.max_time_buy = Some(GameTime::new(0, 45, 0));
    setting.did_time_of_player = Some(GameTime::new(0, 3, 0));

    let p1 = Player::new("p1".to_string(), GameTime::new(0, 0, 10), &setting).unwrap();
    let p2 = Player::new("p2".to_string(), GameTime::new(0, 0, 20), &setting).unwrap();
    assert!(p1 > p2);
    let p3 = Player::new("p3".to_string(), GameTime::new(0, 0, 10), &setting).unwrap();
    assert_eq!(p1, p3);
    let mut p4 = Player::new("p4".to_string(), GameTime::new(0, 0, 10), &setting).unwrap();
    p4.kills = 1;
    assert!(p4 > p1);
    let mut p5 = Player::new("p4".to_string(), GameTime::new(0, 0, 10), &setting).unwrap();
    p5.death = 1;
    assert!(p5 < p1);
}
