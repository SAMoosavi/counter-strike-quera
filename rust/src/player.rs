use crate::gun::{Gun, TypeOfGun};
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Player {
    name: String,
    health: i32,
    money: i32,
    kills: i32,
    killed: i32,
    guns: HashMap<TypeOfGun, Rc<Gun>>,
}

impl Player {
    #[allow(dead_code)]
    pub fn new(knife: Rc<Gun>, name: String) -> Result<Player, ()> {
        if knife.get_type_of() != TypeOfGun::Knife {
            return Err(());
        }

        Ok(Player {
            name,
            health: 100,
            money: 0,
            kills: 0,
            killed: 0,
            guns: HashMap::from([(TypeOfGun::Knife, knife)]),
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
    pub fn buy_gun(&mut self, gun: Rc<Gun>) -> Result<(), String> {
        if self.money < gun.get_gift() {
            return Err(format!(
                "{}'s money is {} but need {}",
                self.name,
                self.money,
                gun.get_gift()
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
}

#[cfg(test)]
mod tests {
    use crate::gun::Gun;
    use std::rc::Rc;

    use super::Player;

    fn create_knife() -> Rc<Gun> {
        Rc::new(Gun::new(
            "knife".to_string(),
            100,
            10,
            20,
            crate::gun::TypeOfGun::Knife,
        ))
    }

    fn create_player() -> Player {
        let knife: Rc<Gun> = create_knife();
        Player::new(knife, "p1".to_string()).unwrap()
    }

    #[test]
    pub fn new_player_when_get_a_gun_that_type_of_it_is_not_knife_should_be_return_error() {
        let knife: Rc<Gun> = Rc::new(Gun::new(
            "not knife".to_string(),
            100,
            10,
            20,
            crate::gun::TypeOfGun::Pistol,
        ));
        assert!(Player::new(knife, "p1".to_string()).is_err());
    }

    #[test]
    pub fn new_player_when_get_a_gun_that_type_of_it_is_knife_should_be_return_ok() {
        let knife: Rc<Gun> = create_knife();
        assert!(Player::new(knife, "p1".to_string()).is_ok());
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
}
