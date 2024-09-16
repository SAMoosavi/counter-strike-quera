use crate::gun::{Gun,TypeOfGun};
use std::rc::Rc;


pub struct Player {
    name: String,
    health: i32,
    money: i32,
    kills: i32,
    killed: i32,
    guns: [Option<Rc<Gun>>; 3],
}


impl Player {
    pub fn new(knife: Rc<Gun>,name: String, health: i32, money: i32, kills: i32, killed: i32) -> Result<Player,()> {
        if knife.get_type_of() != TypeOfGun::Knife {
            return Err(());
        }
        let guns: [Option<Rc<Gun>>; 3] = [Some(knife), None, None];
        Ok(Player {
            name,
            health,
            money,
            kills,
            killed,
            guns,
        })
    }

    pub fn shut(&mut self, health: i32) -> Result<i32, String> {
        if self.health <= 0 {
            return Err(format!("{} did!", self.name));
        }
        self.health -= health;
        if self.health <= 0 {
            self.killed += 1;
            self.health = 0;
            self.guns.iter_mut().for_each(|e| *e = None);
        }
        Ok(self.health)
    }

    pub fn buy_gun(&mut self, gun: Rc<Gun>) -> Result<(), String> {
        if self.money < gun.get_money() {
            return Err(format!(
                "{}'s money is {} but need {}",
                self.name,
                self.money,
                gun.get_money()
            ));
        }

        let gun_type = gun.get_type_of() as usize;
        if self.guns[gun_type].is_none() {
            return Err(format!("the {} gun type is  exist.", gun.get_type_of()));
        }

        self.money -= gun.get_money();
        self.guns[gun_type] = Some(gun);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::gun::Gun;
    use std::rc::Rc;

    use super::Player;

    #[test]
    pub fn new_player_when_get_a_gun_that_type_of_it_is_not_knife_should_be_return_error() {
        let knife :Rc<Gun> = Rc::new(Gun::new("not knife".to_string(), 100, 10, 20, crate::gun::TypeOfGun::Pistol));
        assert!(Player::new(knife, "p1".to_string(), 0, 100, 0, 0).is_err());   
    }

    #[test]
    pub fn new_player_when_get_a_gun_that_type_of_it_is_knife_should_be_return_ok() {
        let knife :Rc<Gun> = Rc::new(Gun::new("knife".to_string(), 100, 10, 20, crate::gun::TypeOfGun::Knife));
        assert!(Player::new(knife, "p1".to_string(), 0, 100, 0, 0).is_ok());   
    }

    #[test]
    pub fn shut_did_player() {
        let knife :Rc<Gun> = Rc::new(Gun::new("knife".to_string(), 100, 10, 20, crate::gun::TypeOfGun::Knife));
        let mut player: Player = Player::new(knife, "p1".to_string(), 0, 100, 0, 0).unwrap();
        let result = player.shut(10);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "p1 did!".to_string());
    }

    #[test]
    fn shut_live_player_and_live_after_suth() {
        let knife :Rc<Gun> = Rc::new(Gun::new("knife".to_string(), 100, 10, 20, crate::gun::TypeOfGun::Knife));
        let mut player: Player = Player::new(knife, "p1".to_string(), 100, 100, 0, 0).unwrap();
        let result = player.shut(10);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 90);
        assert_eq!(player.health, 90);
        assert_eq!(player.killed, 0);
    }

    #[test]
    fn shut_live_player_and_did_after_suth() {
        let knife :Rc<Gun> = Rc::new(Gun::new("knife".to_string(), 100, 10, 20, crate::gun::TypeOfGun::Knife));
        let mut player: Player = Player::new(knife, "p1".to_string(), 100, 100, 0, 0).unwrap();
        let result = player.shut(10);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
        assert_eq!(player.health, 0);
        assert_eq!(player.killed, 1);
    }
}
