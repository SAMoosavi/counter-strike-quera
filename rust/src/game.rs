use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

use crate::{game_time::GameTime, gun::TypeOfGun, player::Player, team::Team};

#[derive(Eq, PartialEq, Hash)]
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

    fn find_player(&self, name: &str) -> Option<Rc<RefCell<Player>>> {
        self.teams
            .iter()
            .find(|team| team.1.get_player(name).is_some())
            .map(|team| team.1.get_player(name).unwrap().clone())
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

    pub fn get_player(&self, name: &str) -> Result<Rc<RefCell<Player>>, String> {
        match self.find_player(name) {
            Some(player) => Ok(player.clone()),
            None => Err(format!("player not found: {}", name)),
        }
    }

    pub fn tap(
        &self,
        attacker: &str,
        attacked: &str,
        type_of_gun: &TypeOfGun,
    ) -> Result<(), String> {
        let mut attacker = self.get_player(attacker)?;
        let mut attacked = self.get_player(attacked)?;

        if !attacker.borrow().is_alive() {
            return Err("attacker is dead".to_string());
        }
        if !attacked.borrow().is_alive() {
            return Err("attacked is dead".to_string());
        }

        let mut attacker = attacker.as_ref().borrow_mut();

        let gun = attacker.get_gun_with_type(type_of_gun);
        if gun.is_none() {
            return Err(format!(
                "{} has no {} gun.",
                attacker.get_name(),
                type_of_gun
            ));
        }
        let gun: &Rc<crate::gun::Gun> = gun.unwrap();
        let mut borrow_mut = attacked.as_ref().borrow_mut();
        let health = borrow_mut.shut(gun.get_damage())?;
        if health <= 0 {
            
            // borrow_mut.
            attacker.add_kill(type_of_gun)?;

        }

        Ok(())
    }
}
