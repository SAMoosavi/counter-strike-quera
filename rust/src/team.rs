use crate::game_time::GameTime;
use crate::gun::Guns;
use crate::player::Player;
use crate::setting::Setting;
use std::rc::Rc;

#[cfg(test)]
mod test;

pub struct Team {
    players: Vec<Rc<Player>>,
    guns: Box<Guns>,
}

impl Team {
    pub fn default() -> Self {
        Self {
            players: vec![],
            guns: Box::new(Guns::new()),
        }
    }

    pub fn add_player(&mut self, name: &str, time: &GameTime) -> Result<(), String> {
        let max_money_of_player = Setting::get_max_money_of_player();
        if max_money_of_player == 0 {
            return Err("the maximum number of players doesn't set!".to_string());
        }
        if (max_money_of_player as usize) == self.players.len() {
            return Err("the team is full!".to_string());
        }

        if self.players.iter().any(|player| player.get_name() == name) {
            return Err(format!("player exist with same name: {}", name));
        }

        let player = Player::new(name.to_string(), time.clone())?;
        Ok(self.players.push(Rc::new(player)))
    }

    pub fn get_player(&self, name: &str) -> Option<Rc<Player>> {
        match self.players.iter().find(|player| player.get_name() == name) {
            Some(player) => Some(player.clone()),
            None => None,
        }
    }

    pub fn reset(&mut self) {
        self.players.iter_mut().for_each(|player| {
            let player = Rc::get_mut(player);
            player.unwrap().reset();
        });
    }

    pub fn does_live_player_exist(&self) -> bool {
        self.players.iter().any(|player| player.get_health() > 0)
    }

    pub fn number_of_live_player(&self) -> usize {
        self.players
            .iter()
            .filter(|player| player.get_health() > 0)
            .count()
    }

    pub fn won(&mut self) {
        self.add_money(Setting::get_won_team_money());
    }

    pub fn lose(&mut self) {
        self.add_money(Setting::get_lose_team_money());
    }

    fn add_money(&mut self, money: u32) {
        self.players.iter_mut().for_each(|player| {
            let player = Rc::get_mut(player);
            player.unwrap().add_money(money);
        });
    }
    pub fn fill_gun(&mut self, guns: Box<Guns>) {
        self.guns = guns
    }

    pub fn get_players(&self) -> Vec<Rc<Player>> {
        let mut players = self.players.clone();
        players.sort();
        players
    }

    pub fn get_guns(&self) -> &Box<Guns> {
        &self.guns
    }

    pub fn buy_gun(&mut self, player_name: &str, gun_name: &str) -> Result<(), String> {
        match self
            .players
            .iter_mut()
            .find(|player| player.get_name() == player_name)
        {
            Some(player) => {
                let gun = self.guns.get_gun(gun_name)?;
                let player = Rc::get_mut(player);
                player.unwrap().buy_gun(gun)
            }
            None => Err(format!("player with name {} does not find!", gun_name)),
        }
    }
}
