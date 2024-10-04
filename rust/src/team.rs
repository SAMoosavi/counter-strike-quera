use crate::game_time::GameTime;
use crate::gun::{Gun, Guns, TypeOfGun};
use crate::player::Player;
use crate::setting::Setting;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod test;

pub struct Team {
    players: Vec<Rc<RefCell<Player>>>,
    guns: Box<Guns>,
    name: String,
}

impl Team {
    pub fn new(name: String) -> Self {
        Self {
            name,
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

        if self.has_player(name) {
            return Err("you are already in this game".to_string());
        }

        let player = Player::new(name.to_string(), time.clone())?;
        Ok(self.players.push(Rc::new(RefCell::new(player))))
    }

    fn get_player(&self, name: &str) -> Option<Rc<RefCell<Player>>> {
        self.players
            .iter()
            .find(|player| player.borrow().get_name() == name)
            .map(|player| player.clone())
            .clone()
    }

    pub fn has_player(&self, name: &str) -> bool {
        self.get_player(name).is_some()
    }

    pub fn is_player_alive(&self, name: &str) -> Result<bool, String> {
        match self.get_player(name) {
            Some(player) => Ok(player.borrow().is_alive()),
            None => Err("invalid username".to_string()),
        }
    }

    pub fn get_players_gun(&self, name: &str, type_of_gun: &TypeOfGun) -> Result<Rc<Gun>, String> {
        match self.get_player(name) {
            Some(player) => match player.borrow().get_gun_with_type(type_of_gun) {
                Some(gun) => Ok(gun.clone()),
                None => Err("no such gun".to_string()),
            },
            None => Err("invalid username".to_string()),
        }
    }

    pub fn shut(&mut self, name: &str, damage: u32) -> Result<u32, String> {
        match self.get_player(name) {
            Some(player) => Ok(player.borrow_mut().shut(damage)?),
            None => Err("invalid username".to_string()),
        }
    }

    pub fn add_kill_of_player(
        &mut self,
        name: &str,
        type_of_gun: &TypeOfGun,
    ) -> Result<(), String> {
        match self.get_player(name) {
            Some(player) => player.borrow_mut().add_kill(type_of_gun),
            None => Err("invalid username".to_string()),
        }
    }

    pub fn reset(&mut self) {
        self.players.iter_mut().for_each(|player| {
            player.borrow_mut().reset();
        });
    }

    pub fn does_live_player_exist(&self) -> bool {
        self.players
            .iter()
            .any(|player| player.borrow().get_health() > 0)
    }

    pub fn number_of_live_player(&self) -> usize {
        self.players
            .iter()
            .filter(|player| player.borrow().get_health() > 0)
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
            player.borrow_mut().add_money(money);
        });
    }

    pub fn get_money(&self, name: &str) -> Result<u32, String> {
        match self.get_player(name) {
            Some(player) => Ok(player.borrow().get_money()),
            None => Err("invalid username".to_string()),
        }
    }

    pub fn get_health(&self, name: &str) -> Result<u32, String> {
        match self.get_player(name) {
            Some(player) => Ok(player.borrow().get_health()),
            None => Err("invalid username".to_string()),
        }
    }

    pub fn fill_gun(&mut self, guns: Box<Guns>) {
        self.guns = guns
    }

    pub fn get_players(&self) -> Vec<Rc<RefCell<Player>>> {
        let mut players = self.players.clone();
        players.sort();
        players
    }

    pub fn get_guns(&self) -> &Guns {
        &self.guns
    }

    pub fn buy_gun(&mut self, player_name: &str, gun_name: &str) -> Result<(), String> {
        match self
            .players
            .iter_mut()
            .find(|player| player.borrow().get_name() == player_name)
        {
            Some(player) => {
                if !player.borrow().is_alive() {
                    return Err("deads can not buy".to_string());
                }
                let gun = self.guns.get_gun(gun_name)?;
                player.borrow_mut().buy_gun(gun)
            }
            None => Err("invalid username".to_string()),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn score_board(&self) -> String {
        let players: Vec<String> = self
            .get_players()
            .iter()
            .enumerate()
            .map(|element| format!("{} {}", element.0 + 1, element.1.borrow().to_string()))
            .collect();

        format!("{}-Players:\n{}", self.name, players.join("\n"))
    }
}
