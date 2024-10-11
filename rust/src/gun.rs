use clap::ValueEnum;
use std::{fmt, rc::Rc};

#[cfg(test)]
mod tests_of_type_of_gun;

#[cfg(test)]
mod tests_gun;

#[cfg(test)]
mod tests_guns;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Hash)]
pub enum TypeOfGun {
    Heavy,
    Pistol,
    Knife,
}

impl fmt::Display for TypeOfGun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl TypeOfGun {
    pub fn to_string(&self) -> &str {
        match &self {
            TypeOfGun::Heavy => "heavy",
            TypeOfGun::Pistol => "pistol",
            TypeOfGun::Knife => "knife",
        }
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

    pub fn get_price(&self) -> u32 {
        self.price
    }

    pub fn get_damage(&self) -> u32 {
        self.damage
    }

    pub fn get_gift(&self) -> u32 {
        self.gift
    }

    pub fn get_type_of(&self) -> &TypeOfGun {
        &self.type_of
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

impl PartialEq for Gun {
    fn eq(&self, other: &Self) -> bool {
        self.damage == other.damage
            && self.gift == other.gift
            && self.name == other.name
            && self.price == other.price
            && self.type_of == other.type_of
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Guns {
    list: Vec<Rc<Gun>>,
}

impl Default for Guns {
    fn default() -> Self {
        Self::new()
    }
}

impl Guns {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn create_gun(
        &mut self,
        name: String,
        price: u32,
        damage: u32,
        gift: u32,
        type_of: TypeOfGun,
    ) -> Result<(), String> {
        if self.list.iter().any(|gun| name == gun.get_name()) {
            return Err("the gun is exist!".to_string());
        } else if type_of == TypeOfGun::Knife
            && self
                .list
                .iter()
                .any(|gun| gun.get_type_of() == &TypeOfGun::Knife)
        {
            return Err("The knife exist".to_string());
        }

        self.list
            .push(Rc::new(Gun::new(name, price, damage, gift, type_of)));
        Ok(())
    }

    pub fn add_gun(&mut self, gun: Rc<Gun>) -> Result<(), String> {
        if self.list.iter().any(|x| gun.get_name() == x.get_name()) {
            return Err("the gun is exist!".to_string());
        } else if gun.get_type_of() == &TypeOfGun::Knife
            && self
                .list
                .iter()
                .any(|gun| gun.get_type_of() == &TypeOfGun::Knife)
        {
            return Err("The knife exist".to_string());
        }

        self.list.push(gun);
        Ok(())
    }

    pub fn get_knife(&self) -> Result<Rc<Gun>, String> {
        for gun in &self.list {
            if gun.get_type_of() == &TypeOfGun::Knife {
                return Ok(gun.clone());
            }
        }
        Err("knife doesn't exist".to_string())
    }

    pub fn get_gun(&self, name: &str) -> Result<Rc<Gun>, String> {
        self.list
            .iter()
            .find(|gun| gun.get_name() == name)
            .cloned()
            .ok_or("invalid category gun".to_string())
    }

    pub fn get_guns_with_type(&self, type_of_gun: TypeOfGun) -> Result<Vec<Rc<Gun>>, String> {
        if type_of_gun == TypeOfGun::Knife {
            return Err("the knife cannot be received".to_string());
        }
        Ok(self
            .list
            .iter()
            .filter(|gun| gun.get_type_of() == &type_of_gun)
            .cloned()
            .collect())
    }
}
