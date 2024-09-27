use crate::gun::{Gun, TypeOfGun};
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, Default)]
struct SettingData {
    max_money_of_player: u32,
    default_money_of_player: u32,
    default_gun: Option<Rc<Gun>>,
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

thread_local! {
    static SETTING: RefCell<SettingData> = RefCell::new(SettingData::default());
}

impl Setting {
    #[allow(dead_code)]
    pub fn get_max_money_of_player() -> u32 {
        SETTING.with(|x| x.borrow().max_money_of_player)
    }

    #[allow(dead_code)]
    pub fn set_max_money_of_player(max_money_of_player: u32) -> Result<(), String> {
        if max_money_of_player <= 0 {
            return Err("the max money of player should be greater than 0.".to_string());
        }
        Ok(SETTING.with(|x| x.borrow_mut().max_money_of_player = max_money_of_player))
    }


    pub fn get_default_money_of_player() -> u32 {
        SETTING.with(|x| x.borrow().default_money_of_player)
    }
    #[allow(dead_code)]
    pub fn set_default_money_of_player(default_money_of_player: u32) -> Result<(), String> {
        if default_money_of_player <= 0 {
            return Err("the default money of player should be greater than 0.".to_string());
        }
        Ok(SETTING.with(|x| x.borrow_mut().default_money_of_player = default_money_of_player))
    }

    pub fn get_default_gun() -> Option<Rc<Gun>> {
        SETTING.with(|x| x.borrow().default_gun.clone())
    }

    #[allow(dead_code)]
    pub fn set_default_gun(gun: Rc<Gun>) -> Result<(), String> {
        match gun.get_type_of() {
            TypeOfGun::Knife => Ok(SETTING.with(|x| x.borrow_mut().default_gun = Some(gun))),
            _ => Err("the default gun should be knife type.".to_string()),
        }
    }

    #[allow(dead_code)]
    pub fn set_max_number_of_team_players(max_number_of_team_players: u32) -> Result<(), String> {
        if max_number_of_team_players == 0 {
            return Err("the max_number_of_team_players should be positive!".to_string());
        }
        Ok(SETTING.with(|x| x.borrow_mut().max_number_of_team_players = max_number_of_team_players))
    }

    #[allow(dead_code)]
    pub fn get_max_number_of_team_players() -> u32 {
        SETTING.with(|x| x.borrow().max_number_of_team_players)
    }

    #[allow(dead_code)]
    pub fn set_won_team_money(won_team_money: u32) -> Result<(), String> {
        if won_team_money == 0 {
            return Err("the won_team_money should be positive!".to_string());
        }
        Ok(SETTING.with(|x| x.borrow_mut().won_team_money = won_team_money))
    }

    #[allow(dead_code)]
    pub fn get_won_team_money() -> u32 {
        SETTING.with(|x| x.borrow().won_team_money)
    }

    #[allow(dead_code)]
    pub fn set_lose_team_money(lose_team_money: u32) -> Result<(), String> {
        if lose_team_money == 0 {
            return Err("the lose_team_money should be positive!".to_string());
        }
        Ok(SETTING.with(|x| x.borrow_mut().lose_team_money = lose_team_money))
    }

    #[allow(dead_code)]
    pub fn get_lose_team_money() -> u32 {
        SETTING.with(|x| x.borrow().lose_team_money)
    }
}

#[cfg(test)]
impl Setting {
    pub fn reset() {
        SETTING.with_borrow_mut(|x| x.reset());
    }

    fn get_setting() -> SettingData {
        SETTING.take()
    }
}

impl fmt::Display for Setting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:}", SETTING.take())
    }
}

#[cfg(test)]
mod tests_setting;
