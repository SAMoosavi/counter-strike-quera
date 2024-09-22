use crate::game_time::GameTime;
use crate::gun::Guns;
use crate::player::Player;
use crate::setting::Setting;
use std::sync::Arc;

pub struct Team {
    players: Vec<Arc<Player>>,
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
        Ok(self.players.push(Arc::new(player)))
    }

    pub fn get_player(&self, name: &str) -> Option<Arc<Player>> {
        match self.players.iter().find(|player| player.get_name() == name) {
            Some(player) => Some(player.clone()),
            None => None,
        }
    }

    pub fn reset(&mut self) {
        for player in &mut self.players {
            player.reset();
        }
    }

    pub fn does_live_palyer_exist(&self) -> bool {
        self.players.iter().any(|player| player.get_health() > 0)
    }

    pub fn number_of_live_player(&self) -> usize {
        self.players
            .iter()
            .filter(|player| player.get_health() > 0)
            .count()
    }

    pub fn won(&mut self) {
        let won_team_money = Setting::get_won_team_money();
        self.players.iter_mut().for_each(|player| {
            player.add_money(won_team_money);
        });
    }

    pub fn lose(&mut self) {
        let lose_team_money = Setting::get_lose_team_money();
        self.players.iter_mut().for_each(|player| {
            player.add_money(lose_team_money);
        });
    }

    pub fn fill_gun(&mut self, guns: Box<Guns>) {
        self.guns = guns
    }

    pub fn get_players(&self) -> &Vec<Arc<Player>> {
        &self.players
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
                player.buy_gun(gun)
            }
            None => Err(format!("player with name {} does not find!", gun_name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::Team;
    use crate::game_time::GameTime;
    use crate::gun::{Gun, Guns, TypeOfGun};
    use crate::player::Player;
    use crate::setting::Setting;

    fn fill_setting_for_create_player() {
        let gun = Gun::new("knife".to_string(), 100, 10, 20, TypeOfGun::Knife);
        Setting::set_default_gun(Arc::new(gun)).unwrap();
        Setting::set_default_money_of_player(1000).unwrap();
    }

    #[test]
    pub fn test_default() {
        let team = Team::default();
        assert_eq!(team.guns, Box::new(Guns::new()));
        assert_eq!(team.players, vec!());
    }

    #[test]
    pub fn test_add_player_should_be_return_error_when_does_not_set_max_number_of_players_in_setting(
    ) {
        Setting::reset();
        fill_setting_for_create_player();
        let mut team = Team::default();
        let time = GameTime::new(0, 0, 0, 1);

        let result = team.add_player("Player", &time);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "the maximum number of players doesn't set!"
        );
        Setting::reset();
    }

    #[test]
    pub fn test_add_player_should_be_return_error_when_team_is_full() {
        Setting::reset();
        fill_setting_for_create_player();
        let mut team = Team::default();
        let time = GameTime::new(0, 0, 0, 1);
        Setting::set_max_money_of_player(1).unwrap();
        team.add_player("Player 1", &time).unwrap();

        let result = team.add_player("Player 2", &time);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "the team is full!");
        Setting::reset();
    }

    #[test]
    pub fn test_add_player_should_be_return_error_when_player_exists_with_same_name() {
        Setting::reset();
        fill_setting_for_create_player();
        let mut team = Team::default();
        let time = GameTime::new(0, 0, 0, 1);
        let name = "Player";
        Setting::set_max_money_of_player(2).unwrap();
        team.add_player(name, &time).unwrap();

        let result = team.add_player(name, &time);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!("player exist with same name: {}", name)
        );
        Setting::reset();
    }

    #[test]
    pub fn test_add_player_added_to_least_of_players() {
        Setting::reset();
        fill_setting_for_create_player();
        let mut team = Team::default();
        Setting::set_max_money_of_player(2).unwrap();
        let name = "Player";
        let time = GameTime::new(0, 0, 0, 1);

        let result = team.add_player(name, &time);

        assert!(result.is_ok());
        assert_eq!(team.players.len(), 1);
        assert_eq!(
            *team.players[0],
            Player::new(name.to_string(), time).unwrap()
        );
        Setting::reset();
    }
}
