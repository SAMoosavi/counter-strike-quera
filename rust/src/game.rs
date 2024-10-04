use std::{collections::HashMap, fmt, rc::Rc};

use crate::{
    game_time::GameTime,
    gun::{Gun, Guns, TypeOfGun},
    setting::Setting,
    team::Team,
};

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
}

impl Game {
    pub fn new() -> Self {
        Setting::set_max_money_of_player(10000).unwrap();
        Setting::set_default_money_of_player(1000).unwrap();
        Setting::set_max_number_of_team_players(10).unwrap();
        Setting::set_won_team_money(2700).unwrap();
        Setting::set_lose_team_money(2400).unwrap();
        Setting::set_max_time_buy(&GameTime::new(0, 45, 0)).unwrap();

        let knife = Rc::new(Gun::new("knife".to_string(), 0, 43, 500, TypeOfGun::Knife));
        Setting::set_default_gun(&knife).unwrap();

        let mut terrorist_guns = Box::new(Guns::new());
        terrorist_guns.add_gun(&knife).unwrap();
        terrorist_guns
            .create_gun("AK".to_string(), 2700, 31, 100, TypeOfGun::Heavy)
            .unwrap();
        terrorist_guns
            .create_gun("AWP".to_string(), 4300, 110, 50, TypeOfGun::Heavy)
            .unwrap();
        terrorist_guns
            .create_gun("Revolver".to_string(), 600, 51, 150, TypeOfGun::Pistol)
            .unwrap();
        terrorist_guns
            .create_gun("Glock-18".to_string(), 300, 11, 200, TypeOfGun::Pistol)
            .unwrap();

        let mut terrorist = Team::new("Terrorist".to_string());
        terrorist.fill_gun(terrorist_guns);

        let mut counter_terrorist_guns = Box::new(Guns::new());
        counter_terrorist_guns.add_gun(&knife).unwrap();
        counter_terrorist_guns
            .create_gun("M4A1".to_string(), 2700, 29, 100, TypeOfGun::Heavy)
            .unwrap();
        counter_terrorist_guns
            .create_gun("AWP".to_string(), 4300, 110, 50, TypeOfGun::Heavy)
            .unwrap();
        counter_terrorist_guns
            .create_gun("Desert-Eagle".to_string(), 600, 53, 175, TypeOfGun::Pistol)
            .unwrap();
        counter_terrorist_guns
            .create_gun("UPS-S".to_string(), 300, 13, 225, TypeOfGun::Pistol)
            .unwrap();

        let mut counter_terrorist = Team::new("Counter-Terrorist".to_string());
        counter_terrorist.fill_gun(counter_terrorist_guns);

        Self {
            teams: HashMap::from([
                (TeamId::Terrorist, terrorist),
                (TeamId::CounterTerrorist, counter_terrorist),
            ]),
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
    ) -> Result<String, String> {
        if self.has_player(name) {
            return Err("you are already in this game".to_string());
        }
        let team = self.teams.get_mut(&team_id).ok_or("team not found")?;
        team.add_player(name, time)?;
        Ok(format!("this user added to {}", team_id))
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
        _time: &GameTime,
    ) -> Result<String, String> {
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

        Ok("nice shot".to_string())
    }

    pub fn buy(&mut self, player: &str, gun: &str, time: &GameTime) -> Result<String, String> {
        match Setting::get_max_time_buy() {
            None => Err("the max_time_buy not initialized!".to_string()),
            Some(max_time) => {
                if time > &max_time {
                    return Err(format!("you are out of Time: {:?}", max_time));
                }
                let team = self.teams.get_mut(&self.get_player(player)?).unwrap();
                team.buy_gun(player, gun)?;

                Ok("I hope you can use it".to_string())
            }
        }
    }

    pub fn end_of_round(&mut self) -> &str {
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

    pub fn get_money_of_player(&self, name: &str, _time: &GameTime) -> Result<u32, String> {
        let team_id = self.get_player(name)?;
        let team = self.teams.get(&team_id).unwrap();
        team.get_money(name)
    }

    pub fn get_health_of_player(&self, name: &str, _time: &GameTime) -> Result<u32, String> {
        let team_id = self.get_player(name)?;
        let team = self.teams.get(&team_id).unwrap();
        team.get_health(name)
    }

    pub fn score_board(&self, _time: &GameTime) -> String {
        let counter_terrorism = self
            .teams
            .get(&TeamId::CounterTerrorist)
            .unwrap()
            .score_board();
        let terrorism = self.teams.get(&TeamId::Terrorist).unwrap().score_board();

        let ans = vec![counter_terrorism, terrorism];
        ans.join("\n")
    }
}
