use crate::gun::{Gun, TypeOfGun};
use std::sync::{Arc, Mutex, OnceLock};
use std::fmt;

#[derive(Debug)]
struct SettingData {
    max_money_of_player: u32,
    default_money_of_player: u32,
    default_gun: Option<Arc<Gun>>,
}

impl fmt::Display for SettingData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Setting {{ max_money_of_player: {}, default_money_of_player: {}, default_gun: {:?} }}",
            self.max_money_of_player,
            self.default_money_of_player,
            self.default_gun
        )
    }
}

impl SettingData {
    pub fn default() -> Self {
        Self {
            max_money_of_player: 0,
            default_money_of_player: 0,
            default_gun: None,
        }
    }


    pub fn set_max_money_of_player(&mut self, max_money_of_player: u32) -> Result<(), String> {
        if max_money_of_player <= 0 {
            return Err("the max money of player should be greater than 0.".to_string());
        }
        self.max_money_of_player = max_money_of_player;
        Ok(())
    }

    pub fn set_default_money_of_player(&mut self, default_money_of_player: u32) -> Result<(), String> {
        if default_money_of_player <= 0 {
            return Err("the default money of player should be greater than 0.".to_string());
        }
        self.default_money_of_player = default_money_of_player;
        Ok(())
    }

    pub fn set_default_gun(&mut self, gun: Arc<Gun>) -> Result<(), String> {
        if gun.get_type_of() != TypeOfGun::Knife {
            return Err("the default gun should be knife type.".to_string());
        }
        self.default_gun = Some(gun);
        Ok(())
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
    pub fn get_max_money_of_player() -> u32 {
        Self::get_setting().lock().unwrap().max_money_of_player
    }

    pub fn set_max_money_of_player(max_money_of_player: u32) -> Result<(), String> {
        let mut setting = Self::get_setting().lock().unwrap();
        setting.set_max_money_of_player(max_money_of_player)
    }
    pub fn get_default_money_of_player() -> u32 {
        Self::get_setting().lock().unwrap().default_money_of_player
    }
    pub fn set_default_money_of_player(default_money_of_player: u32) -> Result<(), String> {
        let mut setting = Self::get_setting().lock().unwrap();
        setting.set_default_money_of_player(default_money_of_player)
    }
    pub fn get_default_gun() -> Option<Arc<Gun>> {
        Self::get_setting().lock().unwrap().default_gun.clone()
    }

    pub fn set_default_gun(gun: Arc<Gun>) -> Result<(), String> {
        let mut setting = Self::get_setting().lock().unwrap();
        setting.set_default_gun(gun)
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
        assert!(Setting::set_max_money_of_player(1000).is_ok());
        assert_eq!(Setting::get_max_money_of_player(), 1000);
    }
    #[test]
    pub fn test_set_max_money_of_player_fail() {
        assert!(Setting::set_max_money_of_player(0).is_err());
        assert_eq!(Setting::get_max_money_of_player(), 0);
    }
    #[test]
    pub fn test_get_and_set_default_money_of_player() {
        assert!(Setting::set_default_money_of_player(100).is_ok());
        assert_eq!(Setting::get_default_money_of_player(), 100);
    }
    #[test]
    pub fn test_set_default_money_of_player_fail() {
        assert!(Setting::set_default_money_of_player(0).is_err());
        assert_eq!(Setting::get_default_money_of_player(), 0);
    }
    #[test]
    pub fn test_get_and_set_default_gun() {
        let gun = Arc::new(Gun::new("knife".to_string(), 100, 20, 100, TypeOfGun::Knife));
        assert!(Setting::set_default_gun(gun.clone()).is_ok());
        assert_eq!(Setting::get_default_gun(), Some(gun));
    }
    #[test]
    pub fn test_set_default_gun_fail() {
        let gun = Arc::new(Gun::new("not knife".to_string(), 100, 20, 100, TypeOfGun::Pistol));
        assert!(Setting::set_default_gun(gun).is_err());
        assert_eq!(Setting::get_default_gun(), None);
    }
    #[test]
    pub fn test_get_setting() {
        let setting = Setting::get_setting();
        assert_eq!(setting.lock().unwrap().max_money_of_player, 0);
        assert_eq!(setting.lock().unwrap().default_money_of_player, 0);
        assert_eq!(setting.lock().unwrap().default_gun, None);
    }
    #[test]
    pub fn test_setting_display() {
        let setting = Setting::get_setting();
        assert_eq!(format!("{}", setting.lock().unwrap()), "{max_money_of_player: 0, default_money_of_player: 0, default_gun: None}");
    }
}