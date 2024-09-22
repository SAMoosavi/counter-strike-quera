use crate::gun::{Gun, TypeOfGun};
use std::sync::{Arc, Mutex, OnceLock};
use std::fmt;

#[derive(Debug)]
struct SettingData {
    max_money_of_player: u32,
    default_money_of_player: u32,
    default_gun: Option<Arc<Gun>>,
    max_number_of_team_players: u32,
}

impl fmt::Display for SettingData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Setting {{max_money_of_player: {}, default_money_of_player: {}, default_gun: {:?}, max_number_of_team_players: {}}}",
            self.max_money_of_player,
            self.default_money_of_player,
            self.default_gun,
            self.max_number_of_team_players
        )
    }
}

impl SettingData {
    pub fn default() -> Self {
        Self {
            max_money_of_player: 0,
            default_money_of_player: 0,
            default_gun: None,
            max_number_of_team_players: 0,
        }
    }


    #[allow(dead_code)]
    pub fn set_max_money_of_player(&mut self, max_money_of_player: u32) -> Result<(), String> {
        if max_money_of_player <= 0 {
            return Err("the max money of player should be greater than 0.".to_string());
        }
        self.max_money_of_player = max_money_of_player;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_default_money_of_player(&mut self, default_money_of_player: u32) -> Result<(), String> {
        if default_money_of_player <= 0 {
            return Err("the default money of player should be greater than 0.".to_string());
        }
        self.default_money_of_player = default_money_of_player;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_default_gun(&mut self, gun: Arc<Gun>) -> Result<(), String> {
        if gun.get_type_of() != TypeOfGun::Knife {
            return Err("the default gun should be knife type.".to_string());
        }
        self.default_gun = Some(gun);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_max_number_of_team_players(&mut self, max_number_of_team_players: u32) -> Result<(), String> {
        if max_number_of_team_players == 0 {
            return Err("the max_number_of_team_players should be positive!".to_string());
        }
        Ok(self.max_number_of_team_players = max_number_of_team_players)
    }
    #[allow(dead_code)]
    pub fn get_max_number_of_team_players(&self) -> u32 {
        self.max_number_of_team_players
    }
    #[cfg(test)]
    pub fn reset(&mut self) {
        self.max_money_of_player = 0;
        self.default_money_of_player = 0;
        self.default_gun = None;
    }
}

#[cfg(test)]
mod tests_setting_data {
    use std::sync::Arc;
    use super::SettingData;
    use crate::gun::{Gun, TypeOfGun};
    #[test]
    pub fn test_default_setting_data() {
        let setting = SettingData::default();
        assert_eq!(setting.max_money_of_player, 0);
        assert_eq!(setting.default_money_of_player, 0);
        assert_eq!(setting.default_gun, None);
    }
    #[test]
    pub fn test_set_max_money_of_player() {
        let mut setting = SettingData::default();
        assert!(setting.set_max_money_of_player(1000).is_ok());
        assert_eq!(setting.max_money_of_player, 1000);
    }
    #[test]
    pub fn test_set_max_money_of_player_fail_when_the_money_is_zero() {
        let mut setting = SettingData::default();
        assert!(setting.set_max_money_of_player(0).is_err());
        assert_eq!(setting.max_money_of_player, 0);
    }
    #[test]
    pub fn test_set_default_money_of_player() {
        let mut setting = SettingData::default();
        assert!(setting.set_default_money_of_player(100).is_ok());
        assert_eq!(setting.default_money_of_player, 100);
    }
    #[test]
    pub fn test_set_default_money_of_player_fail() {
        let mut setting = SettingData::default();
        assert!(setting.set_default_money_of_player(0).is_err());
        assert_eq!(setting.default_money_of_player, 0);
    }
    #[test]
    pub fn test_set_default_gun() {
        let gun = Arc::new(Gun::new("knife".to_string(), 100, 20, 100, TypeOfGun::Knife));
        let mut setting = SettingData::default();
        assert!(setting.set_default_gun(gun.clone()).is_ok());
        assert_eq!(setting.default_gun, Some(gun));
    }
    #[test]
    pub fn test_set_default_gun_fail() {
        let gun = Arc::new(Gun::new("not knife".to_string(), 100, 20, 100, TypeOfGun::Pistol));
        let mut setting = SettingData::default();
        assert!(setting.set_default_gun(gun).is_err());
        assert_eq!(setting.default_gun, None);
    }
}

pub struct Setting {}

impl Setting {
    fn get_setting() -> &'static Mutex<SettingData> {
        static SETTING: OnceLock<Mutex<SettingData>> = OnceLock::new();
        SETTING.get_or_init(|| Mutex::new(SettingData::default()))
    }
    #[allow(dead_code)]
    pub fn get_max_money_of_player() -> u32 {
        Self::get_setting().lock().unwrap().max_money_of_player
    }

    #[allow(dead_code)]
    pub fn set_max_money_of_player(max_money_of_player: u32) -> Result<(), String> {
        let mut setting = Self::get_setting().lock().unwrap();
        setting.set_max_money_of_player(max_money_of_player)
    }
    #[allow(dead_code)]
    pub fn get_default_money_of_player() -> u32 {
        Self::get_setting().lock().unwrap().default_money_of_player
    }
    #[allow(dead_code)]
    pub fn set_default_money_of_player(default_money_of_player: u32) -> Result<(), String> {
        let mut setting = Self::get_setting().lock().unwrap();
        setting.set_default_money_of_player(default_money_of_player)
    }
    pub fn get_default_gun() -> Option<Arc<Gun>> {
        Self::get_setting().lock().unwrap().default_gun.clone()
    }

    #[allow(dead_code)]
    pub fn set_default_gun(gun: Arc<Gun>) -> Result<(), String> {
        let mut setting = Self::get_setting().lock().unwrap();
        setting.set_default_gun(gun)
    }

    #[allow(dead_code)]
    pub fn set_max_number_of_team_players(max_number_of_team_players: u32) -> Result<(), String> {
        let mut setting = Self::get_setting().lock().unwrap();
        setting.set_max_number_of_team_players(max_number_of_team_players)
    }

    #[allow(dead_code)]
    pub fn get_max_number_of_team_players() -> u32 {
        let setting = Self::get_setting().lock().unwrap();
        setting.get_max_number_of_team_players()
    }

    #[cfg(test)]
    pub fn reset() {
        Self::get_setting().lock().unwrap().reset();
    }
}

impl fmt::Display for Setting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:}", Self::get_setting().lock().unwrap())
    }
}

#[cfg(test)]
mod tests_setting {
    use super::Setting;
    use crate::gun::{Gun, TypeOfGun};
    use std::sync::Arc;
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
        let gun = Arc::new(Gun::new("knife".to_string(), 100, 20, 100, TypeOfGun::Knife));
        Setting::reset();
        assert!(Setting::set_default_gun(gun.clone()).is_ok());
        assert_eq!(Setting::get_default_gun(), Some(gun));
        Setting::reset();
    }
    #[test]
    pub fn test_set_default_gun_fail() {
        let gun = Arc::new(Gun::new("not knife".to_string(), 100, 20, 100, TypeOfGun::Pistol));
        Setting::reset();
        assert!(Setting::set_default_gun(gun).is_err());
        assert_eq!(Setting::get_default_gun(), None);
        Setting::reset();
    }
    #[test]
    pub fn test_get_setting() {
        Setting::reset();
        let setting = Setting::get_setting();
        assert_eq!(setting.lock().unwrap().max_money_of_player, 0);
        assert_eq!(setting.lock().unwrap().default_money_of_player, 0);
        assert_eq!(setting.lock().unwrap().default_gun, None);
        Setting::reset();
    }
    #[test]
    pub fn test_setting_display() {
        Setting::reset();
        let setting = Setting::get_setting();
        assert_eq!(format!("{}", setting.lock().unwrap()), "Setting {max_money_of_player: 0, default_money_of_player: 0, default_gun: None, max_number_of_team_players: 0}");
        Setting::reset();
    }
}