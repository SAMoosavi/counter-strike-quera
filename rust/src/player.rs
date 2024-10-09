use crate::game_time::GameTime;
use crate::gun::{Gun, TypeOfGun};
use crate::setting::Setting;
use std::{cmp::Ordering, collections::HashMap, fmt, rc::Rc};

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
    pub fn new(name: String, start_time: GameTime, setting: &Setting) -> Result<Self, String> {
        match &setting.default_gun {
            Some(default_gun) => match setting.default_money_of_player {
                0 => Err("the default money doesn't set!".to_string()),
                money => match &setting.did_time_of_player {
                    Some(did_time) => {
                        let health = {
                            if did_time < &start_time {
                                0
                            } else {
                                100
                            }
                        };

                        Ok(Self {
                            name,
                            health,
                            money,
                            kills: 0,
                            death: 0,
                            guns: HashMap::from([(TypeOfGun::Knife, default_gun.clone())]),
                            start_time,
                        })
                    }
                    None => Err("the default did_time_of_player doesn't set!".to_string()),
                },
            },
            None => Err("the default gun doesn't set!".to_string()),
        }
    }

    pub fn shut(&mut self, health: u32) -> Result<u32, String> {
        if self.health == 0 {
            return Err(format!("{} did!", self.name));
        }

        if self.health <= health {
            self.death += 1;
            self.health = 0;
            let knife = self.guns[&TypeOfGun::Knife].clone();
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
        if self.guns.contains_key(gun_type) {
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
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.kills, self.death)
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
