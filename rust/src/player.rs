use crate::game_time::GameTime;
use crate::gun::{Gun, TypeOfGun};
use crate::setting::Setting;
use std::{cmp::Ordering, collections::HashMap, rc::Rc};

#[cfg(test)]
mod test;

#[derive(Debug, Eq)]
pub struct Player {
    name: String,
    health: u32,
    money: u32,
    kills: u32,
    death: u32,
    guns: HashMap<TypeOfGun, Rc<Gun>>,
    start_time: GameTime,
}

impl Player {
    pub fn new(name: String, time: GameTime, setting: &Setting) -> Result<Self, String> {
        let default_gun = setting.default_gun.clone();
        if default_gun.is_none() {
            return Err("the default gun doesn't set!".to_string());
        }
        let money = setting.default_money_of_player;
        if money <= 0 {
            return Err("the default money doesn't set!".to_string());
        }
        let mut health = 100;
        match &setting.did_time_of_player {
            Some(did_time) => {
                if did_time < &time {
                    health = 0;
                }
            }
            None => {
                return Err("the default did_time_of_player doesn't set!".to_string());
            }
        }

        Ok(Self {
            name,
            health,
            money,
            kills: 0,
            death: 0,
            guns: HashMap::from([(TypeOfGun::Knife, default_gun.unwrap())]),
            start_time: time,
        })
    }

    pub fn shut(&mut self, health: u32) -> Result<u32, String> {
        if self.health <= 0 {
            return Err(format!("{} did!", self.name));
        }

        if self.health <= health {
            self.death += 1;
            self.health = 0;
            let knife = self.guns.get(&TypeOfGun::Knife).unwrap().clone();
            self.guns.clear();
            self.guns.insert(TypeOfGun::Knife, knife);
        } else {
            self.health -= health;
        }
        Ok(self.health)
    }

    pub fn buy_gun(&mut self, gun: Rc<Gun>) -> Result<(), String> {
        if self.money < gun.get_price() {
            return Err("no enough money".to_string());
        }

        let gun_type = gun.get_type_of();
        if self.guns.get(gun_type).is_some() {
            return Err(format!("you have a {}", gun.get_type_of()));
        }

        self.money -= gun.get_price();
        self.guns.insert(*gun_type, gun);
        Ok(())
    }

    pub fn reset(&mut self) {
        self.health = 100;
    }

    pub fn add_kill(&mut self, gun_type: &TypeOfGun) -> Result<(), String> {
        if self.health == 0 {
            return Err(format!("the {} is did!", self.name));
        }
        match self.guns.get(gun_type) {
            Some(x) => {
                self.money += x.get_gift();
                self.kills += 1;
                Ok(())
            }
            None => Err(format!("the {} does not have {} gun!", self.name, gun_type)),
        }
    }

    pub fn get_kills(&self) -> u32 {
        self.kills
    }
    pub fn get_death(&self) -> u32 {
        self.death
    }
    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn get_money(&self) -> u32 {
        self.money
    }
    pub fn get_gun_with_type(&self, gun_type: &TypeOfGun) -> Option<&Rc<Gun>> {
        self.guns.get(gun_type)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn add_money(&mut self, money: u32, setting: &Setting) {
        if self.money + money > setting.max_money_of_player {
            self.money = setting.max_money_of_player;
        } else {
            self.money += money;
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.name, self.kills, self.death)
    }
}

impl PartialEq<Self> for Player {
    fn eq(&self, other: &Self) -> bool {
        self.kills == other.kills
            && self.death == other.death
            && self.start_time == other.start_time
    }
}

impl PartialOrd<Self> for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kills < other.kills {
            return Ordering::Less;
        }
        if self.kills == other.kills {
            if self.death > other.death {
                return Ordering::Less;
            }
            if self.death == other.death {
                return self.start_time.cmp(&other.start_time).reverse();
            }
        }
        Ordering::Greater
    }
}
