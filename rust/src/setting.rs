use crate::gun::{Gun, TypeOfGun};
use std::{
    fmt,
    sync::{Arc, Mutex, OnceLock},
};

#[derive(Debug)]
struct SettingData {
    max_money_of_player: u32,
    default_money_of_player: u32,
    default_gun: Option<Arc<Gun>>,
    max_number_of_team_players: u32,
    won_team_money: u32,
    lose_team_money: u32,
}

impl fmt::Display for SettingData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Setting {{max_money_of_player: {}, default_money_of_player: {}, default_gun: {:?}, max_number_of_team_players: {}, won_team_money: {}, lose_team_money: {}}}",
            self.max_money_of_player,
            self.default_money_of_player,
            self.default_gun,
            self.max_number_of_team_players,
            self.won_team_money,
            self.lose_team_money
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
            won_team_money: 0,
            lose_team_money: 0,
        }
    }

    #[cfg(test)]
    pub fn reset(&mut self) {
        self.max_money_of_player = 0;
        self.default_money_of_player = 0;
        self.default_gun = None;
        self.max_number_of_team_players = 0;
        self.won_team_money = 0;
        self.lose_team_money = 0;
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
        if max_money_of_player <= 0 {
            return Err("the max money of player should be greater than 0.".to_string());
        }
        Ok(Self::get_setting().lock().unwrap().max_money_of_player = max_money_of_player)
    }

    #[allow(dead_code)]
    pub fn get_default_money_of_player() -> u32 {
        Self::get_setting().lock().unwrap().default_money_of_player
    }
    #[allow(dead_code)]
    pub fn set_default_money_of_player(default_money_of_player: u32) -> Result<(), String> {
        if default_money_of_player <= 0 {
            return Err("the default money of player should be greater than 0.".to_string());
        }

        Ok(Self::get_setting().lock().unwrap().default_money_of_player = default_money_of_player)
    }
    pub fn get_default_gun() -> Option<Arc<Gun>> {
        Self::get_setting().lock().unwrap().default_gun.clone()
    }

    #[allow(dead_code)]
    pub fn set_default_gun(gun: Arc<Gun>) -> Result<(), String> {
        if gun.get_type_of() != TypeOfGun::Knife {
            return Err("the default gun should be knife type.".to_string());
        }

        Ok(Self::get_setting().lock().unwrap().default_gun = Some(gun))
    }

    #[allow(dead_code)]
    pub fn set_max_number_of_team_players(max_number_of_team_players: u32) -> Result<(), String> {
        if max_number_of_team_players == 0 {
            return Err("the max_number_of_team_players should be positive!".to_string());
        }
        Ok(Self::get_setting()
            .lock()
            .unwrap()
            .max_number_of_team_players = max_number_of_team_players)
    }

    #[allow(dead_code)]
    pub fn get_max_number_of_team_players() -> u32 {
        Self::get_setting()
            .lock()
            .unwrap()
            .max_number_of_team_players
    }

    #[allow(dead_code)]
    pub fn set_won_team_money(won_team_money: u32) -> Result<(), String> {
        if won_team_money == 0 {
            return Err("the won_team_money should be positive!".to_string());
        }
        Ok(Self::get_setting().lock().unwrap().won_team_money = won_team_money)
    }

    #[allow(dead_code)]
    pub fn get_won_team_money() -> u32 {
        Self::get_setting().lock().unwrap().won_team_money
    }

    #[allow(dead_code)]
    pub fn set_lose_team_money(lose_team_money: u32) -> Result<(), String> {
        if lose_team_money == 0 {
            return Err("the lose_team_money should be positive!".to_string());
        }
        Ok(Self::get_setting().lock().unwrap().lose_team_money = lose_team_money)
    }

    #[allow(dead_code)]
    pub fn get_lose_team_money() -> u32 {
        Self::get_setting().lock().unwrap().lose_team_money
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
mod tests_setting;
