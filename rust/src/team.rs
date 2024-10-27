use crate::game_time::GameTime;
use crate::gun::{Gun, Guns, TypeOfGun};
use crate::player::Player;
use crate::setting::Setting;
use itertools::Itertools;
use std::rc::Rc;

#[cfg(test)]
mod test;

pub struct Team {
    players: Vec<Player>,
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

    pub fn add_player(
        &mut self,
        name: &str,
        time: &GameTime,
        setting: &Setting,
    ) -> Result<(), String> {
        let max_money_of_player = setting.max_money_of_player;
        if max_money_of_player == 0 {
            return Err("the maximum number of players doesn't set!".to_string());
        }
        if (max_money_of_player as usize) == self.players.len() {
            return Err("the team is full!".to_string());
        }

        if self.has_player(name) {
            return Err("you are already in this game".to_string());
        }

        let player = Player::new(name.to_string(), time.clone(), setting)?;
        self.players.push(player);
        Ok(())
    }

    fn get_player(&self, name: &str) -> Result<&Player, String> {
        match self.players.iter().find(|player| player.get_name() == name) {
            Some(player) => Ok(player),
            None => Err("invalid username".to_string()),
        }
    }

    fn get_mut_player(&mut self, name: &str) -> Result<&mut Player, String> {
        match self
            .players
            .iter_mut()
            .find(|player| player.get_name() == name)
        {
            Some(player) => Ok(player),
            None => Err("invalid username".to_string()),
        }
    }

    pub fn has_player(&self, name: &str) -> bool {
        self.get_player(name).is_ok()
    }

    pub fn is_player_alive(&self, name: &str) -> Result<bool, String> {
        Ok(self.get_player(name)?.is_alive())
    }

    pub fn get_players_gun(&self, name: &str, type_of_gun: &TypeOfGun) -> Result<Rc<Gun>, String> {
        match self.get_player(name)?.get_gun_with_type(type_of_gun) {
            Some(gun) => Ok(gun.clone()),
            None => Err("no such gun".to_string()),
        }
    }

    pub fn shut(&mut self, name: &str, damage: u32) -> Result<u32, String> {
        self.get_mut_player(name)?.shut(damage)
    }

    pub fn add_kill_of_player(
        &mut self,
        name: &str,
        type_of_gun: &TypeOfGun,
    ) -> Result<(), String> {
        self.get_mut_player(name)?.add_kill(type_of_gun)
    }

    pub fn reset(&mut self) {
        self.players.iter_mut().for_each(|player| {
            player.reset();
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

    pub fn won(&mut self, setting: &Setting) {
        self.add_money(setting.won_team_money, setting);
    }

    pub fn lose(&mut self, setting: &Setting) {
        self.add_money(setting.lose_team_money, setting);
    }

    fn add_money(&mut self, money: u32, setting: &Setting) {
        self.players.iter_mut().for_each(|player| {
            player.add_money(money, setting);
        });
    }

    pub fn get_money(&self, name: &str) -> Result<u32, String> {
        Ok(self.get_player(name)?.get_money())
    }

    pub fn get_health(&self, name: &str) -> Result<u32, String> {
        Ok(self.get_player(name)?.get_health())
    }

    pub fn fill_gun(&mut self, guns: Box<Guns>) {
        self.guns = guns
    }

    pub fn get_guns(&self) -> &Guns {
        &self.guns
    }

    pub fn buy_gun(&mut self, player_name: &str, gun_name: &str) -> Result<(), String> {
        match self
            .players
            .iter_mut()
            .find(|player| player.get_name() == player_name)
        {
            Some(player) => {
                if !player.is_alive() {
                    return Err("did can not buy".to_string());
                }
                let gun = self.guns.get_gun(gun_name)?;
                player.buy_gun(gun)
            }
            None => Err("invalid username".to_string()),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn score_board(&self) -> String {
        let players = self
            .players
            .iter()
            .sorted()
            .enumerate()
            .map(|element| format!("{} {}", element.0 + 1, element.1))
            .join("\n");

        format!("{}-Players:\n{}", self.name, players)
    }
}
