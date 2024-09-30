use std::{collections::HashMap, fmt};

use crate::{game_time::GameTime, gun::TypeOfGun, setting::Setting, team::Team};

#[derive(Eq, PartialEq, Hash, Clone)]
enum TeamId {
    Terrorist,
    CounterTerrorist,
}

impl fmt::Display for TeamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match &self {
            TeamId::Terrorist => "terrorist",
            TeamId::CounterTerrorist => "counter_terrorist",
        };
        write!(f, "{}", name)
    }
}

struct Game {
    teams: HashMap<TeamId, Team>,
    round: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            teams: HashMap::from([
                (TeamId::Terrorist, Team::new("Terrorist".to_string())),
                (
                    TeamId::CounterTerrorist,
                    Team::new("Counter-Terrorist".to_string()),
                ),
            ]),
            round: 0,
        }
    }

    fn find_player(&self, name: &str) -> Option<&TeamId> {
        self.teams
            .iter()
            .find(|team| team.1.has_player(name))
            .map(|team| team.0)
            .clone()
    }

    fn has_player(&self, name: &str) -> bool {
        self.find_player(name).is_some()
    }

    pub fn add_player(
        &mut self,
        team_id: TeamId,
        name: &str,
        time: &GameTime,
    ) -> Result<(), String> {
        if self.has_player(name) {
            return Err(format!("player exist with same name: {}", name));
        }
        let team = self.teams.get_mut(&team_id).ok_or("team not found")?;
        team.add_player(name, time)
    }

    pub fn get_player(&self, name: &str) -> Result<TeamId, String> {
        match self.find_player(name) {
            Some(team_id) => Ok((*team_id).clone()),
            None => Err(format!("player not found: {}", name)),
        }
    }

    pub fn tap(
        &self,
        attacker: &str,
        attacked: &str,
        type_of_gun: &TypeOfGun,
    ) -> Result<(), String> {
        let attacker_team = self.teams.get_mut(&self.get_player(attacker)?).unwrap();
        let attacked_team = self.teams.get_mut(&self.get_player(attacked)?).unwrap();

        if !attacker_team.is_player_alive(attacker)? {
            return Err("attacker is dead".to_string());
        }
        if !attacked_team.is_player_alive(attacked)? {
            return Err("attacked is dead".to_string());
        }

        let gun = attacker_team.get_players_gun(attacker, type_of_gun)?;

        if attacked_team.get_name() == attacker_team.get_name() && !Setting::get_friendly_fire() {
            return Err(format!(
                "attacker and attacked in same team: {}",
                attacked_team.get_name()
            ));
        }

        let health = attacked_team.shut(attacked, gun.get_damage())?;

        if health == 0 {
            attacker_team.add_kill_of_player(attacker, type_of_gun)?;
        }

        Ok(())
    }

    pub fn buy(&mut self, player: &str, gun: &str, _time: &GameTime) -> Result<(), String> {
        let team = self.teams.get_mut(&self.get_player(player)?).unwrap();
        team.buy_gun(player, gun)
    }
}
