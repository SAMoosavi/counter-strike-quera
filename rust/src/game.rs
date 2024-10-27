use clap::ValueEnum;
use std::{collections::HashMap, fmt, rc::Rc};

use crate::{
    game_time::GameTime,
    gun::{Gun, Guns, TypeOfGun},
    setting::Setting,
    team::Team,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Hash)]
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
    setting: Setting,
}

impl Default for Game {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let knife = Rc::new(Gun::new("knife".to_string(), 0, 43, 500, TypeOfGun::Knife));
        let setting = Setting {
            max_money_of_player: 10000,
            default_money_of_player: 1000,
            max_number_of_team_players: 10,
            won_team_money: 2700,
            lose_team_money: 2400,
            max_time_buy: Some(GameTime::new(0, 45, 0)),
            did_time_of_player: Some(GameTime::new(0, 3, 0)),
            default_gun: Some(knife.clone()),
            friendly_fire: false,
        };

        let mut terrorist_guns = Box::new(Guns::new());
        terrorist_guns.add_gun(knife.clone())?;
        terrorist_guns.create_gun("AK".to_string(), 2700, 31, 100, TypeOfGun::Heavy)?;
        terrorist_guns.create_gun("AWP".to_string(), 4300, 110, 50, TypeOfGun::Heavy)?;
        terrorist_guns.create_gun("Revolver".to_string(), 600, 51, 150, TypeOfGun::Pistol)?;
        terrorist_guns.create_gun("Glock-18".to_string(), 300, 11, 200, TypeOfGun::Pistol)?;

        let mut terrorist = Team::new("Terrorist".to_string());
        terrorist.fill_gun(terrorist_guns);

        let mut counter_terrorist_guns = Box::new(Guns::new());
        counter_terrorist_guns.add_gun(knife.clone())?;
        counter_terrorist_guns.create_gun("M4A1".to_string(), 2700, 29, 100, TypeOfGun::Heavy)?;
        counter_terrorist_guns.create_gun("AWP".to_string(), 4300, 110, 50, TypeOfGun::Heavy)?;
        counter_terrorist_guns.create_gun(
            "Desert-Eagle".to_string(),
            600,
            53,
            175,
            TypeOfGun::Pistol,
        )?;
        counter_terrorist_guns.create_gun("UPS-S".to_string(), 300, 13, 225, TypeOfGun::Pistol)?;

        let mut counter_terrorist = Team::new("Counter-Terrorist".to_string());
        counter_terrorist.fill_gun(counter_terrorist_guns);

        Ok(Self {
            teams: HashMap::from([
                (TeamId::Terrorist, terrorist),
                (TeamId::CounterTerrorist, counter_terrorist),
            ]),
            setting,
        })
    }

    fn find_player(&self, name: &str) -> Option<&TeamId> {
        self.teams
            .iter()
            .find(|team| team.1.has_player(name))
            .map(|team| team.0)
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
        team.add_player(name, time, &self.setting)?;
        Ok(format!("this user added to {}", team_id))
    }

    pub fn get_team_id(&self, name: &str) -> Result<TeamId, String> {
        match self.find_player(name) {
            Some(team_id) => Ok(*team_id),
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
        let attacker_team_id = self.get_team_id(attacker)?;
        let attacked_team_id = self.get_team_id(attacked)?;

        let attacker_team = &self.teams[&attacker_team_id];
        let attacked_team = &self.teams[&attacked_team_id];

        if !attacker_team.is_player_alive(attacker)? {
            return Err("attacker is dead".to_string());
        }

        if !attacked_team.is_player_alive(attacked)? {
            return Err("attacked is dead".to_string());
        }

        let gun = attacker_team.get_players_gun(attacker, type_of_gun)?;

        if attacked_team_id == attacker_team_id && !self.setting.friendly_fire {
            return Err("friendly fire".to_string());
        }

        let attacked_team = self.teams.get_mut(&attacked_team_id).unwrap();

        let health = attacked_team.shut(attacked, gun.get_damage())?;

        let attacker_team = self.teams.get_mut(&attacker_team_id).unwrap();

        if health == 0 {
            attacker_team.add_kill_of_player(attacker, type_of_gun)?;
        }

        Ok("nice shot".to_string())
    }

    pub fn buy(&mut self, player: &str, gun: &str, time: &GameTime) -> Result<String, String> {
        match &self.setting.max_time_buy {
            None => Err("the max_time_buy not initialized!".to_string()),
            Some(max_time) => {
                if time > max_time {
                    return Err(format!("you are out of Time: {:?}", max_time));
                }
                let team = self.teams.get_mut(&self.get_team_id(player)?).unwrap();
                team.buy_gun(player, gun)?;

                Ok("I hope you can use it".to_string())
            }
        }
    }

    pub fn end_of_round(&mut self) -> &str {
        let msg;

        if !self.teams[&TeamId::CounterTerrorist].does_live_player_exist()
            && self.teams[&TeamId::Terrorist].does_live_player_exist()
        {
            self.teams
                .get_mut(&TeamId::Terrorist)
                .unwrap()
                .won(&self.setting);
            self.teams
                .get_mut(&TeamId::CounterTerrorist)
                .unwrap()
                .lose(&self.setting);
            msg = "Terrorist won";
        } else {
            msg = "Counter-Terrorist won";
            self.teams
                .get_mut(&TeamId::Terrorist)
                .unwrap()
                .lose(&self.setting);
            self.teams
                .get_mut(&TeamId::CounterTerrorist)
                .unwrap()
                .won(&self.setting);
        }

        self.teams.iter_mut().for_each(|team| team.1.reset());

        msg
    }

    pub fn get_money_of_player(&self, name: &str, _time: &GameTime) -> Result<u32, String> {
        let team_id = self.get_team_id(name)?;
        let team = self.teams.get(&team_id).unwrap();
        team.get_money(name)
    }

    pub fn get_health_of_player(&self, name: &str, _time: &GameTime) -> Result<u32, String> {
        let team_id = self.get_team_id(name)?;
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

        let ans = [counter_terrorism, terrorism];
        ans.join("\n")
    }
}
