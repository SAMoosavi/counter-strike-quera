use crate::{game_time::GameTime, gun::Gun};
use std::{fmt, rc::Rc};

#[derive(Debug, Default)]
pub struct Setting {
    pub max_money_of_player: u32,
    pub default_money_of_player: u32,
    pub default_gun: Option<Rc<Gun>>,
    pub max_number_of_team_players: u32,
    pub won_team_money: u32,
    pub lose_team_money: u32,
    pub friendly_fire: bool,
    pub max_time_buy: Option<GameTime>,
    pub did_time_of_player: Option<GameTime>,
}

impl fmt::Display for Setting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Setting {{max_money_of_player: {}, default_money_of_player: {}, default_gun: {:?}, max_number_of_team_players: {}, won_team_money: {}, lose_team_money: {}, friendly_fire: {}, max_time_buy: {:?}, did_time_of_player: {:?}}}",
            self.max_money_of_player,
            self.default_money_of_player,
            self.default_gun,
            self.max_number_of_team_players,
            self.won_team_money,
            self.lose_team_money,
            self.friendly_fire,
            self.max_time_buy,
            self.did_time_of_player
        )
    }
}
