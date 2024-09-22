use std::sync::Arc;
use crate::game_time::GameTime;
use crate::gun::Guns;
use crate::player::{self, Player};
use crate::setting::Setting;

pub struct Team {
    players: Vec<Arc<Player>>,
    guns: Guns,
}

impl Team {
    pub fn default() -> Self {
        Self {
            players: vec!(),
            guns: Guns::new(),
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
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::game_time::GameTime;
    use crate::gun::{Guns, Gun, TypeOfGun};
    use crate::player::Player;
    use crate::setting::Setting;
    use super::Team;

    fn fill_setting_for_create_player(){
        let gun = Gun::new("knife".to_string(), 100, 10, 20, TypeOfGun::Knife);
        Setting::set_default_gun(Arc::new(gun)).unwrap();
        Setting::set_default_money_of_player(1000).unwrap();
    }

    #[test]
    pub fn test_default() {
        let team = Team::default();
        assert_eq!(team.guns, Guns::new());
        assert_eq!(team.players, vec!());
    }

    #[test]
    pub fn test_add_player_should_be_return_error_when_does_not_set_max_number_of_players_in_setting() {
        Setting::reset();
        fill_setting_for_create_player();
        let mut team = Team::default();
        let time = GameTime::new(0, 0, 0, 1);

        let result = team.add_player("Player", &time);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "the maximum number of players doesn't set!");
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
        assert_eq!(result.unwrap_err(), format!("player exist with same name: {}", name));
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
        assert_eq!(*team.players[0], Player::new(name.to_string(), time).unwrap());
        Setting::reset();
    }
}