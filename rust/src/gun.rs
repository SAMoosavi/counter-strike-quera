use std::sync::Arc;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum TypeOfGun {
    Heavy,
    Pistol,
    Knife,
}

impl fmt::Display for TypeOfGun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match &self {
            TypeOfGun::Heavy => "Heavy",
            TypeOfGun::Pistol => "Pistol",
            TypeOfGun::Knife => "Knife",
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests_of_type_of_gun {
    use super::TypeOfGun;

    #[test]
    pub fn test_display_heavy() {
        let gun = TypeOfGun::Heavy;
        assert_eq!(gun.to_string(), "Heavy");
    }

    #[test]
    pub fn test_display_pistol() {
        let gun = TypeOfGun::Pistol;
        assert_eq!(gun.to_string(), "Pistol");
    }

    #[test]
    pub fn test_display_knife() {
        let gun = TypeOfGun::Knife;
        assert_eq!(gun.to_string(), "Knife");
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Gun {
    name: String,
    price: u32,
    damage: u32,
    gift: u32,
    type_of: TypeOfGun,
}

impl Gun {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[allow(dead_code)]
    pub fn get_price(&self) -> u32 {
        self.price
    }

    #[allow(dead_code)]
    pub fn get_damage(&self) -> u32 {
        self.damage
    }

    #[allow(dead_code)]
    pub fn get_gift(&self) -> u32 {
        self.gift
    }

    pub fn get_type_of(&self) -> TypeOfGun {
        self.type_of.clone()
    }

    pub fn new(name: String, price: u32, damage: u32, gift: u32, type_of: TypeOfGun) -> Gun {
        Gun {
            name,
            price,
            damage,
            gift,
            type_of,
        }
    }
}

impl fmt::Display for Gun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Gun {{ name: {}, price: {}, damage: {}, gift: {}, type_of: {} }}",
            self.name, self.price, self.damage, self.gift, self.type_of)
    }
}
impl PartialEq for Gun {
    fn eq(&self, other: &Self) -> bool {
        self.damage == other.damage
            && self.gift == other.gift
            && self.name == other.name
            && self.price == other.price
            && self.type_of == other.type_of
    }
}

#[cfg(test)]
mod tests_gun {
    use super::{Gun, TypeOfGun};

    #[test]
    pub fn test_getters() {
        let name = "gun";
        let price = 100;
        let damage = 20;
        let gift = 30;
        let type_of_gun = TypeOfGun::Pistol;
        let gun = Gun::new(name.to_string(), price, damage, gift, type_of_gun);
        assert_eq!(name, gun.get_name());
        assert_eq!(price, gun.get_price());
        assert_eq!(damage, gun.get_damage());
        assert_eq!(gift, gun.get_gift());
        assert_eq!(type_of_gun, gun.get_type_of());
    }

    #[test]
    pub fn check_eq_operator() {
        let name = "gun";
        let price = 100;
        let damage = 20;
        let gift = 30;
        let type_of_gun = TypeOfGun::Pistol;
        let first_gun = Gun::new(name.to_string(), price, damage, gift, type_of_gun);
        let second_gun = Gun::new(name.to_string(), price, damage, gift, type_of_gun);

        assert_eq!(first_gun, second_gun);
    }

    #[test]
    pub fn check_ne_operator() {
        let first_name = "first gun";
        let second_name = "second gun";
        let price = 100;
        let damage = 20;
        let gift = 30;
        let type_of_gun = TypeOfGun::Pistol;
        let first_gun = Gun::new(first_name.to_string(), price, damage, gift, type_of_gun);
        let second_gun = Gun::new(second_name.to_string(), price, damage, gift, type_of_gun);

        assert_ne!(first_gun, second_gun);
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub struct Guns {
    list: Vec<Arc<Gun>>,
}

impl Guns {
    #[allow(dead_code)]
    pub fn new() -> Guns {
        Guns { list: vec![] }
    }

    #[allow(dead_code)]
    pub fn add_gun(
        &mut self,
        name: String,
        price: u32,
        damage: u32,
        gift: u32,
        type_of: TypeOfGun,
    ) -> Result<(), &str> {
        if self.list.iter().any(|gun| name == gun.get_name()) {
            return Err("the gun is exist!");
        } else if type_of == TypeOfGun::Knife
            && self
            .list
            .iter()
            .any(|gun| gun.get_type_of() == TypeOfGun::Knife)
        {
            return Err("The knife exist");
        }

        self.list
            .push(Arc::new(Gun::new(name, price, damage, gift, type_of)));
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_knife(&self) -> Result<Arc<Gun>, ()> {
        for gun in &self.list {
            if gun.get_type_of() == TypeOfGun::Knife {
                return Ok(gun.clone());
            }
        }
        Err(())
    }

    #[allow(dead_code)]
    pub fn get_gun(&self, name: &str) -> Result<Arc<Gun>, String> {
        match self.list.iter().position(|gun| gun.get_name() == name) {
            Some(index) => Ok(self.list[index].clone()),
            None => Err(format!("the gun with name {} does not exist!", name)),
        }
    }

    #[allow(dead_code)]
    pub fn get_guns_with_type(&self, type_of_gun: TypeOfGun) -> Result<Vec<Arc<Gun>>, ()> {
        if type_of_gun == TypeOfGun::Knife {
            return Err(());
        }
        Ok(self
            .list
            .iter()
            .filter(|gun| gun.get_type_of() == type_of_gun)
            .cloned()
            .collect())
    }
}

#[cfg(test)]
mod tests_guns {
    use std::sync::Arc;
    use super::{Guns, TypeOfGun};
    use crate::gun::Gun;

    #[test]
    pub fn add_gun_should_return_error_when_gun_name_is_exist() {
        let mut guns = Guns::new();
        let name = "knife";
        assert!(guns
            .add_gun(name.to_string(), 100, 20, 100, TypeOfGun::Knife)
            .is_ok());
        assert!(guns
            .add_gun(name.to_string(), 10, 2, 500, TypeOfGun::Knife)
            .is_err());
    }

    #[test]
    pub fn add_gun_should_add_a_gun_in_list_of_gun() {
        let mut guns = Guns::new();
        let name = "knife";
        let price = 100;
        let damage = 20;
        let gift = 10;
        let type_of = TypeOfGun::Knife;
        assert!(guns
            .add_gun(name.to_string(), price, damage, gift, type_of)
            .is_ok());
        assert_eq!(guns.list.len(), 1);
        assert_eq!(
            guns.list[0],
            Arc::new(Gun::new(name.to_string(), price, damage, gift, type_of))
        );
    }

    #[test]
    pub fn get_knife_when_has_not_knife_shold_be_retur_error() {
        let mut guns = Guns::new();
        guns.add_gun("not a knife".to_string(), 100, 10, 10, TypeOfGun::Heavy)
            .unwrap();
        assert!(guns.get_knife().is_err());
    }

    #[test]
    pub fn get_knife_when_has_knife_shold_be_retur_knife() {
        let mut guns = Guns::new();
        let name = "knife";
        let price = 100;
        let damage = 20;
        let gift = 10;
        let type_of = TypeOfGun::Knife;
        guns.add_gun(name.to_string(), price, damage, gift, type_of)
            .unwrap();
        let result = guns.get_knife();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Arc::new(Gun::new(name.to_string(), price, damage, gift, type_of))
        );
    }

    #[test]
    pub fn get_gun_func_when_get_a_name_of_gun_does_not_exist_should_be_return_error() {
        let mut guns = Guns::new();
        let name = "test gun";
        let fake_name = "fake gun";
        guns.add_gun(name.to_string(), 100, 10, 10, TypeOfGun::Heavy)
            .unwrap();
        assert!(guns.get_gun(&fake_name).is_err());
    }

    #[test]
    pub fn get_gun_func_when_get_a_name_of_gun_exists_should_be_return_gun() {
        let mut guns = Guns::new();
        let name = "knife";
        let price = 100;
        let damage = 20;
        let gift = 10;
        let type_of = TypeOfGun::Knife;
        guns.add_gun(name.to_string(), price, damage, gift, type_of)
            .unwrap();
        let result = guns.get_gun(&name);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Arc::new(Gun::new(name.to_string(), price, damage, gift, type_of))
        );
    }

    #[test]
    pub fn get_guns_func_when_get_knife_type_should_be_error() {
        let guns = Guns::new();
        assert!(guns.get_guns_with_type(TypeOfGun::Knife).is_err());
    }

    #[test]
    pub fn get_guns_type_func_when_get_a_type_of_gun_should_be_return_list_of_gun_with_this_type() {
        let mut guns = Guns::new();
        let type_of = TypeOfGun::Heavy;
        let other_type_of = TypeOfGun::Pistol;
        guns.add_gun("gun 1".to_string(), 100, 10, 20, type_of)
            .unwrap();
        guns.add_gun("gun 2".to_string(), 100, 10, 20, type_of)
            .unwrap();
        guns.add_gun("gun 3".to_string(), 100, 10, 20, type_of)
            .unwrap();
        guns.add_gun("gun with other type".to_string(), 50, 10, 20, other_type_of)
            .unwrap();

        let result = guns.get_guns_with_type(type_of);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }
}
