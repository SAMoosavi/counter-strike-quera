use std::{collections::HashMap, fmt};

use crate::{game_time::GameTime, gun::TypeOfGun, setting::Setting, team::Team};

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum TeamId {
    Terrorist,
    CounterTerrorist,
}

impl TeamId {
    pub fn to_string(&self) -> &str {
        match &self {
            TeamId::Terrorist => "Terrorist",
            TeamId::CounterTerrorist => "Counter-Terrorist",
        }
    }

    pub fn to_enum(name: &str) -> Result<TeamId, String> {
        match &name.to_lowercase().replace("_", "-").replace(" ", "-")[0..] {
            "counter-terrorist" => Ok(TeamId::CounterTerrorist),
            "terrorist" => Ok(TeamId::Terrorist),
            _ => Err(format!("the name of {} isn't correct.", name)),
        }
    }
}

impl fmt::Display for TeamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match &self {
            TeamId::Terrorist => "Terrorist",
            TeamId::CounterTerrorist => "Counter-Terrorist",
        };
        write!(f, "{}", name)
    }
}

pub struct Game {
    teams: HashMap<TeamId, Team>,
    _round: u32,
}

impl Game {
    pub fn new() -> Self {
        Setting::set_max_money_of_player(10000).unwrap();
        Setting::set_default_money_of_player(1000).unwrap();
        Setting::set_max_number_of_team_players(10).unwrap();
        Setting::set_won_team_money(2700).unwrap();
        Setting::set_lose_team_money(2400).unwrap();
    
        Self {
            teams: HashMap::from([
                (TeamId::Terrorist, Team::new("Terrorist".to_string())),
                (
                    TeamId::CounterTerrorist,
                    Team::new("Counter-Terrorist".to_string()),
                ),
            ]),
            _round: 0,
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
        &mut self,
        attacker: &str,
        attacked: &str,
        type_of_gun: &TypeOfGun,
    ) -> Result<(), String> {
        let attacker_team = self.get_player(attacker)?;
        let attacked_team = self.get_player(attacked)?;

        if !self
            .teams
            .get_mut(&attacker_team)
            .unwrap()
            .is_player_alive(attacker)?
        {
            return Err("attacker is dead".to_string());
        }

        if !self
            .teams
            .get_mut(&attacked_team)
            .unwrap()
            .is_player_alive(attacked)?
        {
            return Err("attacked is dead".to_string());
        }

        let gun = self
            .teams
            .get_mut(&attacker_team)
            .unwrap()
            .get_players_gun(attacker, type_of_gun)?;

        if attacked_team == attacker_team && !Setting::get_friendly_fire() {
            return Err(format!(
                "attacker and attacked in same team: {}",
                self.teams.get_mut(&attacked_team).unwrap().get_name()
            ));
        }

        let health = self
            .teams
            .get_mut(&attacked_team)
            .unwrap()
            .shut(attacked, gun.get_damage())?;

        if health == 0 {
            self.teams
                .get_mut(&attacker_team)
                .unwrap()
                .add_kill_of_player(attacker, type_of_gun)?;
        }

        Ok(())
    }

    pub fn buy(&mut self, player: &str, gun: &str, _time: &GameTime) -> Result<(), String> {
        let team = self.teams.get_mut(&self.get_player(player)?).unwrap();
        team.buy_gun(player, gun)
    }

    pub fn reset(&mut self) -> &str {
        let msg;

        if !self
            .teams
            .get(&TeamId::CounterTerrorist)
            .unwrap()
            .does_live_player_exist()
            && self
                .teams
                .get(&TeamId::Terrorist)
                .unwrap()
                .does_live_player_exist()
        {
            self.teams.get_mut(&TeamId::Terrorist).unwrap().won();
            self.teams
                .get_mut(&TeamId::CounterTerrorist)
                .unwrap()
                .lose();
            msg = "Terrorist won";
        } else {
            msg = "Counter-Terrorist won";
            self.teams.get_mut(&TeamId::Terrorist).unwrap().lose();
            self.teams.get_mut(&TeamId::CounterTerrorist).unwrap().won();
        }

        self.teams.iter_mut().for_each(|team| team.1.reset());

        msg
    }
}
