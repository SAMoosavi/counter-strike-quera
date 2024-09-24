use super::Setting;
use crate::gun::{Gun, TypeOfGun};
use std::rc::Rc;

#[test]
pub fn test_get_and_set_max_money_of_player() {
    Setting::reset();
    assert!(Setting::set_max_money_of_player(1000).is_ok());
    assert_eq!(Setting::get_max_money_of_player(), 1000);
    Setting::reset();
}
#[test]
pub fn test_set_max_money_of_player_fail() {
    Setting::reset();
    assert!(Setting::set_max_money_of_player(0).is_err());
    assert_eq!(Setting::get_max_money_of_player(), 0);
    Setting::reset();
}
#[test]
pub fn test_get_and_set_default_money_of_player() {
    Setting::reset();
    assert!(Setting::set_default_money_of_player(100).is_ok());
    assert_eq!(Setting::get_default_money_of_player(), 100);
    Setting::reset();
}
#[test]
pub fn test_set_default_money_of_player_fail() {
    Setting::reset();
    assert!(Setting::set_default_money_of_player(0).is_err());
    assert_eq!(Setting::get_default_money_of_player(), 0);
    Setting::reset();
}
#[test]
pub fn test_get_and_set_default_gun() {
    let gun = Rc::new(Gun::new(
        "knife".to_string(),
        100,
        20,
        100,
        TypeOfGun::Knife,
    ));
    Setting::reset();
    assert!(Setting::set_default_gun(gun.clone()).is_ok());
    assert_eq!(Setting::get_default_gun(), Some(gun));
    Setting::reset();
}
#[test]
pub fn test_set_default_gun_fail() {
    let gun = Rc::new(Gun::new(
        "not knife".to_string(),
        100,
        20,
        100,
        TypeOfGun::Pistol,
    ));
    Setting::reset();
    assert!(Setting::set_default_gun(gun).is_err());
    assert_eq!(Setting::get_default_gun(), None);
    Setting::reset();
}
#[test]
pub fn test_get_setting() {
    Setting::reset();
    let setting = Setting::get_setting();
    assert_eq!(setting.max_money_of_player, 0);
    assert_eq!(setting.default_money_of_player, 0);
    assert_eq!(setting.default_gun, None);
    Setting::reset();
}
#[test]
pub fn test_setting_display() {
    Setting::reset();
    let setting = Setting::get_setting();
    assert_eq!(format!("{}", setting), "Setting {max_money_of_player: 0, default_money_of_player: 0, default_gun: None, max_number_of_team_players: 0}");
    Setting::reset();
}
