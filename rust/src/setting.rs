use crate::{
    game_time::GameTime,
    gun::{Gun, TypeOfGun},
};
use std::{cell::RefCell, fmt, rc::Rc};

#[cfg(test)]
mod test;

#[derive(Debug, Default)]
struct SettingData {
    max_money_of_player: u32,
    default_money_of_player: u32,
    default_gun: Option<Rc<Gun>>,
    max_number_of_team_players: u32,
    won_team_money: u32,
    lose_team_money: u32,
    friendly_fire: bool,
    max_time_buy: Option<GameTime>,
}

impl fmt::Display for SettingData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Setting {{max_money_of_player: {}, default_money_of_player: {}, default_gun: {:?}, max_number_of_team_players: {}, won_team_money: {}, lose_team_money: {}, friendly_fire: {}, max_time_buy: {:?}}}",
            self.max_money_of_player,
            self.default_money_of_player,
            self.default_gun,
            self.max_number_of_team_players,
            self.won_team_money,
            self.lose_team_money,
            self.friendly_fire,
            self.max_time_buy
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
            friendly_fire: false,
            max_time_buy: None,
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
        self.friendly_fire = false;
        self.max_time_buy = None;
    }
}

pub struct Setting {}

thread_local! {
    static SETTING: RefCell<SettingData> = RefCell::new(SettingData::default());
}

impl Setting {
    pub fn get_max_money_of_player() -> u32 {
        SETTING.with(|x| x.borrow().max_money_of_player)
    }

    pub fn set_max_money_of_player(max_money_of_player: u32) -> Result<(), String> {
        if max_money_of_player <= 0 {
            return Err("the max money of player should be greater than 0.".to_string());
        }
        Ok(SETTING.with(|x| x.borrow_mut().max_money_of_player = max_money_of_player))
    }

    pub fn get_default_money_of_player() -> u32 {
        SETTING.with(|x| x.borrow().default_money_of_player)
    }
    pub fn set_default_money_of_player(default_money_of_player: u32) -> Result<(), String> {
        if default_money_of_player <= 0 {
            return Err("the default money of player should be greater than 0.".to_string());
        }
        Ok(SETTING.with(|x| x.borrow_mut().default_money_of_player = default_money_of_player))
    }

    pub fn get_default_gun() -> Option<Rc<Gun>> {
        SETTING.with(|x| x.borrow().default_gun.clone())
    }

    pub fn set_default_gun(gun: &Rc<Gun>) -> Result<(), String> {
        match gun.get_type_of() {
            TypeOfGun::Knife => {
                Ok(SETTING.with(|x| x.borrow_mut().default_gun = Some(gun.clone())))
            }
            _ => Err("the default gun should be knife type.".to_string()),
        }
    }

    pub fn set_max_number_of_team_players(max_number_of_team_players: u32) -> Result<(), String> {
        if max_number_of_team_players == 0 {
            return Err("the max_number_of_team_players should be positive!".to_string());
        }
        Ok(
            SETTING
                .with(|x| x.borrow_mut().max_number_of_team_players = max_number_of_team_players),
        )
    }

    pub fn get_max_number_of_team_players() -> u32 {
        SETTING.with(|x| x.borrow().max_number_of_team_players)
    }

    pub fn set_won_team_money(won_team_money: u32) -> Result<(), String> {
        if won_team_money == 0 {
            return Err("the won_team_money should be positive!".to_string());
        }
        Ok(SETTING.with(|x| x.borrow_mut().won_team_money = won_team_money))
    }

    pub fn get_won_team_money() -> u32 {
        SETTING.with(|x| x.borrow().won_team_money)
    }

    pub fn set_lose_team_money(lose_team_money: u32) -> Result<(), String> {
        if lose_team_money == 0 {
            return Err("the lose_team_money should be positive!".to_string());
        }
        Ok(SETTING.with(|x| x.borrow_mut().lose_team_money = lose_team_money))
    }

    pub fn get_lose_team_money() -> u32 {
        SETTING.with(|x| x.borrow().lose_team_money)
    }

    pub fn set_friendly_fire(friendly_fire: bool) {
        SETTING.with(|x| x.borrow_mut().friendly_fire = friendly_fire)
    }

    pub fn get_friendly_fire() -> bool {
        SETTING.with(|x| x.borrow().friendly_fire)
    }

    pub fn get_max_time_buy() -> Option<GameTime> {
        SETTING.with(|x| x.borrow().max_time_buy.clone())
    }

    pub fn set_max_time_buy(max_time_buy: &GameTime) -> Result<(), String> {
        if !(max_time_buy > &GameTime::new(0, 0, 0)) {
            return Err("the max_time_buy should not be None!".to_string());
        }
        SETTING.with(|x| x.borrow_mut().max_time_buy = Some(max_time_buy.clone()));
        Ok(())
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
        write!(f, "{}", SETTING.take())
    }
}
