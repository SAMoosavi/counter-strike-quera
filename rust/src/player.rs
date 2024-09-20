use crate::gun::{Gun, TypeOfGun};
use crate::setting::Setting;
use std::collections::HashMap;
use std::sync::Arc;

#[allow(dead_code)]
pub struct Player {
    name: String,
    health: i32,
    money: i32,
    kills: i32,
    killed: i32,
    guns: HashMap<TypeOfGun, Arc<Gun>>,
}

impl Player {
    #[allow(dead_code)]
    pub fn new(name: String) -> Result<Self, ()> {
        let default_gun = Setting::get_default_gun();
        if let None =  default_gun {
            return Err(());
        }

        Ok(Self {
            name,
            health: 100,
            money: 0,
            kills: 0,
            killed: 0,
            guns: HashMap::from([(TypeOfGun::Knife, default_gun.unwrap())]),
        })
    }

    #[allow(dead_code)]
    pub fn shut(&mut self, health: i32) -> Result<i32, String> {
        if self.health <= 0 {
            return Err(format!("{} did!", self.name));
        }

        self.health -= health;
        if self.health <= 0 {
            self.killed += 1;
            self.health = 0;
            let knife = self.guns.get(&TypeOfGun::Knife).unwrap().clone();
            self.guns.clear();
            self.guns.insert(TypeOfGun::Knife, knife);
        }
        Ok(self.health)
    }

    #[allow(dead_code)]
    pub fn buy_gun(&mut self, gun: Arc<Gun>) -> Result<(), String> {
        if self.money < gun.get_price() {
            return Err(format!(
                "{}'s money is {} but need {}",
                self.name,
                self.money,
                gun.get_price()
            ));
        }

        let gun_type = gun.get_type_of();
        if self.guns.get(&gun_type).is_some() {
            return Err(format!("the {} gun type is exist.", gun.get_type_of()));
        }

        self.money -= gun.get_price();
        self.guns.insert(gun_type, gun);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.health = 100;
    }

    #[allow(dead_code)]
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
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::gun::{Gun, TypeOfGun};
    use crate::setting::Setting;
    use super::Player;


    fn create_player() -> Player {
        let gun = Gun::new("knife".to_string(), 100, 10, 20, TypeOfGun::Knife);
        Setting::set_default_gun(Arc::new(gun)).unwrap();
        Player::new("p1".to_string()).unwrap()
    }

    #[test]
    pub fn new_player_when_get_a_gun_that_type_of_it_is_not_knife_should_be_return_error() {
        Setting::reset();
        assert!(Player::new("p1".to_string()).is_err());
    }

    #[test]
    pub fn new_player_when_get_a_gun_that_type_of_it_is_knife_should_be_return_ok() {
        let gun = Gun::new("knife".to_string(), 100, 10, 20, TypeOfGun::Knife);
        Setting::set_default_gun(Arc::new(gun)).unwrap();
        assert!(Player::new("p1".to_string()).is_ok());
    }

    #[test]
    pub fn shut_did_player() {
        let mut player: Player = create_player();
        player.health = 0;
        let result = player.shut(10);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "p1 did!".to_string());
    }

    #[test]
    pub fn player_should_be_live_when_its_health_has_more_then_shut() {
        let mut player: Player = create_player();
        let result = player.shut(10);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 90);
        assert_eq!(player.health, 90);
        assert_eq!(player.killed, 0);
    }

    #[test]
    pub fn player_should_be_dead_when_its_health_has_lese_then_shut() {
        let mut player: Player = create_player();
        player.health = 5;
        let result = player.shut(10);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
        assert_eq!(player.health, 0);
        assert_eq!(player.killed, 1);
    }

    #[test]
    pub fn player_can_not_buy_gun_when_does_not_have_enough_money() {
        let mut player: Player = create_player();
        player.money = 10;

        let gun = Arc::new(Gun::new(
            "new gun".to_string(),
            100,
            10,
            20,
            crate::gun::TypeOfGun::Heavy,
        ));

        let result = player.buy_gun(gun);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "p1's money is 10 but need 100");
    }

    #[test]
    pub fn player_can_not_buy_gun_when_exist_its_type() {
        let mut player: Player = create_player();
        player.money = 1000;

        let heavy_gun_1 = Arc::new(Gun::new(
            "heavy gun 1".to_string(),
            100,
            10,
            20,
            crate::gun::TypeOfGun::Heavy,
        ));

        let heavy_gun_2 = Arc::new(Gun::new(
            "heavy gun 2".to_string(),
            100,
            10,
            20,
            crate::gun::TypeOfGun::Heavy,
        ));

        let result = player.buy_gun(heavy_gun_1);

        assert!(result.is_ok());

        let result = player.buy_gun(heavy_gun_2);
        assert!(result.is_err());

        assert_eq!(result.unwrap_err(), "the Heavy gun type is exist.");
    }

    #[test]
    pub fn player_can_buy_gun_when_does_not_have_its_type_and_enough_money() {
        let mut player: Player = create_player();
        player.money = 1000;

        let gun = Arc::new(Gun::new(
            "heavy gun 1".to_string(),
            100,
            10,
            20,
            crate::gun::TypeOfGun::Heavy,
        ));

        let result = player.buy_gun(gun.clone());

        assert!(result.is_ok());
        assert_eq!(player.money, 900);
        assert_eq!(player.guns.get(&crate::gun::TypeOfGun::Heavy), Some(&gun));
    }

    #[test]
    pub fn players_health_set_100_when_call_reset_function() {
        let mut player = create_player();
        player.health = 30;

        player.reset();
        assert_eq!(player.health, 100);
    }

    #[test]
    pub fn the_add_kill_func_should_be_return_error_when_player_is_did() {
        let mut player = create_player();
        player.health = 0;

        let result = player.add_kill(&crate::gun::TypeOfGun::Knife);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "the p1 is did!");
    }

    #[test]
    pub fn the_add_kill_func_should_be_return_error_when_player_does_not_have_this_type_of_gun() {
        let mut player = create_player();

        let result = player.add_kill(&crate::gun::TypeOfGun::Heavy);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "the p1 does not have Heavy gun!");
    }

    #[test]
    pub fn the_add_kill_func_should_be_add_kill_number_and_money() {
        let mut player = create_player();
        player.money = 1100;
        let gun = Arc::new(Gun::new(
            "heavy gun".to_string(),
            100,
            10,
            20,
            crate::gun::TypeOfGun::Heavy,
        ));

        player.buy_gun(gun.clone()).unwrap();

        let result = player.add_kill(&crate::gun::TypeOfGun::Heavy);

        assert!(result.is_ok());
        assert_eq!(player.kills, 1);
        assert_eq!(player.money, 1020);
    }
}
